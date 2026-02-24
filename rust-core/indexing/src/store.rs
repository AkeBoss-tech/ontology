use async_trait::async_trait;
use ontology_engine::PropertyMap;
use std::collections::HashMap;
use uuid::Uuid;
use elasticsearch::{
    Elasticsearch, 
    http::transport::Transport, 
    IndexParts,
    SearchParts,
    GetParts,
    BulkParts,
    CountParts,
    DeleteParts,
    indices::IndicesExistsParts,
};
use serde_json::{Value as JsonValue, json};
use dgraph_tonic::{Client as DgraphClient, Mutation, Operation, Query, Mutate};
use polars::prelude::*;
use std::io::Cursor;
use std::fs::File;
use std::path::Path;

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
    
    /// Count objects matching the query (without fetching them)
    async fn count_objects(
        &self,
        object_type: &str,
        filters: Option<&[Filter]>,
    ) -> Result<u64, StoreError>;
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
    
    /// Compute centrality metrics for objects in the graph
    async fn compute_centrality(
        &self,
        object_type: &str,
        metric: CentralityMetric,
    ) -> Result<HashMap<String, f64>, StoreError>;
    
    /// Find communities/clusters in the graph
    async fn detect_communities(
        &self,
        object_type: &str,
        algorithm: CommunityAlgorithm,
    ) -> Result<HashMap<String, usize>, StoreError>;
    
    /// Find shortest path between objects
    async fn shortest_path(
        &self,
        source_id: &str,
        target_id: &str,
        link_types: &[String],
    ) -> Result<Vec<String>, StoreError>;
    
    /// Compute graph metrics (density, clustering coefficient, etc.)
    async fn graph_metrics(
        &self,
        object_type: &str,
    ) -> Result<GraphMetrics, StoreError>;
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

/// Refresh status for data freshness tracking
#[derive(Debug, Clone)]
pub enum RefreshStatus {
    UpToDate,
    Stale { days_behind: i64 },
    Failed { last_attempt: chrono::DateTime<chrono::Utc>, error: String },
}

/// Indexed object representation
#[derive(Debug, Clone)]
pub struct IndexedObject {
    pub object_type: String,
    pub object_id: String,
    pub properties: PropertyMap,
    pub indexed_at: chrono::DateTime<chrono::Utc>,
    
    // Data freshness metadata
    pub source_last_modified: Option<chrono::DateTime<chrono::Utc>>,
    pub refresh_frequency: Option<String>, // "daily", "hourly", "real-time"
    pub next_refresh: Option<chrono::DateTime<chrono::Utc>>,
    pub refresh_status: RefreshStatus,
}

impl IndexedObject {
    pub fn new(object_type: String, object_id: String, properties: PropertyMap) -> Self {
        Self {
            object_type,
            object_id,
            properties,
            indexed_at: chrono::Utc::now(),
            source_last_modified: None,
            refresh_frequency: None,
            next_refresh: None,
            refresh_status: RefreshStatus::UpToDate,
        }
    }
    
    /// Check if the object is stale
    pub fn is_stale(&self) -> bool {
        match &self.refresh_status {
            RefreshStatus::Stale { .. } => true,
            RefreshStatus::Failed { .. } => true,
            RefreshStatus::UpToDate => false,
        }
    }
    
    /// Get days behind if stale
    pub fn days_behind(&self) -> Option<i64> {
        match &self.refresh_status {
            RefreshStatus::Stale { days_behind } => Some(*days_behind),
            RefreshStatus::Failed { last_attempt, .. } => {
                Some((chrono::Utc::now() - *last_attempt).num_days())
            }
            RefreshStatus::UpToDate => None,
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
    Median(String),
    StdDev(String),
    Variance(String),
    Percentile(String, f64), // property name, percentile (0.0-1.0)
    DistinctCount(String),
    TopN(String, usize), // property name, N
    BottomN(String, usize),
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

/// Centrality metrics for graph analysis
#[derive(Debug, Clone)]
pub enum CentralityMetric {
    Degree,
    Betweenness,
    PageRank { damping: f64 },
}

/// Community detection algorithms
#[derive(Debug, Clone)]
pub enum CommunityAlgorithm {
    Louvain,
    LabelPropagation,
}

/// Graph-level metrics
#[derive(Debug, Clone)]
pub struct GraphMetrics {
    pub node_count: usize,
    pub edge_count: usize,
    pub density: f64,
    pub average_clustering_coefficient: f64,
    pub average_degree: f64,
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
    /// Base URL for direct HTTP operations (for alias/reindex APIs)
    base_url: String,
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
            base_url: endpoint,
        })
    }

    /// Generate index name from object type (e.g., "ontology_user" or "ontology_document")
    fn index_name(&self, object_type: &str) -> String {
        format!("{}_{}", self.index_prefix, object_type)
    }
    
    /// Generate versioned index name (e.g., "ontology_user_v1" or "ontology_user_v2")
    fn versioned_index_name(&self, object_type: &str, version: u64) -> String {
        format!("{}_{}_v{}", self.index_prefix, object_type, version)
    }
    
    /// Generate alias name (e.g., "ontology_user" - this is what clients query)
    fn alias_name(&self, object_type: &str) -> String {
        format!("{}_{}", self.index_prefix, object_type)
    }
    
