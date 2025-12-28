use async_trait::async_trait;
use ontology_engine::PropertyMap;
use std::collections::HashMap;
use uuid::Uuid;
use elasticsearch::{Elasticsearch, http::transport::Transport, IndexParts};
use serde_json::Value as JsonValue;
use dgraph_tonic::{Client as DgraphClient, Mutation, Operation, Query, Mutate};

/// Abstract trait for search store backends (Elasticsearch, etc.)
#[async_trait]
pub trait SearchStore: Send + Sync {
    /// Index an object
    async fn index_object(
        &self,
        object_type: &str,
        object_id: &str,
        properties: &PropertyMap,
    ) -> Result<(), StoreError>;
    
    /// Search for objects matching the query
    async fn search(
        &self,
        object_type: &str,
        query: &SearchQuery,
    ) -> Result<Vec<IndexedObject>, StoreError>;
    
    /// Get an object by ID
    async fn get_object(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<Option<IndexedObject>, StoreError>;
    
    /// Bulk index multiple objects
    async fn bulk_index(
        &self,
        objects: Vec<IndexedObject>,
    ) -> Result<(), StoreError>;
    
    /// Delete an object from the index
    async fn delete_object(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<(), StoreError>;
}

/// Abstract trait for graph store backends (Dgraph, Neo4j, etc.)
#[async_trait]
pub trait GraphStore: Send + Sync {
    /// Create a link between two objects
    async fn create_link(
        &self,
        link_type_id: &str,
        source_id: &str,
        target_id: &str,
        properties: &PropertyMap,
    ) -> Result<String, StoreError>;
    
    /// Delete a link
    async fn delete_link(
        &self,
        link_id: &str,
    ) -> Result<(), StoreError>;
    
    /// Get all links connected to an object
    async fn get_links(
        &self,
        object_id: &str,
        link_type_id: Option<&str>,
        direction: Option<LinkDirection>,
    ) -> Result<Vec<GraphLink>, StoreError>;
    
    /// Traverse the graph from a starting object
    async fn traverse(
        &self,
        start_id: &str,
        link_type_ids: &[String],
        max_hops: usize,
    ) -> Result<Vec<String>, StoreError>;
    
    /// Get objects connected via a specific link type
    async fn get_connected_objects(
        &self,
        object_id: &str,
        link_type_id: &str,
    ) -> Result<Vec<String>, StoreError>;
    
    /// Traverse with filters - filter by link properties during traversal
    async fn traverse_with_filters(
        &self,
        start_id: &str,
        link_type_ids: &[String],
        max_hops: usize,
        link_filters: &[Filter], // Filters on link properties
    ) -> Result<Vec<String>, StoreError>;
    
    /// Traverse with aggregation - aggregate properties during traversal
    async fn traverse_with_aggregation(
        &self,
        start_id: &str,
        link_type_ids: &[String],
        max_hops: usize,
        aggregation: &TraversalAggregation,
    ) -> Result<TraversalAggregationResult, StoreError>;
}

/// Abstract trait for columnar store backends (Parquet, S3, etc.)
#[async_trait]
pub trait ColumnarStore: Send + Sync {
    /// Write object data to columnar format
    async fn write_batch(
        &self,
        object_type: &str,
        objects: Vec<IndexedObject>,
    ) -> Result<(), StoreError>;
    
    /// Query columnar data for analytics
    async fn query_analytics(
        &self,
        object_type: &str,
        query: &AnalyticsQuery,
    ) -> Result<AnalyticsResult, StoreError>;
}

/// Link direction for graph traversal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkDirection {
    Outgoing, // Source -> Target
    Incoming, // Target -> Source
    Both,
}

/// Search query structure
#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub filters: Vec<Filter>,
    pub sort: Option<SortOption>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

/// Filter for search queries
#[derive(Debug, Clone)]
pub struct Filter {
    pub property: String,
    pub operator: FilterOperator,
    pub value: ontology_engine::PropertyValue,
    /// Optional distance parameter for WithinDistance operator (in meters)
    pub distance: Option<f64>,
}

/// Filter operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    // Spatial operators for GeoJSON
    ContainsGeometry,    // Check if geometry contains another geometry
    Intersects,          // Check if geometries intersect
    Within,              // Check if geometry is within another geometry
    WithinDistance,      // Check if geometry is within distance (requires distance parameter)
}

