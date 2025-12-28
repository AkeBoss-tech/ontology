use async_graphql::{Context, Object, FieldResult, InputObject, SimpleObject};
use indexing::store::{SearchStore, GraphStore, SearchQuery, Filter};
use indexing::hydration::ObjectHydrator;
use ontology_engine::{Ontology, FunctionExecutor, InterfaceValidator};
use versioning::time_query;
use std::sync::Arc;
use std::collections::HashMap;
use serde_json::Value;

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
        
        // Build filters first
        let mut store_filters = Vec::new();
        if let Some(filter_inputs) = filters {
            for filter_input in filter_inputs {
                // Parse operator
                let operator = match filter_input.operator.to_lowercase().as_str() {
                    "equals" | "eq" => indexing::store::FilterOperator::Equals,
                    "notequals" | "ne" => indexing::store::FilterOperator::NotEquals,
                    "greaterthan" | "gt" => indexing::store::FilterOperator::GreaterThan,
                    "lessthan" | "lt" => indexing::store::FilterOperator::LessThan,
                    "greaterthanorequal" | "gte" => indexing::store::FilterOperator::GreaterThanOrEqual,
                    "lessthanorequal" | "lte" => indexing::store::FilterOperator::LessThanOrEqual,
                    "contains" => indexing::store::FilterOperator::Contains,
                    "startswith" => indexing::store::FilterOperator::StartsWith,
                    "endswith" => indexing::store::FilterOperator::EndsWith,
                    "in" => indexing::store::FilterOperator::In,
                    "notin" => indexing::store::FilterOperator::NotIn,
                    "containsgeometry" => indexing::store::FilterOperator::ContainsGeometry,
                    "intersects" => indexing::store::FilterOperator::Intersects,
                    "within" => indexing::store::FilterOperator::Within,
                    "withindistance" => indexing::store::FilterOperator::WithinDistance,
                    _ => return Err(async_graphql::Error::new(format!(
                        "Invalid filter operator: {}",
                        filter_input.operator
                    ))),
                };
                
                // Parse value (simplified - would need proper JSON parsing)
                let value = serde_json::from_str::<serde_json::Value>(&filter_input.value)
                    .map_err(|e| async_graphql::Error::new(format!("Invalid filter value JSON: {}", e)))?;
                
                let property_value = match value {
                    serde_json::Value::String(s) => ontology_engine::PropertyValue::String(s),
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            ontology_engine::PropertyValue::Integer(i)
                        } else if let Some(d) = n.as_f64() {
                            ontology_engine::PropertyValue::Double(d)
                        } else {
                            return Err(async_graphql::Error::new("Invalid number in filter value"));
                        }
                    }
                    serde_json::Value::Bool(b) => ontology_engine::PropertyValue::Boolean(b),
                    _ => return Err(async_graphql::Error::new("Unsupported filter value type")),
                };
                
                store_filters.push(Filter {
                    property: filter_input.property,
                    operator,
                    value: property_value,
                    distance: filter_input.distance,
                });
            }
        }
        
        // Try to get data from in-memory store first
        let data_store = ctx.data::<Arc<tokio::sync::RwLock<HashMap<String, Vec<Value>>>>>();
        
        if let Ok(store) = data_store {
            let store_read = store.read().await;
            eprintln!("DEBUG: Looking for object_type: {}", object_type);
            eprintln!("DEBUG: Available keys in store: {:?}", store_read.keys().collect::<Vec<_>>());
            if let Some(objects) = store_read.get(&object_type) {
                eprintln!("DEBUG: Found {} objects for {}", objects.len(), object_type);
                // Get object type definition for metadata
                let object_type_def = ontology.get_object_type(&object_type)
                    .ok_or_else(|| async_graphql::Error::new("Object type not found in ontology"))?;
                
                // Filter objects based on filters
                let mut filtered: Vec<&Value> = objects.iter().collect();
                
                // Apply filters
                for filter in &store_filters {
                    filtered.retain(|obj| {
                        if let Some(prop_value) = obj.get(&filter.property) {
                            match &filter.operator {
                                indexing::store::FilterOperator::Equals => {
                                    match filter.value {
                                        ontology_engine::PropertyValue::String(ref s) => {
                                            prop_value.as_str().map_or(false, |v| v == s)
                                        }
                                        ontology_engine::PropertyValue::Integer(i) => {
                                            prop_value.as_i64().map_or(false, |v| v == i)
                                        }
                                        ontology_engine::PropertyValue::Double(d) => {
                                            prop_value.as_f64().map_or(false, |v| (v - d).abs() < 0.0001)
                                        }
                                        _ => false,
                                    }
                                }
                                indexing::store::FilterOperator::GreaterThan => {
                                    match filter.value {
                                        ontology_engine::PropertyValue::Integer(i) => {
                                            prop_value.as_i64().map_or(false, |v| v > i)
                                        }
                                        ontology_engine::PropertyValue::Double(d) => {
                                            prop_value.as_f64().map_or(false, |v| v > d)
                                        }
                                        _ => false,
                                    }
                                }
                                indexing::store::FilterOperator::LessThan => {
                                    match filter.value {
                                        ontology_engine::PropertyValue::Integer(i) => {
                                            prop_value.as_i64().map_or(false, |v| v < i)
                                        }
                                        ontology_engine::PropertyValue::Double(d) => {
                                            prop_value.as_f64().map_or(false, |v| v < d)
                                        }
                                        _ => false,
                                    }
                                }
                                _ => true, // For other operators, keep for now
                            }
                        } else {
                            false
                        }
                    });
                }
                
                // Apply pagination
                let start = offset.unwrap_or(0);
                let end = limit.map(|l| start + l).unwrap_or(filtered.len());
                let paginated: Vec<&Value> = filtered.into_iter().skip(start).take(end - start).collect();
                
                // Convert to ObjectResult
                let results: Vec<ObjectResult> = paginated.iter().map(|obj| {
                    let object_id = obj.get(&object_type_def.primary_key)
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();
                    
                    let title = object_type_def.title_key
                        .as_ref()
                        .and_then(|key| obj.get(key))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| object_id.clone());
                    
                    ObjectResult {
                        object_type: object_type.clone(),
                        object_id,
                        title,
                        properties: serde_json::to_string(obj).unwrap_or_else(|_| "{}".to_string()),
                    }
                }).collect();
                
                eprintln!("DEBUG: Returning {} results from in-memory store", results.len());
                return Ok(results);
            } else {
                eprintln!("DEBUG: No objects found for object_type: {}", object_type);
                eprintln!("DEBUG: Available keys: {:?}", store_read.keys().collect::<Vec<_>>());
            }
        } else {
            eprintln!("DEBUG: Failed to get data_store from context. Error: {:?}", data_store);
        }
        
        // Fallback to search store - get object type definition
        let object_type_def = ontology.get_object_type(&object_type)
            .ok_or_else(|| async_graphql::Error::new("Object type not found"))?;
        
        let query = SearchQuery {
            filters: store_filters,
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
    
    /// Spatial query - search objects by geospatial criteria
    async fn spatial_query(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        property: String,
        operator: String,
        geometry: String, // GeoJSON string
        distance: Option<f64>, // For WithinDistance operator
    ) -> FieldResult<Vec<ObjectResult>> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let search_store = ctx.data::<Arc<dyn SearchStore>>()?;
        let hydrator = ctx.data::<ObjectHydrator>()?;
        
        let object_type_def = ontology.get_object_type(&object_type)
            .ok_or_else(|| async_graphql::Error::new("Object type not found"))?;
        
        // Validate that the property exists and is GeoJSON type
        let prop = object_type_def.get_property(&property)
            .ok_or_else(|| async_graphql::Error::new(format!("Property '{}' not found", property)))?;
        
        if prop.property_type != ontology_engine::PropertyType::GeoJSON {
            return Err(async_graphql::Error::new(format!(
                "Property '{}' is not a GeoJSON type",
                property
            )));
        }
        
        // Parse operator
        let filter_operator = match operator.to_lowercase().as_str() {
            "contains" => indexing::store::FilterOperator::ContainsGeometry,
            "intersects" => indexing::store::FilterOperator::Intersects,
            "within" => indexing::store::FilterOperator::Within,
            "within_distance" => indexing::store::FilterOperator::WithinDistance,
            _ => return Err(async_graphql::Error::new(format!(
                "Invalid spatial operator: {}. Valid operators: contains, intersects, within, within_distance",
                operator
            ))),
        };
        
        // Validate GeoJSON
        let geometry_value = ontology_engine::PropertyValue::GeoJSON(geometry.clone());
        if let Err(e) = prop.validate_value(&geometry_value) {
            return Err(async_graphql::Error::new(format!(
                "Invalid GeoJSON geometry: {}",
                e
            )));
        }
        
        // Build filter
        let filter = Filter {
            property,
            operator: filter_operator,
            value: geometry_value,
            distance,
        };
        
        let query = SearchQuery {
            filters: vec![filter],
            sort: None,
            limit: None,
            offset: None,
        };
        
        // Execute search
        let indexed_objects = search_store.search(&object_type, &query).await
            .map_err(|e| async_graphql::Error::new(format!("Spatial search error: {}", e)))?;
        
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
    
    /// Temporal query - query objects by year/vintage
    async fn temporal_query(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        year: Option<i64>,
        year_range_start: Option<i64>,
        year_range_end: Option<i64>,
        as_of_date: Option<String>, // ISO 8601 datetime string
    ) -> FieldResult<Vec<ObjectResult>> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let versioning = ctx.data::<Arc<time_query::TimeQuery>>()?;
        let hydrator = ctx.data::<ObjectHydrator>()?;
        
        let object_type_def = ontology.get_object_type(&object_type)
            .ok_or_else(|| async_graphql::Error::new("Object type not found"))?;
        
        let historical_objects = if let Some(as_of_str) = as_of_date {
            // Parse as_of_date
            let as_of = chrono::DateTime::parse_from_rfc3339(&as_of_str)
                .map_err(|e| async_graphql::Error::new(format!("Invalid date format: {}", e)))?
                .with_timezone(&chrono::Utc);
            
            versioning.query_as_of_date(&object_type, as_of, year)
        } else if let (Some(start), Some(end)) = (year_range_start, year_range_end) {
            versioning.query_by_year_range(&object_type, start, end, None)
        } else if let Some(y) = year {
            versioning.query_by_year(&object_type, y, None)
        } else {
            return Err(async_graphql::Error::new(
                "Must provide either year, year_range_start/year_range_end, or as_of_date"
            ));
        };
        
        // Convert historical objects to hydrated objects
        let mut results = Vec::new();
        for historical in historical_objects {
            // Create a temporary IndexedObject from HistoricalObject
            let indexed = indexing::store::IndexedObject {
                object_type: historical.object_type.clone(),
                object_id: historical.object_id.clone(),
                properties: historical.properties.clone(),
                indexed_at: historical.reconstructed_at,
            };
            
            if let Ok(hydrated) = hydrator.hydrate_from_indexed(&indexed, object_type_def) {
                results.push(ObjectResult {
                    object_type: hydrated.object_type,
                    object_id: hydrated.object_id,
                    title: hydrated.title,
                    properties: serde_json::to_string(&hydrated.properties).unwrap_or_else(|_| "{}".to_string()),
                });
            }
        }
        
        Ok(results)
    }
    
    /// Get available years for an object type
    async fn get_available_years(
        &self,
        ctx: &Context<'_>,
        object_type: String,
    ) -> FieldResult<Vec<i64>> {
        // Try to get years from in-memory store first
        let data_store = ctx.data::<Arc<tokio::sync::RwLock<HashMap<String, Vec<Value>>>>>();
        
        if let Ok(store) = data_store {
            let store_read = store.read().await;
            if let Some(objects) = store_read.get(&object_type) {
                let mut years: std::collections::HashSet<i64> = std::collections::HashSet::new();
                for obj in objects {
                    if let Some(year) = obj.get("year").and_then(|v| v.as_i64()) {
                        years.insert(year);
                    }
                }
                let mut years_vec: Vec<i64> = years.into_iter().collect();
                years_vec.sort();
                return Ok(years_vec);
            }
        }
        
        // Fallback to versioning service
        let versioning = ctx.data::<Arc<time_query::TimeQuery>>();
        if let Ok(vq) = versioning {
            return Ok(vq.get_available_years(&object_type, None));
        }
        
        // Default fallback
        Ok(vec![2010, 2020])
    }
    
    /// Traverse graph with filters and aggregations
    async fn traverse_graph(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        object_id: String,
        link_types: Vec<String>,
        max_hops: usize,
        aggregate_property: Option<String>,
        aggregate_operation: Option<String>, // "count", "sum", "avg", "min", "max"
    ) -> FieldResult<TraversalResult> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let graph_store = ctx.data::<Arc<dyn GraphStore>>()?;
        let search_store = ctx.data::<Arc<dyn SearchStore>>()?;
        let hydrator = ctx.data::<ObjectHydrator>()?;
        
        // If aggregation is requested, use aggregation traversal
        if let (Some(prop), Some(op)) = (aggregate_property, aggregate_operation) {
            let aggregation_op = match op.to_lowercase().as_str() {
                "count" => indexing::store::Aggregation::Count,
                "sum" => indexing::store::Aggregation::Sum(prop.clone()),
                "avg" => indexing::store::Aggregation::Avg(prop.clone()),
                "min" => indexing::store::Aggregation::Min(prop.clone()),
                "max" => indexing::store::Aggregation::Max(prop.clone()),
                _ => return Err(async_graphql::Error::new(format!(
                    "Invalid aggregation operation: {}. Valid: count, sum, avg, min, max",
                    op
                ))),
            };
            
            let aggregation = indexing::store::TraversalAggregation {
                property: prop,
                operation: aggregation_op,
                object_filters: vec![],
            };
            
            let result = graph_store.traverse_with_aggregation(
                &object_id,
                &link_types,
                max_hops,
                &aggregation,
            ).await
            .map_err(|e| async_graphql::Error::new(format!("Traversal error: {}", e)))?;
            
            return Ok(TraversalResult {
                object_ids: vec![],
                aggregated_value: Some(serde_json::to_string(&result.value).unwrap_or_else(|_| "null".to_string())),
                count: Some(result.count),
            });
        }
        
        // Regular traversal
        let object_ids = graph_store.traverse(
            &object_id,
            &link_types,
            max_hops,
        ).await
        .map_err(|e| async_graphql::Error::new(format!("Traversal error: {}", e)))?;
        
        Ok(TraversalResult {
            object_ids: object_ids.clone(),
            aggregated_value: None,
            count: Some(object_ids.len()),
        })
    }
    
    /// Aggregate query - perform aggregations on objects
    async fn aggregate(
        &self,
        ctx: &Context<'_>,
        object_type: String,
        aggregations: Vec<AggregationInput>,
        filters: Option<Vec<FilterInput>>,
        group_by: Option<Vec<String>>,
    ) -> FieldResult<AggregationResult> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let columnar_store = ctx.data::<Arc<dyn indexing::store::ColumnarStore>>()?;
        
        let object_type_def = ontology.get_object_type(&object_type)
            .ok_or_else(|| async_graphql::Error::new("Object type not found"))?;
        
        // Convert GraphQL aggregations to store aggregations
        let mut store_aggregations = Vec::new();
        for agg_input in aggregations {
            let agg = match agg_input.operation.to_lowercase().as_str() {
                "count" => indexing::store::Aggregation::Count,
                "sum" => indexing::store::Aggregation::Sum(agg_input.property.clone()),
                "avg" => indexing::store::Aggregation::Avg(agg_input.property.clone()),
                "min" => indexing::store::Aggregation::Min(agg_input.property.clone()),
                "max" => indexing::store::Aggregation::Max(agg_input.property.clone()),
                _ => return Err(async_graphql::Error::new(format!(
                    "Invalid aggregation operation: {}. Valid: count, sum, avg, min, max",
                    agg_input.operation
                ))),
            };
            store_aggregations.push(agg);
        }
        
        // Convert filters (simplified - would need proper parsing)
        let store_filters = vec![]; // TODO: Convert FilterInput to Filter
        
        // Build analytics query
        let query = indexing::store::AnalyticsQuery {
            aggregations: store_aggregations,
            filters: store_filters,
            group_by: group_by.unwrap_or_default(),
        };
        
        // Execute aggregation
        let result = columnar_store.query_analytics(&object_type, &query).await
            .map_err(|e| async_graphql::Error::new(format!("Aggregation error: {}", e)))?;
        
        // Convert results
        let rows: Vec<serde_json::Value> = result.rows.iter()
            .map(|row| {
                let mut json_row = serde_json::Map::new();
                for (key, value) in row {
                    json_row.insert(key.clone(), serde_json::to_value(value).unwrap_or(serde_json::Value::Null));
                }
                serde_json::Value::Object(json_row)
            })
            .collect();
        
        Ok(AggregationResult {
            rows: serde_json::to_string(&rows).unwrap_or_else(|_| "[]".to_string()),
            total: result.total,
        })
    }
    
    /// Call a function defined in the ontology
    async fn call_function(
        &self,
        ctx: &Context<'_>,
        function_id: String,
        parameters: HashMap<String, String>, // JSON strings representing PropertyValues
    ) -> FieldResult<FunctionResult> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let graph_store = ctx.data::<Arc<dyn GraphStore>>()?;
        let search_store = ctx.data::<Arc<dyn SearchStore>>()?;
        
        // Get function definition
        let function_def = ontology.get_function_type(&function_id)
            .ok_or_else(|| async_graphql::Error::new(format!("Function '{}' not found", function_id)))?;
        
        // Parse parameters from JSON strings to PropertyValues
        let mut param_map = ontology_engine::PropertyMap::new();
        for (key, json_value) in parameters {
            let value: serde_json::Value = serde_json::from_str(&json_value)
                .map_err(|e| async_graphql::Error::new(format!("Invalid parameter JSON for '{}': {}", key, e)))?;
            
            let prop_value = match value {
                serde_json::Value::String(s) => ontology_engine::PropertyValue::String(s),
                serde_json::Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        ontology_engine::PropertyValue::Integer(i)
                    } else if let Some(d) = n.as_f64() {
                        ontology_engine::PropertyValue::Double(d)
                    } else {
                        return Err(async_graphql::Error::new(format!("Invalid number for parameter '{}'", key)));
                    }
                }
                serde_json::Value::Bool(b) => ontology_engine::PropertyValue::Boolean(b),
                serde_json::Value::Array(arr) => {
                    let prop_values: Result<Vec<ontology_engine::PropertyValue>, _> = arr.into_iter()
                        .map(|v| {
                            match v {
                                serde_json::Value::String(s) => Ok(ontology_engine::PropertyValue::String(s)),
                                serde_json::Value::Number(n) => {
                                    if let Some(i) = n.as_i64() {
                                        Ok(ontology_engine::PropertyValue::Integer(i))
                                    } else if let Some(d) = n.as_f64() {
                                        Ok(ontology_engine::PropertyValue::Double(d))
                                    } else {
                                        Err("Invalid number in array")
                                    }
                                }
                                serde_json::Value::Bool(b) => Ok(ontology_engine::PropertyValue::Boolean(b)),
                                _ => Err("Unsupported array element type"),
                            }
                        })
                        .collect();
                    ontology_engine::PropertyValue::Array(prop_values
                        .map_err(|e| async_graphql::Error::new(format!("Invalid array for parameter '{}': {:?}", key, e)))?)
                }
                _ => return Err(async_graphql::Error::new(format!("Unsupported parameter type for '{}'", key))),
            };
            
            param_map.insert(key, prop_value);
        }
        
        // Execute function
        let result = FunctionExecutor::execute(
            function_def,
            &param_map,
            None, // get_object_property callback - would need to be implemented
            None, // get_linked_objects callback - would need to be implemented
            None, // aggregate_linked_properties callback - would need to be implemented
        ).await
        .map_err(|e| async_graphql::Error::new(format!("Function execution error: {}", e)))?;
        
        Ok(FunctionResult {
            value: serde_json::to_string(&result.value).unwrap_or_else(|_| "null".to_string()),
            cached: false, // TODO: Implement caching
        })
    }
    
    /// Query objects implementing an interface (polymorphic query)
    async fn query_interface(
        &self,
        ctx: &Context<'_>,
        interface_id: String,
        filters: Option<Vec<FilterInput>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> FieldResult<Vec<ObjectResult>> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        let search_store = ctx.data::<Arc<dyn SearchStore>>()?;
        let hydrator = ctx.data::<ObjectHydrator>()?;
        
        // Get interface definition
        let interface = ontology.get_interface(&interface_id)
            .ok_or_else(|| async_graphql::Error::new(format!("Interface '{}' not found", interface_id)))?;
        
        // Get all object types that implement this interface
        let implementers = InterfaceValidator::get_implementers(
            &interface_id,
            ontology.object_types(),
        );
        
        if implementers.is_empty() {
            return Ok(Vec::new());
        }
        
        // Query each implementing object type and combine results
        let mut all_results = Vec::new();
        for object_type in implementers {
            // Build filters (simplified - would need proper filter conversion)
            let query = SearchQuery {
                filters: vec![], // TODO: Convert FilterInput to Filter for each object type
                sort: None,
                limit,
                offset,
            };
            
            // Search objects of this type
            let indexed_objects = search_store.search(&object_type.id, &query).await
                .map_err(|e| async_graphql::Error::new(format!("Search error: {}", e)))?;
            
            // Hydrate and add to results
            let hydrated = hydrator.hydrate_batch(&indexed_objects, object_type)
                .map_err(|e| async_graphql::Error::new(format!("Hydration error: {}", e)))?;
            
            for h in hydrated {
                all_results.push(ObjectResult {
                    object_type: h.object_type,
                    object_id: h.object_id,
                    title: h.title,
                    properties: serde_json::to_string(&h.properties).unwrap_or_else(|_| "{}".to_string()),
                });
            }
        }
        
        Ok(all_results)
    }
    
    /// Get all available functions
    async fn get_functions(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<FunctionDefinition>> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        
        let functions: Vec<FunctionDefinition> = ontology
            .function_types()
            .map(|f| {
                let parameters: Vec<PropertyOutput> = f.parameters.iter().map(|p| {
                    PropertyOutput {
                        id: p.id.clone(),
                        display_name: p.display_name.clone(),
                        property_type: format!("{:?}", p.property_type),
                        required: p.required,
                    }
                }).collect();
                
                FunctionDefinition {
                    id: f.id.clone(),
                    display_name: f.display_name.clone(),
                    description: f.description.clone(),
                    parameters,
                    return_type: format!("{:?}", f.return_type),
                    cacheable: f.cacheable,
                }
            })
            .collect();
        
        Ok(functions)
    }
    
    /// Execute a function with parameters
    async fn execute_function(
        &self,
        ctx: &Context<'_>,
        function_id: String,
        parameters: HashMap<String, String>, // JSON strings
    ) -> FieldResult<FunctionResult> {
        // Use existing call_function implementation
        self.call_function(ctx, function_id, parameters).await
    }
    
    /// Get all available interfaces
    async fn get_interfaces(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<InterfaceDefinition>> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        
        let interfaces: Vec<InterfaceDefinition> = ontology
            .interfaces()
            .map(|i| {
                let properties: Vec<PropertyOutput> = i.properties.iter().map(|p| {
                    PropertyOutput {
                        id: p.id.clone(),
                        display_name: p.display_name.clone(),
                        property_type: format!("{:?}", p.property_type),
                        required: p.required,
                    }
                }).collect();
                
                // Get implementers
                let implementers: Vec<ImplementerInfo> = InterfaceValidator::get_implementers(
                    &i.id,
                    ontology.object_types(),
                )
                .iter()
                .map(|ot| {
                    // Count would need to come from search store - simplified for now
                    ImplementerInfo {
                        object_type: ot.id.clone(),
                        count: 0, // TODO: Get actual count from search store
                    }
                })
                .collect();
                
                InterfaceDefinition {
                    id: i.id.clone(),
                    display_name: i.display_name.clone(),
                    properties,
                    implementers,
                }
            })
            .collect();
        
        Ok(interfaces)
    }
    
    /// Query objects by interface (alias for query_interface)
    async fn query_by_interface(
        &self,
        ctx: &Context<'_>,
        interface_id: String,
        filters: Option<Vec<FilterInput>>,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> FieldResult<Vec<ObjectResult>> {
        // Use existing query_interface implementation
        self.query_interface(ctx, interface_id, filters, limit, offset).await
    }
    
    /// Get all object types
    async fn get_object_types(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<ObjectTypeResult>> {
        let ontology = ctx.data::<Arc<Ontology>>()?;
        
        let object_types: Vec<ObjectTypeResult> = ontology
            .object_types()
            .map(|ot| ObjectTypeResult {
                id: ot.id.clone(),
                display_name: ot.display_name.clone(),
            })
            .collect();
        
        Ok(object_types)
    }
}


/// Input for aggregation operations
#[derive(InputObject)]
struct AggregationInput {
    property: String,
    operation: String, // "count", "sum", "avg", "min", "max"
}

/// GraphQL result type for aggregations
#[derive(SimpleObject)]
pub struct AggregationResult {
    pub rows: String, // JSON array of aggregated rows
    pub total: usize,
}

/// Input for search filters
#[derive(InputObject)]
struct FilterInput {
    property: String,
    operator: String,
    value: String, // JSON string for now - TODO: implement proper PropertyValue GraphQL type
    distance: Option<f64>, // For spatial WithinDistance operator
}

/// GraphQL result type for objects
#[derive(SimpleObject)]
pub struct ObjectResult {
    pub object_type: String,
    pub object_id: String,
    pub title: String,
    pub properties: String, // JSON string for now - TODO: implement proper PropertyMap GraphQL type
}

/// GraphQL result type for graph traversal
#[derive(SimpleObject)]
pub struct TraversalResult {
    pub object_ids: Vec<String>,
    pub aggregated_value: Option<String>, // JSON string of aggregated value
    pub count: Option<usize>,
}

/// Pagination info for cursor-based pagination
#[derive(SimpleObject)]
pub struct PageInfo {
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub start_cursor: Option<String>,
    pub end_cursor: Option<String>,
}

/// Paginated result wrapper
#[derive(SimpleObject)]
pub struct PaginatedObjectResult {
    pub items: Vec<ObjectResult>,
    pub page_info: PageInfo,
    pub total_count: usize,
}

/// GraphQL result type for function calls
#[derive(SimpleObject)]
pub struct FunctionResult {
    pub value: String, // JSON string of the returned PropertyValue
    pub cached: bool,
}

/// GraphQL result type for object types
#[derive(SimpleObject)]
pub struct ObjectTypeResult {
    pub id: String,
    #[graphql(name = "displayName")]
    pub display_name: String,
}

/// GraphQL result type for property definitions (output)
#[derive(SimpleObject, Clone)]
pub struct PropertyOutput {
    pub id: String,
    #[graphql(name = "displayName")]
    pub display_name: Option<String>,
    #[graphql(name = "type")]
    pub property_type: String,
    pub required: bool,
}

/// GraphQL input for function parameters
#[derive(InputObject, Clone)]
pub struct PropertyInput {
    pub id: String,
    #[graphql(name = "displayName")]
    pub display_name: Option<String>,
    #[graphql(name = "type")]
    pub property_type: String,
    pub required: bool,
}

/// GraphQL result type for function definitions
#[derive(SimpleObject)]
pub struct FunctionDefinition {
    pub id: String,
    #[graphql(name = "displayName")]
    pub display_name: String,
    pub description: Option<String>,
    pub parameters: Vec<PropertyOutput>,
    #[graphql(name = "returnType")]
    pub return_type: String,
    pub cacheable: bool,
}

/// GraphQL result type for interface definitions
#[derive(SimpleObject)]
pub struct InterfaceDefinition {
    pub id: String,
    #[graphql(name = "displayName")]
    pub display_name: String,
    pub properties: Vec<PropertyOutput>,
    #[graphql(name = "implementers")]
    pub implementers: Vec<ImplementerInfo>,
}

/// GraphQL result for interface implementers
#[derive(SimpleObject)]
pub struct ImplementerInfo {
    #[graphql(name = "objectType")]
    pub object_type: String,
    pub count: usize,
}