    /// Create an index alias pointing to a versioned index
    pub async fn create_alias(
        &self,
        object_type: &str,
        version: u64,
    ) -> Result<(), StoreError> {
        let alias = self.alias_name(object_type);
        let index = self.versioned_index_name(object_type, version);
        
        // Create alias pointing to the versioned index
        let alias_body = json!({
            "actions": [{
                "add": {
                    "index": index,
                    "alias": alias
                }
            }]
        });
        
        // Use HTTP client directly for alias operations
        // Note: The elasticsearch crate's alias API may vary by version
        // This implementation uses direct HTTP calls for maximum compatibility
        let url = format!("{}/_aliases", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&alias_body)
            .send()
            .await
            .map_err(|e| StoreError::WriteError(format!("Failed to create alias: {}", e)))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::WriteError(format!(
                "Failed to create alias: {} - {}",
                status.as_u16(),
                error_body
            )));
        }
        
        Ok(())
    }
    
    /// Atomically swap alias from one version to another (blue/green deployment)
    pub async fn swap_alias(
        &self,
        object_type: &str,
        from_version: u64,
        to_version: u64,
    ) -> Result<(), StoreError> {
        let alias = self.alias_name(object_type);
        let old_index = self.versioned_index_name(object_type, from_version);
        let new_index = self.versioned_index_name(object_type, to_version);
        
        // Atomic swap: remove old alias, add new alias in a single operation
        let alias_body = json!({
            "actions": [
                {
                    "remove": {
                        "index": old_index,
                        "alias": alias
                    }
                },
                {
                    "add": {
                        "index": new_index,
                        "alias": alias
                    }
                }
            ]
        });
        
        // Use HTTP client directly for atomic alias swap
        let url = format!("{}/_aliases", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&alias_body)
            .send()
            .await
            .map_err(|e| StoreError::WriteError(format!("Failed to swap alias: {}", e)))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::WriteError(format!(
                "Failed to swap alias: {} - {}",
                status.as_u16(),
                error_body
            )));
        }
        
        Ok(())
    }
    
    /// Reindex data from one version to another
    pub async fn reindex(
        &self,
        object_type: &str,
        from_version: u64,
        to_version: u64,
    ) -> Result<(), StoreError> {
        let source_index = self.versioned_index_name(object_type, from_version);
        let dest_index = self.versioned_index_name(object_type, to_version);
        
        // Use Elasticsearch reindex API
        let reindex_body = json!({
            "source": {
                "index": source_index
            },
            "dest": {
                "index": dest_index
            }
        });
        
        // Use HTTP client directly for reindex operation
        let url = format!("{}/_reindex", self.base_url);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&reindex_body)
            .send()
            .await
            .map_err(|e| StoreError::WriteError(format!("Failed to reindex: {}", e)))?;
        
        let status = response.status();
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::WriteError(format!(
                "Failed to reindex: {} - {}",
                status.as_u16(),
                error_body
            )));
        }
        
        // Check for errors in reindex response
        let response_body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| StoreError::WriteError(format!("Failed to parse reindex response: {}", e)))?;
        
        if let Some(failures) = response_body.get("failures").and_then(|f| f.as_array()) {
            if !failures.is_empty() {
                return Err(StoreError::WriteError(format!(
                    "Reindex had {} failures",
                    failures.len()
                )));
            }
        }
        
        Ok(())
    }
    
    /// Get the current version that an alias points to
    pub async fn get_alias_version(
        &self,
        object_type: &str,
    ) -> Result<Option<u64>, StoreError> {
        let alias = self.alias_name(object_type);
        
        // Use HTTP client directly to get alias information
        let url = format!("{}/{}/_alias", self.base_url, alias);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| StoreError::ReadError(format!("Failed to get alias: {}", e)))?;
        
        let status = response.status();
        if !status.is_success() {
            if status == 404 {
                return Ok(None); // Alias doesn't exist
            }
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::ReadError(format!(
                "Failed to get alias: {} - {}",
                status.as_u16(),
                error_body
            )));
        }
        
        let response_body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| StoreError::ReadError(format!("Failed to parse alias response: {}", e)))?;
        
        // Extract version from index name (e.g., "ontology_user_v2" -> 2)
        for (index_name, _) in response_body.as_object().unwrap_or(&serde_json::Map::new()) {
            if let Some(version_str) = index_name.strip_suffix(&format!("_{}", object_type)) {
                if let Some(version) = version_str.strip_prefix(&format!("{}_v", self.index_prefix)) {
                    if let Ok(version_num) = version.parse::<u64>() {
                        return Ok(Some(version_num));
                    }
                }
            }
            // Alternative pattern: check if index name ends with _v{number}
            if let Some(v_part) = index_name.strip_prefix(&format!("{}_{}_v", self.index_prefix, object_type)) {
                if let Ok(version_num) = v_part.parse::<u64>() {
                    return Ok(Some(version_num));
                }
            }
        }
        
        Ok(None)
    }
    
    /// Delete an old versioned index (after migration is complete)
    pub async fn delete_versioned_index(
        &self,
        object_type: &str,
        version: u64,
    ) -> Result<(), StoreError> {
        let index = self.versioned_index_name(object_type, version);
        
        let response = self.client
            .indices()
            .delete(elasticsearch::indices::IndicesDeleteParts::Index(&[&index]))
            .send()
            .await
            .map_err(|e| StoreError::WriteError(format!("Failed to delete index: {}", e)))?;
        
        let status_code = response.status_code();
        if !status_code.is_success() {
            if status_code == 404 {
                return Ok(()); // Index doesn't exist, that's fine
            }
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::WriteError(format!(
                "Failed to delete index: {} - {}",
                status_code.as_u16(),
                error_body
            )));
        }
        
        Ok(())
    }
    
    /// Build Elasticsearch query body from filters (reusable for search and count)
    fn build_query_body(&self, filters: Option<&[Filter]>) -> Result<JsonValue, StoreError> {
        let mut query_body = serde_json::Map::new();
        
        if let Some(filter_slice) = filters {
            if !filter_slice.is_empty() {
                let mut must_clauses = Vec::new();
                let mut must_not_clauses = Vec::new();
                
                for filter in filter_slice {
                    let clause = self.build_query_clause(filter)?;
                    match filter.operator {
                        FilterOperator::NotEquals | FilterOperator::NotIn => {
                            must_not_clauses.push(clause);
                        }
                        _ => {
                            must_clauses.push(clause);
                        }
                    }
                }
                
                let mut bool_query = serde_json::Map::new();
                if !must_clauses.is_empty() {
                    bool_query.insert("must".to_string(), JsonValue::Array(must_clauses));
                }
                if !must_not_clauses.is_empty() {
                    bool_query.insert("must_not".to_string(), JsonValue::Array(must_not_clauses));
                }
                let mut query_obj = serde_json::Map::new();
                query_obj.insert("bool".to_string(), JsonValue::Object(bool_query));
                query_body.insert("query".to_string(), JsonValue::Object(query_obj));
            } else {
                // Match all if no filters
                let mut match_all = serde_json::Map::new();
                query_body.insert("query".to_string(), JsonValue::Object(match_all));
            }
        } else {
            // Match all if no filters provided
            let mut match_all = serde_json::Map::new();
            query_body.insert("query".to_string(), JsonValue::Object(match_all));
        }
        
        Ok(JsonValue::Object(query_body))
    }
    
    /// Build an Elasticsearch query clause from a Filter
    fn build_query_clause(&self, filter: &Filter) -> Result<JsonValue, StoreError> {
        let mut clause = serde_json::Map::new();
        
        match filter.operator {
            FilterOperator::Equals => {
                let term_value = self.property_value_to_es_value(&filter.value)?;
                let mut term_obj = serde_json::Map::new();
                term_obj.insert(filter.property.clone(), term_value);
                clause.insert("term".to_string(), JsonValue::Object(term_obj));
            }
            FilterOperator::Contains | FilterOperator::StartsWith | FilterOperator::EndsWith => {
                let query_string = match &filter.value {
                    ontology_engine::PropertyValue::String(s) => s.clone(),
                    _ => return Err(StoreError::Query("Contains/StartsWith/EndsWith requires string value".to_string())),
                };
                
                let mut match_obj = serde_json::Map::new();
                let pattern = match filter.operator {
                    FilterOperator::Contains => format!("*{}*", query_string),
                    FilterOperator::StartsWith => format!("{}*", query_string),
                    FilterOperator::EndsWith => format!("*{}", query_string),
                    _ => unreachable!(),
                };
                match_obj.insert(filter.property.clone(), JsonValue::String(pattern));
                clause.insert("wildcard".to_string(), JsonValue::Object(match_obj));
            }
            FilterOperator::GreaterThan => {
                let mut range_obj = serde_json::Map::new();
                let mut gt_obj = serde_json::Map::new();
                gt_obj.insert("gt".to_string(), self.property_value_to_es_value(&filter.value)?);
                range_obj.insert(filter.property.clone(), JsonValue::Object(gt_obj));
                clause.insert("range".to_string(), JsonValue::Object(range_obj));
            }
            FilterOperator::LessThan => {
                let mut range_obj = serde_json::Map::new();
                let mut lt_obj = serde_json::Map::new();
                lt_obj.insert("lt".to_string(), self.property_value_to_es_value(&filter.value)?);
                range_obj.insert(filter.property.clone(), JsonValue::Object(lt_obj));
                clause.insert("range".to_string(), JsonValue::Object(range_obj));
            }
            FilterOperator::GreaterThanOrEqual => {
                let mut range_obj = serde_json::Map::new();
                let mut gte_obj = serde_json::Map::new();
                gte_obj.insert("gte".to_string(), self.property_value_to_es_value(&filter.value)?);
                range_obj.insert(filter.property.clone(), JsonValue::Object(gte_obj));
                clause.insert("range".to_string(), JsonValue::Object(range_obj));
            }
            FilterOperator::LessThanOrEqual => {
                let mut range_obj = serde_json::Map::new();
                let mut lte_obj = serde_json::Map::new();
                lte_obj.insert("lte".to_string(), self.property_value_to_es_value(&filter.value)?);
                range_obj.insert(filter.property.clone(), JsonValue::Object(lte_obj));
                clause.insert("range".to_string(), JsonValue::Object(range_obj));
            }
            FilterOperator::In => {
                let values = match &filter.value {
                    ontology_engine::PropertyValue::Array(arr) => {
                        arr.iter().map(|v| self.property_value_to_es_value(v)).collect::<Result<Vec<_>, _>>()?
                    }
                    _ => return Err(StoreError::Query("In operator requires array value".to_string())),
                };
                let mut terms_obj = serde_json::Map::new();
                terms_obj.insert(filter.property.clone(), JsonValue::Array(values));
                clause.insert("terms".to_string(), JsonValue::Object(terms_obj));
            }
            FilterOperator::NotIn => {
                let values = match &filter.value {
                    ontology_engine::PropertyValue::Array(arr) => {
                        arr.iter().map(|v| self.property_value_to_es_value(v)).collect::<Result<Vec<_>, _>>()?
                    }
                    _ => return Err(StoreError::Query("NotIn operator requires array value".to_string())),
                };
                let mut terms_obj = serde_json::Map::new();
                terms_obj.insert(filter.property.clone(), JsonValue::Array(values));
                clause.insert("terms".to_string(), JsonValue::Object(terms_obj));
            }
            _ => {
                return Err(StoreError::Query(format!(
                    "Filter operator {:?} not yet implemented for Elasticsearch",
                    filter.operator
                )));
            }
        }
        
        Ok(JsonValue::Object(clause))
    }
    
    /// Convert PropertyValue to Elasticsearch JSON value
    fn property_value_to_es_value(&self, value: &ontology_engine::PropertyValue) -> Result<JsonValue, StoreError> {
        match value {
            ontology_engine::PropertyValue::String(s) => Ok(JsonValue::String(s.clone())),
            ontology_engine::PropertyValue::Integer(i) => Ok(JsonValue::Number((*i).into())),
            ontology_engine::PropertyValue::Double(d) => {
                serde_json::Number::from_f64(*d)
                    .map(JsonValue::Number)
                    .ok_or_else(|| StoreError::Query("Invalid double value".to_string()))
            }
            ontology_engine::PropertyValue::Boolean(b) => Ok(JsonValue::Bool(*b)),
            _ => Err(StoreError::Query(format!("Unsupported PropertyValue type for Elasticsearch: {:?}", value))),
        }
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
        let index_name = self.index_name(object_type);
        
        // Build query body using helper method
        let query_body = self.build_query_body(Some(&query.filters))?;
        
        // Extract the query body map for adding sort/pagination
        let mut query_body_map = if let JsonValue::Object(map) = query_body {
            map
        } else {
            return Err(StoreError::Query("Invalid query body structure".to_string()));
        };
        
        // Add sorting
        if let Some(sort) = &query.sort {
            let mut sort_obj = serde_json::Map::new();
            sort_obj.insert(sort.property.clone(), JsonValue::String(
                if sort.ascending { "asc" } else { "desc" }.to_string()
            ));
            query_body_map.insert("sort".to_string(), JsonValue::Array(vec![JsonValue::Object(sort_obj)]));
        }
        
        // Add pagination
        if let Some(size) = query.limit {
            query_body_map.insert("size".to_string(), JsonValue::Number(size.into()));
        }
        if let Some(from) = query.offset {
            query_body_map.insert("from".to_string(), JsonValue::Number(from.into()));
        }
        
        let response = self.client
            .search(SearchParts::Index(&[&index_name]))
            .body(JsonValue::Object(query_body_map))
            .send()
            .await
            .map_err(|e| StoreError::Query(format!("Elasticsearch search failed: {}", e)))?;
        
        let status_code = response.status_code();
        if !status_code.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::Query(format!(
                "Elasticsearch returned error {}: {}",
                status_code.as_u16(),
                error_body
            )));
        }
        
        // Parse response
        let response_body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| StoreError::Query(format!("Failed to parse response: {}", e)))?;
        
        // Extract hits
        let empty_vec = Vec::new();
        let hits = response_body.get("hits")
            .and_then(|h| h.get("hits"))
            .and_then(|h| h.as_array())
            .unwrap_or(&empty_vec);
        
        let mut results = Vec::new();
        for hit in hits {
            let source = hit.get("_source")
                .ok_or_else(|| StoreError::Query("Missing _source in hit".to_string()))?;
            
            let id = hit.get("_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            // Convert JSON back to PropertyMap
            let mut properties = PropertyMap::new();
            if let Some(obj) = source.as_object() {
                for (key, value) in obj {
                    // Skip metadata fields
                    if key == "object_id" || key == "object_type" || key == "indexed_at" {
                        continue;
                    }
                    
                    let prop_value: ontology_engine::PropertyValue = serde_json::from_value(value.clone())
                        .map_err(|e| StoreError::Query(format!("Failed to deserialize property '{}': {}", key, e)))?;
                    properties.insert(key.clone(), prop_value);
                }
            }
            
            let indexed_at = source.get("indexed_at")
                .and_then(|v| v.as_str())
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(chrono::Utc::now);
            
            results.push(IndexedObject {
                object_type: object_type.to_string(),
                object_id: id.to_string(),
                properties,
                indexed_at,
                source_last_modified: None,
                refresh_frequency: None,
                next_refresh: None,
                refresh_status: RefreshStatus::UpToDate,
            });
        }
        
        Ok(results)
    }
    
    async fn get_object(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<Option<IndexedObject>, StoreError> {
        let index_name = self.index_name(object_type);
        
        let response = self.client
            .get(GetParts::IndexId(&index_name, object_id))
            .send()
            .await
            .map_err(|e| StoreError::ReadError(format!("Elasticsearch get failed: {}", e)))?;
        
        let status_code = response.status_code();
        if !status_code.is_success() {
            if status_code == 404 {
                return Ok(None);
            }
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::ReadError(format!(
                "Elasticsearch returned error {}: {}",
                status_code.as_u16(),
                error_body
            )));
        }
        
        // Parse response
        let response_body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| StoreError::ReadError(format!("Failed to parse response: {}", e)))?;
        
        // Extract source document
        let source = response_body.get("_source")
            .ok_or_else(|| StoreError::ReadError("Missing _source in response".to_string()))?;
        
        // Convert JSON back to PropertyMap
        let mut properties = PropertyMap::new();
        if let Some(obj) = source.as_object() {
            for (key, value) in obj {
                // Skip metadata fields
                if key == "object_id" || key == "object_type" || key == "indexed_at" {
                    continue;
                }
                
                let prop_value: ontology_engine::PropertyValue = serde_json::from_value(value.clone())
                    .map_err(|e| StoreError::ReadError(format!("Failed to deserialize property '{}': {}", key, e)))?;
                properties.insert(key.clone(), prop_value);
            }
        }
        
        // Extract indexed_at from source or use current time
        let indexed_at = source.get("indexed_at")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or_else(chrono::Utc::now);
        
        Ok(Some(IndexedObject {
            object_type: object_type.to_string(),
            object_id: object_id.to_string(),
            properties,
            indexed_at,
            source_last_modified: None,
            refresh_frequency: None,
            next_refresh: None,
            refresh_status: RefreshStatus::UpToDate,
        }))
    }
    
    async fn bulk_index(
        &self,
        objects: Vec<IndexedObject>,
    ) -> Result<(), StoreError> {
        if objects.is_empty() {
            return Ok(());
        }
        
        // Group objects by type for efficient bulk indexing
        let mut by_type: std::collections::HashMap<String, Vec<(String, JsonValue)>> = std::collections::HashMap::new();
        
        for obj in objects {
            let mut properties = obj.properties.clone();
            // Add metadata to properties for indexing
            properties.insert("object_id".to_string(), ontology_engine::PropertyValue::String(obj.object_id.clone()));
            properties.insert("object_type".to_string(), ontology_engine::PropertyValue::String(obj.object_type.clone()));
            properties.insert("indexed_at".to_string(), ontology_engine::PropertyValue::DateTime(obj.indexed_at.to_rfc3339()));
            
            // Serialize PropertyMap to JSON
            let mut json_map = serde_json::Map::new();
            for (key, value) in properties.iter() {
                let json_value = serde_json::to_value(value)
                    .map_err(|e| StoreError::Serialization(format!("Failed to serialize property '{}': {}", key, e)))?;
                json_map.insert(key.clone(), json_value);
            }
            let json_body = JsonValue::Object(json_map);
            
            by_type.entry(obj.object_type.clone())
                .or_insert_with(Vec::new)
                .push((obj.object_id, json_body));
        }
        
        // Perform bulk operations per object type
        // Note: Proper bulk API with NDJSON requires determining the correct body type
        // for elasticsearch crate version 8.19.0-alpha.1. The current implementation
        // uses individual operations which is still more efficient than before (grouped by type).
        for (object_type, items) in by_type {
            for (id, doc) in items {
                let mut properties = PropertyMap::new();
                if let JsonValue::Object(map) = doc {
                    for (key, value) in map {
                        // Skip metadata fields - they're added automatically in index_object
                        if key == "object_id" || key == "object_type" || key == "indexed_at" {
                            continue;
                        }
                        let prop_value: ontology_engine::PropertyValue = serde_json::from_value(value)
                            .map_err(|e| StoreError::Serialization(format!("Failed to deserialize property '{}': {}", key, e)))?;
                        properties.insert(key, prop_value);
                    }
                }
                
                self.index_object(&object_type, &id, &properties).await?;
            }
            // Note: Proper bulk API with NDJSON requires determining the correct body type
            // for elasticsearch crate version 8.19.0-alpha.1. The current implementation
            // uses individual operations which is still more efficient than before (grouped by type).
        }
        
        Ok(())
    }
    
    async fn count_objects(
        &self,
        object_type: &str,
        filters: Option<&[Filter]>,
    ) -> Result<u64, StoreError> {
        let index_name = self.index_name(object_type);
        let query_body = self.build_query_body(filters)?;
        
        let response = self.client
            .count(CountParts::Index(&[&index_name]))
            .body(query_body)
            .send()
            .await
            .map_err(|e| StoreError::Query(format!("Elasticsearch count failed: {}", e)))?;
        
        let status_code = response.status_code();
        if !status_code.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::Query(format!(
                "Elasticsearch returned error {}: {}",
                status_code.as_u16(),
                error_body
            )));
        }
        
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| StoreError::Query(format!("Failed to parse count response: {}", e)))?;
        
        Ok(json["count"].as_u64().unwrap_or(0))
    }
    
    async fn delete_object(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<(), StoreError> {
        let index_name = self.index_name(object_type);
        
        let response = self.client
            .delete(DeleteParts::IndexId(&index_name, object_id))
            .send()
            .await
            .map_err(|e| StoreError::WriteError(format!("Elasticsearch delete failed: {}", e)))?;
        
        let status_code = response.status_code();
        if !status_code.is_success() {
            if status_code == 404 {
                // Document not found - this is OK for delete operations
                return Ok(());
            }
            let error_body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            return Err(StoreError::WriteError(format!(
                "Elasticsearch returned error {}: {}",
                status_code.as_u16(),
                error_body
            )));
        }
        
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
    
    /// Build a Dgraph filter expression from a Filter
    /// Filters are applied to facets (edge properties) using @filter directive
    /// This is a helper method for traverse_with_filters
    fn build_dgraph_filter(&self, filter: &Filter) -> Result<String, StoreError> {
        let value_str = match &filter.value {
            ontology_engine::PropertyValue::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            ontology_engine::PropertyValue::Integer(i) => i.to_string(),
            ontology_engine::PropertyValue::Double(d) => d.to_string(),
            ontology_engine::PropertyValue::Boolean(b) => b.to_string(),
            _ => return Err(StoreError::Query(format!(
                "Unsupported PropertyValue type for Dgraph filter: {:?}",
                filter.value
            ))),
        };
        
        // Dgraph uses facet syntax: @facets(property_name operator value)
        // For filtering on facets during traversal, we use @filter(eq(facet_name, value))
        let filter_expr = match filter.operator {
            FilterOperator::Equals => {
                format!("eq({}, {})", filter.property, value_str)
            }
            FilterOperator::NotEquals => {
                format!("not eq({}, {})", filter.property, value_str)
            }
            FilterOperator::GreaterThan => {
                format!("gt({}, {})", filter.property, value_str)
            }
            FilterOperator::LessThan => {
                format!("lt({}, {})", filter.property, value_str)
            }
            FilterOperator::GreaterThanOrEqual => {
                format!("ge({}, {})", filter.property, value_str)
            }
            FilterOperator::LessThanOrEqual => {
                format!("le({}, {})", filter.property, value_str)
            }
            FilterOperator::In => {
                // For In, we need to build an OR expression
                if let ontology_engine::PropertyValue::Array(arr) = &filter.value {
                    let mut value_strs = Vec::new();
                    for v in arr {
                        let value_str = match v {
                            ontology_engine::PropertyValue::String(s) => format!("\"{}\"", s.replace('"', "\\\"")),
                            ontology_engine::PropertyValue::Integer(i) => i.to_string(),
                            ontology_engine::PropertyValue::Double(d) => d.to_string(),
                            ontology_engine::PropertyValue::Boolean(b) => b.to_string(),
                            _ => return Err(StoreError::Query("Unsupported array element type for Dgraph In filter".to_string())),
                        };
                        value_strs.push(value_str);
                    }
                    if value_strs.is_empty() {
                        return Err(StoreError::Query("In operator requires non-empty array".to_string()));
                    }
                    // Build OR expression for all values
                    let or_parts: Vec<String> = value_strs.iter()
                        .map(|vs| format!("eq({}, {})", filter.property, vs))
                        .collect();
                    format!("({})", or_parts.join(" OR "))
                } else {
                    return Err(StoreError::Query("In operator requires array value".to_string()));
                }
            }
            _ => {
                return Err(StoreError::Query(format!(
                    "Filter operator {:?} not yet implemented for Dgraph",
                    filter.operator
                )));
            }
        };
        
        Ok(filter_expr)
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
        let start_uid = self.get_or_create_uid(start_id).await?;
        let mut all_target_ids = Vec::new();
        
        // Build filter string for Dgraph @filter directive
        let filter_str = if !link_filters.is_empty() {
            let mut filter_parts = Vec::new();
            for filter in link_filters {
                let filter_expr = self.build_dgraph_filter(filter)?;
                filter_parts.push(filter_expr);
            }
            if !filter_parts.is_empty() {
                format!(" @filter({})", filter_parts.join(" AND "))
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        // Build traversal query with filters
        for link_type_id in link_type_ids {
            let predicate = link_type_id.replace('-', "_").replace('.', "_");
            
            let mut query_parts = Vec::new();
            query_parts.push(format!("query {{\n  node(func: uid({})) {{", start_uid));
            
            // Build nested traversal with filters
            for hop in 0..max_hops {
                let indent = "  ".repeat(hop + 1);
                if hop == 0 {
                    // Apply filter on first hop
                    query_parts.push(format!("{}{} {} {{", indent, predicate, filter_str));
                } else {
                    query_parts.push(format!("{}~{} {{", indent, predicate));
                }
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
            query_parts.push("  }".to_string());
            query_parts.push("}".to_string());
            
            let query = query_parts.join("\n");
            
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
    
    async fn traverse_with_aggregation(
        &self,
        start_id: &str,
        link_type_ids: &[String],
        max_hops: usize,
        aggregation: &TraversalAggregation,
    ) -> Result<TraversalAggregationResult, StoreError> {
        let start_uid = self.get_or_create_uid(start_id).await?;
        
        // Build aggregation query using Dgraph's aggregation functions
        // Dgraph supports: count(uid), sum(predicate), avg(predicate), min(predicate), max(predicate)
        let predicate = if !link_type_ids.is_empty() {
            link_type_ids[0].replace('-', "_").replace('.', "_")
        } else {
            return Err(StoreError::Query("At least one link type is required for aggregation".to_string()));
        };
        
        // Build aggregation expression based on operation
        // Note: Dgraph supports limited aggregations, so some will need post-processing
        let agg_expr = match &aggregation.operation {
            Aggregation::Count => "count(uid)".to_string(),
            Aggregation::Sum(prop) => format!("sum({})", prop),
            Aggregation::Avg(prop) => format!("avg({})", prop),
            Aggregation::Min(prop) => format!("min({})", prop),
            Aggregation::Max(prop) => format!("max({})", prop),
            Aggregation::Median(_) | Aggregation::StdDev(_) | Aggregation::Variance(_) |
            Aggregation::Percentile(_, _) | Aggregation::DistinctCount(_) |
            Aggregation::TopN(_, _) | Aggregation::BottomN(_, _) => {
                return Err(StoreError::Query(
                    format!("Aggregation {:?} not supported in graph traversal. Use columnar store instead.", aggregation.operation)
                ));
            }
        };
        
        // Build filter string if object filters are provided
        let filter_str = if !aggregation.object_filters.is_empty() {
            let mut filter_parts = Vec::new();
            for filter in &aggregation.object_filters {
                let filter_expr = self.build_dgraph_filter(filter)?;
                filter_parts.push(filter_expr);
            }
            if !filter_parts.is_empty() {
                format!(" @filter({})", filter_parts.join(" AND "))
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        // Build query with aggregation
        // For multi-hop traversal with aggregation, we aggregate over all reached nodes
        let mut query_parts = Vec::new();
        query_parts.push(format!("query {{\n  node(func: uid({})) {{", start_uid));
        
        // Build traversal path
        for hop in 0..max_hops {
            let indent = "  ".repeat(hop + 1);
            if hop == 0 {
                query_parts.push(format!("{}{} {} {{", indent, predicate, filter_str));
            } else {
                query_parts.push(format!("{}~{} {{", indent, predicate));
            }
            
            if hop == max_hops - 1 {
                // At the final hop, apply aggregation
                query_parts.push(format!("{}  {}", indent, agg_expr));
            } else {
                query_parts.push(format!("{}  uid", indent));
                query_parts.push(format!("{}  xid", indent));
            }
            
            if hop < max_hops - 1 {
                query_parts.push(format!("{}  ~{} {{", indent, predicate));
            }
        }
        
        // Close all brackets
        for hop in (0..max_hops).rev() {
            let indent = "  ".repeat(hop + 1);
            query_parts.push(format!("{}}}", indent));
        }
        query_parts.push("  }".to_string());
        query_parts.push("}".to_string());
        
        let query = query_parts.join("\n");
        
        let mut txn = self.client.new_read_only_txn();
        let response = txn.query(query).await
            .map_err(|e| StoreError::ReadError(format!("Query error: {}", e)))?;
        
        let json: serde_json::Value = serde_json::from_slice(&response.json)
            .map_err(|e| StoreError::ReadError(format!("Parse error: {}", e)))?;
        
        // Extract aggregation result
        // The result structure depends on the aggregation type
        let (value, count) = if let Some(node_arr) = json.get("node").and_then(|n| n.as_array()) {
            if let Some(node) = node_arr.first() {
                // Navigate through the traversal path to find the aggregation result
                // The aggregation result is at the deepest level
                let mut current = node;
                for _ in 0..max_hops {
                    if let Some(pred_obj) = current.get(&predicate).and_then(|p| p.as_array()).and_then(|a| a.first()) {
                        current = pred_obj;
                    } else {
                        break;
                    }
                }
                
                // Extract aggregation value based on operation type
                let (value, count) = match &aggregation.operation {
                    Aggregation::Count => {
                        // Count returns the number of UIDs
                        if let Some(arr) = current.as_array() {
                            (arr.len() as i64, arr.len())
                        } else if let Some(count_val) = current.get("count").and_then(|v| v.as_u64()) {
                            (count_val as i64, count_val as usize)
                        } else {
                            (0, 0)
                        }
                    }
                    Aggregation::Sum(prop) => {
                        let agg_key = format!("sum_{}", prop);
                        if let Some(num_val) = current.get(&agg_key).and_then(|v| {
                            v.as_f64().or_else(|| v.as_i64().map(|i| i as f64))
                        }) {
                            (num_val as i64, 1)
                        } else {
                            (0, 0)
                        }
                    }
                    Aggregation::Avg(prop) => {
                        let agg_key = format!("avg_{}", prop);
                        if let Some(num_val) = current.get(&agg_key).and_then(|v| {
                            v.as_f64().or_else(|| v.as_i64().map(|i| i as f64))
                        }) {
                            (num_val as i64, 1)
                        } else {
                            (0, 0)
                        }
                    }
                    Aggregation::Min(prop) => {
                        let agg_key = format!("min_{}", prop);
                        if let Some(num_val) = current.get(&agg_key).and_then(|v| {
                            v.as_f64().or_else(|| v.as_i64().map(|i| i as f64))
                        }) {
                            (num_val as i64, 1)
                        } else {
                            (0, 0)
                        }
                    }
                    Aggregation::Max(prop) => {
                        let agg_key = format!("max_{}", prop);
                        if let Some(num_val) = current.get(&agg_key).and_then(|v| {
                            v.as_f64().or_else(|| v.as_i64().map(|i| i as f64))
                        }) {
                            (num_val as i64, 1)
                        } else {
                            (0, 0)
                        }
                    }
                    Aggregation::Median(_) | Aggregation::StdDev(_) | Aggregation::Variance(_) |
                    Aggregation::Percentile(_, _) | Aggregation::DistinctCount(_) |
                    Aggregation::TopN(_, _) | Aggregation::BottomN(_, _) => {
                        // These aggregations are not supported in graph traversal
                        (0, 0)
                    }
                };
                
                (value, count)
            } else {
                (0, 0)
            }
        } else {
            (0, 0)
        };
        
        // Convert to PropertyValue
        // For aggregations, we need to extract the actual numeric value from the query result
        // Dgraph returns aggregation results differently - we need to handle the actual response structure
        let prop_value = match &aggregation.operation {
            Aggregation::Count => ontology_engine::PropertyValue::Integer(value),
            Aggregation::Sum(_) | Aggregation::Min(_) | Aggregation::Max(_) => {
                ontology_engine::PropertyValue::Integer(value)
            }
            Aggregation::Avg(_) => {
                // Average might be a decimal, but for now we'll use Integer
                // In a full implementation, we'd extract the actual double value from Dgraph response
                ontology_engine::PropertyValue::Integer(value)
            }
            Aggregation::Median(_) | Aggregation::StdDev(_) | Aggregation::Variance(_) |
            Aggregation::Percentile(_, _) | Aggregation::DistinctCount(_) |
            Aggregation::TopN(_, _) | Aggregation::BottomN(_, _) => {
                // These aggregations are not supported in graph traversal
                return Err(StoreError::Query(
                    format!("Aggregation {:?} not supported in graph traversal", aggregation.operation)
                ));
            }
        };
        
        Ok(TraversalAggregationResult {
            value: prop_value,
            count,
        })
    }
    
    async fn compute_centrality(
        &self,
        _object_type: &str,
        _metric: CentralityMetric,
    ) -> Result<HashMap<String, f64>, StoreError> {
        // Placeholder implementation - would require graph algorithms
        Err(StoreError::Query("Centrality computation not yet implemented".to_string()))
    }
    
    async fn detect_communities(
        &self,
        _object_type: &str,
        _algorithm: CommunityAlgorithm,
    ) -> Result<HashMap<String, usize>, StoreError> {
        // Placeholder implementation - would require community detection algorithms
        Err(StoreError::Query("Community detection not yet implemented".to_string()))
    }
    
    async fn shortest_path(
        &self,
        _source_id: &str,
        _target_id: &str,
        _link_types: &[String],
    ) -> Result<Vec<String>, StoreError> {
        // Placeholder implementation - would require shortest path algorithm
        Err(StoreError::Query("Shortest path computation not yet implemented".to_string()))
    }
    
    async fn graph_metrics(
        &self,
        _object_type: &str,
    ) -> Result<GraphMetrics, StoreError> {
        // Placeholder implementation
        Ok(GraphMetrics {
            node_count: 0,
            edge_count: 0,
            density: 0.0,
            average_clustering_coefficient: 0.0,
            average_degree: 0.0,
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

// Parquet store implementation using Polars
pub struct ParquetStore {
    base_path: String,
}

impl ParquetStore {
    pub fn new(base_path: String) -> Self {
        Self { base_path }
    }

    fn file_path(&self, object_type: &str) -> String {
        format!("{}/{}.parquet", self.base_path, object_type)
    }

    /// Convert PropertyValue to serde_json::Value
    fn property_value_to_json(value: &ontology_engine::PropertyValue) -> JsonValue {
        match value {
            ontology_engine::PropertyValue::String(s) => JsonValue::String(s.clone()),
            ontology_engine::PropertyValue::Integer(i) => JsonValue::Number((*i).into()),
            ontology_engine::PropertyValue::Double(d) => {
                serde_json::Number::from_f64(*d)
                    .map(JsonValue::Number)
                    .unwrap_or(JsonValue::Null)
            }
            ontology_engine::PropertyValue::Boolean(b) => JsonValue::Bool(*b),
            ontology_engine::PropertyValue::Date(d) => JsonValue::String(d.clone()),
            ontology_engine::PropertyValue::DateTime(dt) => JsonValue::String(dt.clone()),
            ontology_engine::PropertyValue::ObjectReference(id) => JsonValue::String(id.clone()),
            ontology_engine::PropertyValue::GeoJSON(gj) => {
                // Try to parse as JSON, fallback to string
                serde_json::from_str(gj).unwrap_or_else(|_| JsonValue::String(gj.clone()))
            }
            ontology_engine::PropertyValue::Array(arr) => {
                JsonValue::Array(arr.iter().map(Self::property_value_to_json).collect())
            }
            ontology_engine::PropertyValue::Map(map) => {
                let mut json_map = serde_json::Map::new();
                for (key, value) in map {
                    json_map.insert(key.clone(), Self::property_value_to_json(value));
                }
                JsonValue::Object(json_map)
            }
            ontology_engine::PropertyValue::Object(obj) => {
                let mut json_obj = serde_json::Map::new();
                for (key, value) in obj {
                    json_obj.insert(key.clone(), Self::property_value_to_json(value));
                }
                JsonValue::Object(json_obj)
            }
            ontology_engine::PropertyValue::Null => JsonValue::Null,
        }
    }

    /// Convert IndexedObject to JSON for Polars ingestion
    fn indexed_object_to_json(obj: &IndexedObject) -> Result<JsonValue, StoreError> {
        let mut json_obj = serde_json::Map::new();
        
        // Add metadata fields
        json_obj.insert("object_id".to_string(), JsonValue::String(obj.object_id.clone()));
        json_obj.insert("object_type".to_string(), JsonValue::String(obj.object_type.clone()));
        json_obj.insert(
            "indexed_at".to_string(),
            JsonValue::String(obj.indexed_at.to_rfc3339()),
        );
        
        // Convert properties
        for (key, value) in obj.properties.iter() {
            json_obj.insert(key.clone(), Self::property_value_to_json(value));
        }
        
        Ok(JsonValue::Object(json_obj))
    }
}

#[async_trait]
impl ColumnarStore for ParquetStore {
    async fn write_batch(
        &self,
        object_type: &str,
        objects: Vec<IndexedObject>,
    ) -> Result<(), StoreError> {
        if objects.is_empty() {
            return Ok(());
        }

        // Ensure base directory exists
        std::fs::create_dir_all(&self.base_path)
            .map_err(|e| StoreError::WriteError(format!("Failed to create directory: {}", e)))?;

        // 1. Convert IndexedObjects to JSON
        let mut json_objects = Vec::new();
        for obj in &objects {
            json_objects.push(Self::indexed_object_to_json(obj)?);
        }

        // 2. Serialize as a JSON array (polars JsonReader expects an array, not NDJSON)
        let json_array = JsonValue::Array(json_objects);
        let json_buffer = serde_json::to_vec(&json_array)
            .map_err(|e| StoreError::WriteError(format!("Serialization error: {}", e)))?;

        // 3. Load into Polars DataFrame
        let cursor = Cursor::new(json_buffer);
        let mut df = JsonReader::new(cursor)
            .infer_schema_len(Some(1000)) // Check up to 1000 rows to determine column types
            .finish()
            .map_err(|e| StoreError::WriteError(format!("DataFrame creation error: {}", e)))?;

        // 4. Write to Parquet
        let path = self.file_path(object_type);
        let file = File::create(&path)
            .map_err(|e| StoreError::WriteError(format!("File creation error: {}", e)))?;

        ParquetWriter::new(file)
            .finish(&mut df)
            .map_err(|e| StoreError::WriteError(format!("Parquet write error: {}", e)))?;

        Ok(())
    }
    
    async fn query_analytics(
        &self,
        object_type: &str,
        query: &AnalyticsQuery,
    ) -> Result<AnalyticsResult, StoreError> {
        let path = self.file_path(object_type);
        if !Path::new(&path).exists() {
            return Err(StoreError::ReadError(format!("File not found: {}", path)));
        }

        // 1. Lazy Scan (doesn't load whole file into memory)
        let lf = LazyFrame::scan_parquet(&path, ScanArgsParquet::default())
            .map_err(|e| StoreError::ReadError(format!("Scan error: {}", e)))?;

        // 2. Apply filters if any
        let mut lf = lf;
        for filter in &query.filters {
            // Basic filter support - can be extended
            match filter.operator {
                FilterOperator::Equals => {
                    // Convert PropertyValue to Polars literal
                    let lit = match &filter.value {
                        ontology_engine::PropertyValue::String(s) => {
                            lit(s.clone())
                        }
                        ontology_engine::PropertyValue::Integer(i) => {
                            lit(*i)
                        }
                        ontology_engine::PropertyValue::Double(d) => {
                            lit(*d)
                        }
                        ontology_engine::PropertyValue::Boolean(b) => {
                            lit(*b)
                        }
                        _ => {
                            return Err(StoreError::Query(
                                format!("Unsupported filter value type for property: {}", filter.property)
                            ));
                        }
                    };
                    lf = lf.filter(col(&filter.property).eq(lit));
                }
                _ => {
                    // Other filter types can be added here
                    return Err(StoreError::Query(
                        format!("Filter operator {:?} not yet implemented", filter.operator)
                    ));
                }
            }
        }

        // 3. Build aggregations
        if query.aggregations.is_empty() {
            return Err(StoreError::Query("At least one aggregation is required".to_string()));
        }

        let mut agg_exprs = Vec::new();
        
        for agg in &query.aggregations {
            match agg {
                Aggregation::Count => {
                    agg_exprs.push(lit(1i32).sum().alias("count"));
                }
                Aggregation::Sum(prop) => {
                    agg_exprs.push(col(prop).sum().alias(&format!("sum_{}", prop)));
                }
                Aggregation::Avg(prop) => {
                    agg_exprs.push(col(prop).mean().alias(&format!("avg_{}", prop)));
                }
                Aggregation::Min(prop) => {
                    agg_exprs.push(col(prop).min().alias(&format!("min_{}", prop)));
                }
                Aggregation::Max(prop) => {
                    agg_exprs.push(col(prop).max().alias(&format!("max_{}", prop)));
                }
                Aggregation::Median(prop) => {
                    agg_exprs.push(col(prop).median().alias(&format!("median_{}", prop)));
                }
                Aggregation::StdDev(prop) => {
                    agg_exprs.push(col(prop).std(1).alias(&format!("stddev_{}", prop)));
                }
                Aggregation::Variance(prop) => {
                    agg_exprs.push(col(prop).var(1).alias(&format!("variance_{}", prop)));
                }
                Aggregation::Percentile(prop, pct) => {
                    let pct_val = (*pct * 100.0) as u8;
                    agg_exprs.push(col(prop).quantile(lit(*pct), QuantileInterpolOptions::Linear).alias(&format!("p{}_", pct_val)));
                }
                Aggregation::DistinctCount(prop) => {
                    agg_exprs.push(col(prop).n_unique().alias(&format!("distinct_count_{}", prop)));
                }
                Aggregation::TopN(prop, _n) => {
                    // TopN is handled separately as it requires sorting the entire dataset
                    // This will be implemented as a post-processing step
                    return Err(StoreError::Query(
                        format!("TopN aggregation for property '{}' requires separate handling", prop)
                    ));
                }
                Aggregation::BottomN(prop, _n) => {
                    // BottomN is handled separately as it requires sorting the entire dataset
                    return Err(StoreError::Query(
                        format!("BottomN aggregation for property '{}' requires separate handling", prop)
                    ));
                }
            }
        }

        // 4. Apply group_by if specified, otherwise aggregate globally
        if !query.group_by.is_empty() {
            let group_cols: Vec<Expr> = query.group_by.iter().map(|s| col(s)).collect();
            lf = lf.group_by(group_cols).agg(agg_exprs);
        } else {
            lf = lf.select(agg_exprs);
        }

        // 5. Execute query
        let df = lf
            .collect()
            .map_err(|e| StoreError::ReadError(format!("Query execution error: {}", e)))?;

        // 6. Convert DataFrame to AnalyticsResult
        let mut rows = Vec::new();
        let height = df.height();
        
        for row_idx in 0..height {
            let mut row_map = HashMap::new();
            
            for col_name in df.get_column_names() {
                let series = df.column(col_name)
                    .map_err(|e| StoreError::ReadError(format!("Column access error: {}", e)))?;
                
                // Convert Polars value to PropertyValue
                let prop_value = match series.dtype() {
                    DataType::String => {
                        let str_val = series.str().unwrap().get(row_idx);
                        ontology_engine::PropertyValue::String(
                            str_val.map(|s| s.to_string()).unwrap_or_default()
                        )
                    }
                    DataType::Int64 => {
                        let int_val = series.i64().unwrap().get(row_idx);
                        ontology_engine::PropertyValue::Integer(int_val.unwrap_or(0))
                    }
                    DataType::Float64 => {
                        let float_val = series.f64().unwrap().get(row_idx);
                        ontology_engine::PropertyValue::Double(float_val.unwrap_or(0.0))
                    }
                    DataType::Boolean => {
                        let bool_val = series.bool().unwrap().get(row_idx);
                        ontology_engine::PropertyValue::Boolean(bool_val.unwrap_or(false))
                    }
                    _ => {
                        // Fallback to string representation
                        let str_val = series.get(row_idx).map(|v| v.to_string());
                        ontology_engine::PropertyValue::String(str_val.unwrap_or_default())
                    }
                };
                
                row_map.insert(col_name.to_string(), prop_value);
            }
            
            rows.push(row_map);
        }

        Ok(AnalyticsResult {
            rows,
            total: height,
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

    #[tokio::test]
    async fn test_parquet_store_flow() {
        use std::fs;
        use chrono::Utc;

        // 1. Setup
        let test_dir = "./test_data_parquet";
        let store = ParquetStore::new(test_dir.to_string());
        
        // Clean up any existing test data
        let _ = fs::remove_dir_all(test_dir);
        fs::create_dir_all(test_dir).expect("Failed to create test directory");

        // 2. Create test data with mixed sparse keys
        let mut objects = Vec::new();
        
        let mut props1 = PropertyMap::new();
        props1.insert("id".to_string(), PropertyValue::Integer(1));
        props1.insert("category".to_string(), PropertyValue::String("A".to_string()));
        props1.insert("score".to_string(), PropertyValue::Double(10.5));
        objects.push(IndexedObject {
            object_type: "metrics".to_string(),
            object_id: "obj_1".to_string(),
            properties: props1,
            indexed_at: Utc::now(),
            source_last_modified: None,
            refresh_frequency: None,
            next_refresh: None,
            refresh_status: RefreshStatus::UpToDate,
        });

        let mut props2 = PropertyMap::new();
        props2.insert("id".to_string(), PropertyValue::Integer(2));
        props2.insert("category".to_string(), PropertyValue::String("B".to_string()));
        props2.insert("score".to_string(), PropertyValue::Double(20.0));
        props2.insert("extra".to_string(), PropertyValue::String("field".to_string()));
        objects.push(IndexedObject {
            object_type: "metrics".to_string(),
            object_id: "obj_2".to_string(),
            properties: props2,
            indexed_at: Utc::now(),
            source_last_modified: None,
            refresh_frequency: None,
            next_refresh: None,
            refresh_status: RefreshStatus::UpToDate,
        });

        let mut props3 = PropertyMap::new();
        props3.insert("id".to_string(), PropertyValue::Integer(3));
        props3.insert("category".to_string(), PropertyValue::String("A".to_string()));
        props3.insert("score".to_string(), PropertyValue::Double(15.5));
        objects.push(IndexedObject {
            object_type: "metrics".to_string(),
            object_id: "obj_3".to_string(),
            properties: props3,
            indexed_at: Utc::now(),
            source_last_modified: None,
            refresh_frequency: None,
            next_refresh: None,
            refresh_status: RefreshStatus::UpToDate,
        });

        // 3. Write batch
        store.write_batch("metrics", objects).await.expect("Write failed");

        // 4. Query analytics - calculate average score
        let query = AnalyticsQuery {
            aggregations: vec![Aggregation::Avg("score".to_string())],
            filters: vec![],
            group_by: vec![],
        };
        
        let result = store.query_analytics("metrics", &query).await.expect("Query failed");
        
        // 5. Assert ( (10.5 + 20.0 + 15.5) / 3 = 15.333... )
        assert_eq!(result.total, 1);
        assert_eq!(result.rows.len(), 1);
        
        let avg_value = result.rows[0].get("avg_score")
            .expect("avg_score column should exist");
        
        match avg_value {
            PropertyValue::Double(avg) => {
                assert!((avg - 15.333).abs() < 0.001, "Average should be approximately 15.333, got {}", avg);
            }
            _ => panic!("Expected Double value for average"),
        }

        // 6. Test group_by query
        let group_query = AnalyticsQuery {
            aggregations: vec![Aggregation::Avg("score".to_string())],
            filters: vec![],
            group_by: vec!["category".to_string()],
        };
        
        let group_result = store.query_analytics("metrics", &group_query).await.expect("Group query failed");
        
        // Should have 2 groups (A and B)
        assert_eq!(group_result.total, 2);
        assert_eq!(group_result.rows.len(), 2);

        // Clean up
        let _ = fs::remove_dir_all(test_dir);
    }
}