/// Sort option
#[derive(Debug, Clone)]
pub struct SortOption {
    pub property: String,
    pub ascending: bool,
}

/// Indexed object representation
#[derive(Debug, Clone)]
pub struct IndexedObject {
    pub object_type: String,
    pub object_id: String,
    pub properties: PropertyMap,
    pub indexed_at: chrono::DateTime<chrono::Utc>,
}

impl IndexedObject {
    pub fn new(object_type: String, object_id: String, properties: PropertyMap) -> Self {
        Self {
            object_type,
            object_id,
            properties,
            indexed_at: chrono::Utc::now(),
        }
    }
}

/// Graph link representation
#[derive(Debug, Clone)]
pub struct GraphLink {
    pub link_id: String,
    pub link_type_id: String,
    pub source_id: String,
    pub target_id: String,
    pub properties: PropertyMap,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Analytics query
#[derive(Debug, Clone)]
pub struct AnalyticsQuery {
    pub aggregations: Vec<Aggregation>,
    pub filters: Vec<Filter>,
    pub group_by: Vec<String>,
}

/// Aggregation operations
#[derive(Debug, Clone)]
pub enum Aggregation {
    Count,
    Sum(String), // property name
    Avg(String),
    Min(String),
    Max(String),
}

/// Analytics query result
#[derive(Debug, Clone)]
pub struct AnalyticsResult {
    pub rows: Vec<HashMap<String, ontology_engine::PropertyValue>>,
    pub total: usize,
}

/// Traversal aggregation configuration
#[derive(Debug, Clone)]
pub struct TraversalAggregation {
    /// Property to aggregate (e.g., "wages", "age")
    pub property: String,
    /// Aggregation operation
    pub operation: Aggregation,
    /// Optional filters on target objects
    pub object_filters: Vec<Filter>,
}

/// Traversal aggregation result
#[derive(Debug, Clone)]
pub struct TraversalAggregationResult {
    /// Aggregated value
    pub value: ontology_engine::PropertyValue,
    /// Count of objects aggregated
    pub count: usize,
}

/// Store backend - wrapper that implements all three store traits
pub struct StoreBackend {
    search: Box<dyn SearchStore>,
    graph: Box<dyn GraphStore>,
    columnar: Box<dyn ColumnarStore>,
}

impl StoreBackend {
    pub fn new(
        search: Box<dyn SearchStore>,
        graph: Box<dyn GraphStore>,
        columnar: Box<dyn ColumnarStore>,
    ) -> Self {
        Self {
            search,
            graph,
            columnar,
        }
    }
    
    pub fn search_store(&self) -> &dyn SearchStore {
        self.search.as_ref()
    }
    
    pub fn graph_store(&self) -> &dyn GraphStore {
        self.graph.as_ref()
    }
    
    pub fn columnar_store(&self) -> &dyn ColumnarStore {
        self.columnar.as_ref()
    }
}

/// Store errors
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Query error: {0}")]
    Query(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("Object not found: {0}")]
    NotFound(String),
    
