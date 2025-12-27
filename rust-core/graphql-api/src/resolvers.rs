use async_graphql::{Context, Object, FieldResult, InputObject, SimpleObject};
use indexing::store::{SearchStore, GraphStore, SearchQuery, Filter};
use indexing::hydration::ObjectHydrator;
use ontology_engine::Ontology;
use std::sync::Arc;

/// Root query type for GraphQL API
#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Search for objects of a specific type
    async fn search_objects(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        filters: Option<Vec<FilterInput>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> FieldResult<Vec<ObjectResult>> {
        // Get services from context
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let search_store = ctx.data::<Arc<dyn SearchStore>>()?;
        let hydrator = ctx.data::<ObjectHydrator>()?;
        
        // Get object type definition
        let object_type_def = ontology.get_object_type(&object_type)
            .ok_or_else(|| async_graphql::Error::new("Object type not found"))?;
        
        // Build search query
        // TODO: Implement proper filter conversion from GraphQL input
        let query = SearchQuery {
            filters: vec![], // Simplified for now - filters not yet fully implemented
            sort: None,
            limit,
            offset,
        };
        
        // Execute search
        let indexed_objects = search_store.search(&object_type, &query).await
            .map_err(|e| async_graphql::Error::new(format!("Search error: {}", e)))?;
        
        // Hydrate objects
        let hydrated = hydrator.hydrate_batch(&indexed_objects, object_type_def)
            .map_err(|e| async_graphql::Error::new(format!("Hydration error: {}", e)))?;
        
        // Convert to GraphQL results
        Ok(hydrated.into_iter().map(|h| ObjectResult {
            object_type: h.object_type,
            object_id: h.object_id,
            title: h.title,
            properties: serde_json::to_string(&h.properties).unwrap_or_else(|_| "{}".to_string()),
        }).collect())
    }
    
    /// Get a specific object by ID
    async fn get_object(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        object_id: String,
    ) -> FieldResult<Option<ObjectResult>> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let search_store = ctx.data::<Arc<dyn SearchStore>>()?;
        let hydrator = ctx.data::<ObjectHydrator>()?;
        
        let object_type_def = ontology.get_object_type(&object_type)
            .ok_or_else(|| async_graphql::Error::new("Object type not found"))?;
        
        let indexed = search_store.get_object(&object_type, &object_id).await
            .map_err(|e| async_graphql::Error::new(format!("Get error: {}", e)))?;
        
        if let Some(indexed) = indexed {
            let hydrated = hydrator.hydrate_from_indexed(&indexed, object_type_def)
                .map_err(|e| async_graphql::Error::new(format!("Hydration error: {}", e)))?;
            
            Ok(Some(ObjectResult {
                object_type: hydrated.object_type,
                object_id: hydrated.object_id,
                title: hydrated.title,
                properties: serde_json::to_string(&hydrated.properties).unwrap_or_else(|_| "{}".to_string()),
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Get linked objects via a specific link type
    async fn get_linked_objects(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        object_id: String,
        link_type: String,
    ) -> FieldResult<Vec<ObjectResult>> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let graph_store = ctx.data::<Arc<dyn GraphStore>>()?;
        let search_store = ctx.data::<Arc<dyn SearchStore>>()?;
        let hydrator = ctx.data::<ObjectHydrator>()?;
        
        // Validate link type
        let link_type_def = ontology.get_link_type(&link_type)
            .ok_or_else(|| async_graphql::Error::new("Link type not found"))?;
        
        // Determine target object type
        let target_type = if link_type_def.source == object_type {
            &link_type_def.target
        } else if link_type_def.target == object_type {
            &link_type_def.source
        } else {
            return Err(async_graphql::Error::new("Link type does not connect to this object type"));
        };
        
        let target_type_def = ontology.get_object_type(target_type)
            .ok_or_else(|| async_graphql::Error::new("Target object type not found"))?;
        
        // Get linked object IDs from graph store
        let linked_ids = graph_store.get_connected_objects(&object_id, &link_type).await
            .map_err(|e| async_graphql::Error::new(format!("Graph query error: {}", e)))?;
        
        // Fetch and hydrate linked objects
        let mut results = Vec::new();
        for id in linked_ids {
            if let Some(indexed) = search_store.get_object(target_type, &id).await
                .map_err(|e| async_graphql::Error::new(format!("Get error: {}", e)))?
            {
                if let Ok(hydrated) = hydrator.hydrate_from_indexed(&indexed, target_type_def) {
                    results.push(ObjectResult {
                        object_type: hydrated.object_type,
                        object_id: hydrated.object_id,
                        title: hydrated.title,
                        properties: serde_json::to_string(&hydrated.properties).unwrap_or_else(|_| "{}".to_string()),
                    });
                }
            }
        }
        
        Ok(results)
    }
}

/// Input for search filters
#[derive(InputObject)]
struct FilterInput {
    property: String,
    operator: String,
    value: String, // JSON string for now - TODO: implement proper PropertyValue GraphQL type
}

/// GraphQL result type for objects
#[derive(SimpleObject)]
pub struct ObjectResult {
    pub object_type: String,
    pub object_id: String,
    pub title: String,
    pub properties: String, // JSON string for now - TODO: implement proper PropertyMap GraphQL type
}