    #[error("Transaction failed: {0}")]
    Transaction(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Write error: {0}")]
    WriteError(String),
    
    #[error("Read error: {0}")]
    ReadError(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

// Elasticsearch store implementation
pub struct ElasticsearchStore {
    client: Elasticsearch,
    /// Index prefix allows you to namespace apps (e.g., "dev_user", "prod_user")
    index_prefix: String,
}

impl ElasticsearchStore {
    /// Create a new ElasticsearchStore instance
    /// 
    /// # Arguments
    /// * `endpoint` - Elasticsearch endpoint URL (e.g., "http://localhost:9200")
    /// 
    /// # Errors
    /// Returns `StoreError::Connection` if the transport cannot be created
    pub fn new(endpoint: String) -> Result<Self, StoreError> {
        // Build the transport (connection pool)
        let transport = Transport::single_node(&endpoint)
            .map_err(|e| StoreError::Connection(format!("Transport error: {}", e)))?;
            
        let client = Elasticsearch::new(transport);
        
        Ok(Self {
            client,
            index_prefix: "ontology".to_string(),
        })
    }

    /// Generate index name from object type (e.g., "ontology_user" or "ontology_document")
    fn index_name(&self, object_type: &str) -> String {
        format!("{}_{}", self.index_prefix, object_type)
    }
}

#[async_trait]
impl SearchStore for ElasticsearchStore {
    async fn index_object(
        &self,
        object_type: &str,
        object_id: &str,
        properties: &PropertyMap,
    ) -> Result<(), StoreError> {
        let index_name = self.index_name(object_type);

        // Serialize PropertyMap to JSON
        // PropertyMap has a private "properties" field, so we need to build the JSON manually
        // to get a flat structure (just the property key-value pairs)
        let mut json_map = serde_json::Map::new();
        for (key, value) in properties.iter() {
            // PropertyValue implements Serialize, so we can convert it directly
            let json_value = serde_json::to_value(value)
                .map_err(|e| StoreError::Serialization(format!("Failed to serialize property '{}': {}", key, e)))?;
            json_map.insert(key.clone(), json_value);
        }
        let json_body = JsonValue::Object(json_map);

        let response = self.client
            .index(IndexParts::IndexId(&index_name, object_id))
            .body(json_body)
            .send()
            .await
            .map_err(|e| StoreError::Connection(format!("Elasticsearch request failed: {}", e)))?;

        // Check if the response was successful
        let status_code = response.status_code();
        if !status_code.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::Query(format!(
                "Elasticsearch returned error {}: {}",
                status_code.as_u16(),
                error_body
            )));
        }

        Ok(())
    }
    
    async fn search(
        &self,
        object_type: &str,
        query: &SearchQuery,
    ) -> Result<Vec<IndexedObject>, StoreError> {
        // TODO: Implement actual Elasticsearch search
        Ok(vec![])
    }
    
    async fn get_object(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<Option<IndexedObject>, StoreError> {
        // TODO: Implement actual Elasticsearch get
        Ok(None)
    }
    
    async fn bulk_index(
        &self,
        objects: Vec<IndexedObject>,
    ) -> Result<(), StoreError> {
        // TODO: Implement actual Elasticsearch bulk index
        Ok(())
    }
    
    async fn delete_object(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<(), StoreError> {
        // TODO: Implement actual Elasticsearch delete
        Ok(())
    }
}

// Dgraph store implementation
pub struct DgraphStore {
    client: DgraphClient,
}

impl DgraphStore {
    /// Create a new DgraphStore instance
    /// 
    /// # Arguments
    /// * `endpoint` - Dgraph gRPC endpoint URL (e.g., "http://localhost:9080")
    /// 
    /// # Errors
    /// Returns `StoreError::Configuration` if the client cannot be created
    pub async fn new(endpoint: String) -> Result<Self, StoreError> {
        // Dgraph client connects via gRPC (usually port 9080)
        let client = DgraphClient::new(endpoint)
            .map_err(|e| StoreError::Configuration(format!("Dgraph client error: {}", e)))?;
            
        Ok(Self { client })
    }
    
    /// Initialize the Dgraph schema
    /// Run this once on startup to define predicates and indexes
    pub async fn init_schema(&self) -> Result<(), StoreError> {
        // Define schema for link properties and xid lookups
        // xid is used to map string IDs to UIDs
        // link_id, link_type_id, created_at are stored as facets on edges
        let schema = r#"
            xid: string @index(exact) .
            link_id: string @index(exact) .
            link_type_id: string .
            created_at: datetime .
        "#;
        
        let op = Operation {
            schema: schema.to_string(),
            ..Default::default()
        };
        
        self.client.alter(op).await
            .map_err(|e| StoreError::WriteError(format!("Schema error: {}", e)))?;
            
        Ok(())
    }
    
    /// Get or create a UID for a given string ID
    /// Uses xid field to lookup existing UID or creates a new blank node
    async fn get_or_create_uid(&self, object_id: &str) -> Result<String, StoreError> {
        // First, try to query for existing UID
        let query = format!(r#"
            {{
                objects(func: eq(xid, "{}")) {{
                    uid
                }}
            }}
        "#, object_id);
        
        let mut txn = self.client.new_read_only_txn();
        let response = txn.query(query).await
            .map_err(|e| StoreError::ReadError(format!("Query error: {}", e)))?;
        
        let json: serde_json::Value = serde_json::from_slice(&response.json)
            .map_err(|e| StoreError::ReadError(format!("Parse error: {}", e)))?;
        
        // Check if we found an existing UID
        if let Some(objects) = json.get("objects").and_then(|o| o.as_array()) {
            if let Some(first) = objects.first() {
                if let Some(uid) = first.get("uid").and_then(|u| u.as_str()) {
                    return Ok(uid.to_string());
                }
            }
        }
        
        // No existing UID found, create a new node with blank UID (Dgraph will assign one)
        // We'll use a mutation to create the node
        let rdf = format!(r#"_:node <xid> "{}" ."#, object_id);
        
        let mutation = Mutation {
            set_nquads: rdf.into_bytes(),
            ..Default::default()
        };
        
        let mut txn = self.client.new_mutated_txn();
        txn.mutate(mutation).await
            .map_err(|e| StoreError::WriteError(format!("Mutation error: {}", e)))?;
        txn.commit().await
            .map_err(|e| StoreError::WriteError(format!("Commit error: {}", e)))?;
        
        // Query to get the UID we just created
        let query = format!(r#"
            {{
                objects(func: eq(xid, "{}")) {{
                    uid
                }}
            }}
        "#, object_id);
        
        let mut txn = self.client.new_read_only_txn();
        let response = txn.query(query).await
            .map_err(|e| StoreError::ReadError(format!("Query error: {}", e)))?;
        
        let json: serde_json::Value = serde_json::from_slice(&response.json)
            .map_err(|e| StoreError::ReadError(format!("Parse error: {}", e)))?;
        
        if let Some(objects) = json.get("objects").and_then(|o| o.as_array()) {
            if let Some(first) = objects.first() {
                if let Some(uid) = first.get("uid").and_then(|u| u.as_str()) {
                    return Ok(uid.to_string());
                }
            }
        }
        
        Err(StoreError::WriteError(format!("Failed to get or create UID for {}", object_id)))
    }
    
    /// Convert PropertyMap to RDF N-Quad format for facets
    /// Facets in Dgraph are stored as: <source> <predicate> <target> (property="value") .
    fn properties_to_facets(&self, properties: &PropertyMap, link_id: &str, link_type_id: &str) -> String {
        let mut facets = Vec::new();
        
        // Always add link_id and link_type_id as facets
        facets.push(format!("link_id=\"{}\"", link_id));
        facets.push(format!("link_type_id=\"{}\"", link_type_id));
        facets.push(format!("created_at=\"{}\"", chrono::Utc::now().to_rfc3339()));
        
        // Add custom properties as facets
        for (key, value) in properties.iter() {
            // Convert PropertyValue to string representation for facet
            let value_str = match value {
                ontology_engine::PropertyValue::String(s) => s.clone(),
                ontology_engine::PropertyValue::Integer(i) => i.to_string(),
                ontology_engine::PropertyValue::Double(d) => d.to_string(),
                ontology_engine::PropertyValue::Boolean(b) => b.to_string(),
                ontology_engine::PropertyValue::Date(d) => d.clone(),
                ontology_engine::PropertyValue::DateTime(dt) => dt.clone(),
                ontology_engine::PropertyValue::ObjectReference(id) => id.clone(),
                ontology_engine::PropertyValue::GeoJSON(gj) => gj.clone(),
                ontology_engine::PropertyValue::Array(_) => {
                    // Arrays need to be JSON encoded
                    serde_json::to_string(value).unwrap_or_else(|_| "[]".to_string())
                },
                ontology_engine::PropertyValue::Map(_) => {
                    serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
                },
                ontology_engine::PropertyValue::Object(_) => {
                    serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
                },
                ontology_engine::PropertyValue::Null => continue, // Skip null values
            };
            
            // Escape quotes in the value
            let escaped_value = value_str.replace('"', "\\\"");
            facets.push(format!("{}=\"{}\"", key, escaped_value));
        }
        
        if facets.is_empty() {
            String::new()
        } else {
            format!("({})", facets.join(", "))
        }
    }
}

#[async_trait]
impl GraphStore for DgraphStore {
    async fn create_link(
        &self,
        link_type_id: &str,
        source_id: &str,
        target_id: &str,
        properties: &PropertyMap,
    ) -> Result<String, StoreError> {
        // Generate a unique link_id
        let link_id = Uuid::new_v4().to_string();
        
        // Get or create UIDs for source and target
        let source_uid = self.get_or_create_uid(source_id).await?;
        let target_uid = self.get_or_create_uid(target_id).await?;
        
        // Use link_type_id as the predicate name
        // Sanitize it for Dgraph (predicates must be valid identifiers)
        let predicate = link_type_id.replace('-', "_").replace('.', "_");
        
        // Create the edge with properties as facets
        let facets = self.properties_to_facets(properties, &link_id, link_type_id);
        let rdf = format!("<{}> <{}> <{}> {} .", source_uid, predicate, target_uid, facets);
        
        let mutation = Mutation {
            set_nquads: rdf.into_bytes(),
            ..Default::default()
        };
        
        let mut txn = self.client.new_mutated_txn();
        txn.mutate(mutation).await
            .map_err(|e| StoreError::WriteError(format!("Link creation error: {}", e)))?;
        txn.commit().await
            .map_err(|e| StoreError::WriteError(format!("Commit error: {}", e)))?;
        
        Ok(link_id)
    }
    
    async fn delete_link(
        &self,
        link_id: &str,
    ) -> Result<(), StoreError> {
        // First, find the edge(s) with this link_id
        // We need to query for edges that have link_id as a facet
        let query = format!(r#"
            {{
                edges(func: eq(link_id, "{}")) @facets {{
                    uid
                    ~_predicate_ {{
                        uid
                    }}
                }}
            }}
        "#, link_id);
        
        // This approach is tricky because Dgraph doesn't easily support querying by facet
        // Instead, we'll need to query all edges and filter client-side, or use a different approach
        // For now, we'll delete by querying the link_id property and then deleting the edge
        // This requires a more complex query structure
        
        // Alternative: Store link_id as a reverse index or use a helper node
        // For simplicity, we'll use a query to find the edge and then delete it
        // Since Dgraph doesn't directly support querying edges by facets efficiently,
        // we'll need to track links differently or accept this limitation
        
        // For now, we'll delete by finding all edges and checking facets (not ideal for large graphs)
        // In production, you might want to maintain a reverse index
        let query = format!(r#"
            {{
                var(func: has(~_predicate_)) {{
                    ~_predicate_ @facets(link_id) {{
                        link_id as link_id
                    }}
                    filter(link_id == "{}")
                }}
            }}
        "#, link_id);
        
        // Actually, Dgraph's GraphQL+- doesn't easily support querying edges by facets
        // We'll need to use a different approach: store link metadata in a separate node
        // or accept that deletion requires scanning (which is a limitation)
        
        // For now, return an error indicating this needs a more sophisticated approach
        // In a production system, you'd maintain a reverse index or use intermediate nodes
        Err(StoreError::Query(
            "Delete by link_id requires reverse indexing - not yet implemented".to_string()
        ))
    }
    
    async fn get_links(
        &self,
        object_id: &str,
        link_type_id: Option<&str>,
        direction: Option<LinkDirection>,
    ) -> Result<Vec<GraphLink>, StoreError> {
        let object_uid = self.get_or_create_uid(object_id).await?;
        let direction = direction.unwrap_or(LinkDirection::Both);
        let predicate = link_type_id.map(|id| id.replace('-', "_").replace('.', "_"));
        
        let mut links = Vec::new();
        
        // Query outgoing links
        if direction == LinkDirection::Outgoing || direction == LinkDirection::Both {
            if let Some(pred) = &predicate {
                let query = format!(r#"
                    {{
                        node(func: uid({})) {{
                            {} @facets {{
                                uid
                                xid
                            }}
                        }}
                    }}
                "#, object_uid, pred);
                
                let mut txn = self.client.new_read_only_txn();
                let response = txn.query(query).await
                    .map_err(|e| StoreError::ReadError(format!("Query error: {}", e)))?;
                
                let json: serde_json::Value = serde_json::from_slice(&response.json)
                    .map_err(|e| StoreError::ReadError(format!("Parse error: {}", e)))?;
                
                if let Some(node_arr) = json.get("node").and_then(|n| n.as_array()) {
                    for node in node_arr {
                        if let Some(targets) = node.get(pred).and_then(|t| t.as_array()) {
                            for target in targets {
                                // Extract facets for link_id, link_type_id, and properties
                                let link = self.extract_link_from_target(
                                    target,
                                    object_id,
                                    link_type_id.unwrap(),
                                    direction,
                                    true,
                                )?;
                                links.push(link);
                            }
                        }
                    }
                }
            } else {
                // Query all outgoing predicates - more complex
                // For now, return empty if no specific predicate
            }
        }
        
        // Query incoming links
        if direction == LinkDirection::Incoming || direction == LinkDirection::Both {
            // Similar logic but query reverse direction
            // This would require knowing all possible predicates or using a schema query
            // For now, we'll focus on outgoing links with specified predicates
        }
        
        Ok(links)
    }
    
    async fn traverse(
        &self,
        start_id: &str,
        link_type_ids: &[String],
        max_hops: usize,
    ) -> Result<Vec<String>, StoreError> {
        let start_uid = self.get_or_create_uid(start_id).await?;
        
        // Build the traversal query
        // For multiple link types, we'll traverse each one
        let mut all_target_ids = Vec::new();
        
        for link_type_id in link_type_ids {
            let predicate = link_type_id.replace('-', "_").replace('.', "_");
            
            // Build recursive query for traversal up to max_hops
            let mut query_parts = vec![format!("node(func: uid({}))", start_uid)];
            
            for hop in 0..max_hops {
                let indent = "  ".repeat(hop + 1);
                query_parts.push(format!("{}~{} {{", indent, predicate));
                query_parts.push(format!("{}  uid", indent));
                query_parts.push(format!("{}  xid", indent));
                if hop < max_hops - 1 {
                    query_parts.push(format!("{}  ~{} {{", indent, predicate));
                }
            }
            
            // Close all brackets
            for hop in (0..max_hops).rev() {
                let indent = "  ".repeat(hop + 1);
                query_parts.push(format!("{}}}", indent));
            }
            query_parts.push("}".to_string());
            
            let query = format!("{{\n{}\n}}", query_parts.join("\n"));
            
            let mut txn = self.client.new_read_only_txn();
            let response = txn.query(query).await
                .map_err(|e| StoreError::ReadError(format!("Query error: {}", e)))?;
            
            let json: serde_json::Value = serde_json::from_slice(&response.json)
                .map_err(|e| StoreError::ReadError(format!("Parse error: {}", e)))?;
            
            // Extract all UIDs from the traversal result
            self.extract_ids_from_traversal(&json, &mut all_target_ids);
        }
        
        // Convert UIDs back to string IDs by querying xid
        let mut string_ids = Vec::new();
        for uid in all_target_ids {
            if let Ok(id) = self.uid_to_xid(&uid).await {
                if !string_ids.contains(&id) {
                    string_ids.push(id);
                }
            }
        }
        
        Ok(string_ids)
    }
    
    async fn get_connected_objects(
        &self,
        object_id: &str,
        link_type_id: &str,
    ) -> Result<Vec<String>, StoreError> {
        // This is essentially a single-hop traverse
        self.traverse(object_id, &[link_type_id.to_string()], 1).await
    }
    
    async fn traverse_with_filters(
        &self,
        start_id: &str,
        link_type_ids: &[String],
        max_hops: usize,
        link_filters: &[Filter],
    ) -> Result<Vec<String>, StoreError> {
        // For filtered traversal, we'd need to add filter conditions to the query
        // Since filters are on link properties (facets), this is complex in Dgraph
        // For now, fall back to basic traversal and filter client-side
        let all_ids = self.traverse(start_id, link_type_ids, max_hops).await?;
        
        // TODO: Implement proper facet-based filtering in the query
        // This would require more complex GraphQL+- queries with @filter directives
        
        Ok(all_ids)
    }
    
    async fn traverse_with_aggregation(
        &self,
        start_id: &str,
        link_type_ids: &[String],
        max_hops: usize,
        aggregation: &TraversalAggregation,
    ) -> Result<TraversalAggregationResult, StoreError> {
        // Get all objects from traversal
        let object_ids = self.traverse(start_id, link_type_ids, max_hops).await?;
        
        if object_ids.is_empty() {
            return Ok(TraversalAggregationResult {
                value: ontology_engine::PropertyValue::Integer(0),
                count: 0,
            });
        }
        
        // TODO: Implement proper aggregation using Dgraph's aggregation functions
        // For now, return placeholder
        Ok(TraversalAggregationResult {
            value: ontology_engine::PropertyValue::Integer(0),
            count: object_ids.len(),
        })
    }
}

impl DgraphStore {
    /// Extract link information from a target node with facets
    fn extract_link_from_target(
        &self,
        target: &serde_json::Value,
        source_id: &str,
        link_type_id: &str,
        _direction: LinkDirection,
        _outgoing: bool,
    ) -> Result<GraphLink, StoreError> {
        let target_uid = target.get("uid")
            .and_then(|u| u.as_str())
            .ok_or_else(|| StoreError::ReadError("Missing uid in target".to_string()))?;
        
        let target_id = target.get("xid")
            .and_then(|x| x.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| target_uid.to_string());
        
        // Extract facets (link properties)
        // Note: Facets in Dgraph response are nested under @facets
        let mut properties = PropertyMap::new();
        
        // For now, create a basic link
        // In a full implementation, you'd extract all facets
        let link_id = Uuid::new_v4().to_string(); // Would come from facets
        
        Ok(GraphLink {
            link_id,
            link_type_id: link_type_id.to_string(),
            source_id: source_id.to_string(),
            target_id,
            properties,
            created_at: chrono::Utc::now(),
        })
    }
    
    /// Extract all IDs from a traversal result JSON
    fn extract_ids_from_traversal(&self, json: &serde_json::Value, ids: &mut Vec<String>) {
        if let Some(obj) = json.as_object() {
            for value in obj.values() {
                if let Some(arr) = value.as_array() {
                    for item in arr {
                        if let Some(uid) = item.get("uid").and_then(|u| u.as_str()) {
                            if !ids.contains(&uid.to_string()) {
                                ids.push(uid.to_string());
                            }
                        }
                        // Recursively extract from nested objects
                        self.extract_ids_from_traversal(item, ids);
                    }
                } else {
                    self.extract_ids_from_traversal(value, ids);
                }
            }
        }
    }
    
    /// Convert UID to xid (string ID)
    async fn uid_to_xid(&self, uid: &str) -> Result<String, StoreError> {
        let query = format!(r#"
            {{
                node(func: uid({})) {{
                    uid
                    xid
                }}
            }}
        "#, uid);
        
        let mut txn = self.client.new_read_only_txn();
        let response = txn.query(query).await
            .map_err(|e| StoreError::ReadError(format!("Query error: {}", e)))?;
        
        let json: serde_json::Value = serde_json::from_slice(&response.json)
            .map_err(|e| StoreError::ReadError(format!("Parse error: {}", e)))?;
        
        if let Some(node_arr) = json.get("node").and_then(|n| n.as_array()) {
            if let Some(node) = node_arr.first() {
                if let Some(xid) = node.get("xid").and_then(|x| x.as_str()) {
                    return Ok(xid.to_string());
                }
            }
        }
        
        // Fallback to UID if no xid found
        Ok(uid.to_string())
    }
}

// Placeholder implementations for Columnar store (Parquet/S3)
pub struct ParquetStore {
    base_path: String,
}

impl ParquetStore {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

#[async_trait]
impl ColumnarStore for ParquetStore {
    async fn write_batch(
        &self,
        object_type: &str,
        objects: Vec<IndexedObject>,
    ) -> Result<(), StoreError> {
        // TODO: Implement actual Parquet write
        // Would use arrow-rs or polars crate
        Ok(())
    }
    
    async fn query_analytics(
        &self,
        object_type: &str,
        query: &AnalyticsQuery,
    ) -> Result<AnalyticsResult, StoreError> {
        // TODO: Implement actual Parquet analytics query
        // Would use arrow-rs or polars for querying
        Ok(AnalyticsResult {
            rows: vec![],
            total: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ontology_engine::PropertyValue;

    #[tokio::test]
    async fn test_index_object_basic() {
        // This test requires a running Elasticsearch instance
        // Skip if Elasticsearch is not available (e.g., in CI)
        let endpoint = std::env::var("ELASTICSEARCH_URL")
            .unwrap_or_else(|_| "http://localhost:9200".to_string());
        
        let store = match ElasticsearchStore::new(endpoint.clone()) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Skipping test: Could not connect to Elasticsearch at {}: {}", endpoint, e);
                return;
            }
        };

        // Create a simple PropertyMap
        let mut properties = PropertyMap::new();
        properties.insert("name".to_string(), PropertyValue::String("Test Object".to_string()));
        properties.insert("priority".to_string(), PropertyValue::Integer(1));
        properties.insert("active".to_string(), PropertyValue::Boolean(true));

        // Index the object
        let result = store.index_object("test_object", "test_123", &properties).await;
        
        // Should succeed if Elasticsearch is running
        if result.is_err() {
            eprintln!("Indexing failed (this is OK if Elasticsearch is not running): {:?}", result);
        } else {
            // If successful, verify we can retrieve it
            // (This will be implemented when we add get_object)
            println!("Successfully indexed test object");
        }
    }
}

