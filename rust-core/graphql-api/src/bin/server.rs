use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use axum::{body::Body, extract::State, response::IntoResponse, routing::get, Router};
use graphql_api::{AdminMutations, QueryRoot};
use indexing::hydration::ObjectHydrator;
use indexing::store::{DgraphStore, ElasticsearchStore, ParquetStore};
use ontology_engine::Ontology;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use versioning::event_log::EventLog;
use versioning::time_query::TimeQuery;

// In-memory data store for demo purposes
lazy_static::lazy_static! {
    static ref DATA_STORE: Arc<tokio::sync::RwLock<HashMap<String, Vec<Value>>>> =
        Arc::new(tokio::sync::RwLock::new(HashMap::new()));
}

async fn load_data_from_files() {
    // Try multiple possible paths
    let possible_paths = vec![
        std::path::Path::new("examples/census/data"),
        std::path::Path::new("../../examples/census/data"),
        std::path::Path::new("../examples/census/data"),
    ];

    let mut data_dir = None;
    for path in &possible_paths {
        if path.exists() {
            data_dir = Some(path);
            println!("Using data directory: {}", path.display());
            break;
        }
    }

    let data_dir = match data_dir {
        Some(dir) => dir,
        None => {
            println!(
                "⚠ Warning: Could not find data directory. Tried: {:?}",
                possible_paths
            );
            return;
        }
    };

    let files = vec![
        ("states.json", "state_vintage"),
        ("counties.json", "county_vintage"),
        ("tracts.json", "census_tract_vintage"),
        ("pumas.json", "puma_vintage"),
        ("crosswalks.json", "boundary_crosswalk"),
        ("households.json", "pums_household"),
        ("persons.json", "pums_person"),
    ];

    let mut store = DATA_STORE.write().await;
    let mut total_loaded = 0;

    for (filename, object_type) in files {
        let file_path = data_dir.join(filename);
        if file_path.exists() {
            match fs::read_to_string(&file_path) {
                Ok(content) => {
                    match serde_json::from_str::<Vec<Value>>(&content) {
                        Ok(mut objects) => {
                            // Inject computed primary keys where the data doesn't include them
                            if object_type == "pums_person" {
                                for obj in &mut objects {
                                    if let Value::Object(map) = obj {
                                        if !map.contains_key("person_id_year") {
                                            let person_id = map
                                                .get("person_id")
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("unknown");
                                            let year = map
                                                .get("year")
                                                .and_then(|v| v.as_i64())
                                                .unwrap_or(0);
                                            map.insert(
                                                "person_id_year".to_string(),
                                                Value::String(format!("{}_{}", person_id, year)),
                                            );
                                        }
                                    }
                                }
                            }

                            let key = object_type.to_string();
                            let count = objects.len();
                            store.insert(key.clone(), objects);
                            total_loaded += count;
                            println!(
                                "✓ Loaded {} {} objects from {}",
                                count, object_type, filename
                            );
                        }
                        Err(e) => {
                            println!("⚠ Failed to parse {}: {}", filename, e);
                        }
                    }
                }
                Err(e) => {
                    println!("⚠ Failed to read {}: {}", filename, e);
                }
            }
        } else {
            println!("⚠ File not found: {}", file_path.display());
        }
    }

    println!(
        "✓ Data loading complete: {} total objects loaded",
        total_loaded
    );
    println!("  Store keys: {:?}", store.keys().collect::<Vec<_>>());
}

#[tokio::main]
async fn main() {
    // Load data first
    load_data_from_files().await;

    // Load ontology
    let ontology_path = std::env::var("ONTOLOGY_PATH")
        .unwrap_or_else(|_| "examples/census/config/census_ontology.yaml".to_string());

    println!("Loading ontology from: {}", ontology_path);
    let ontology_content =
        fs::read_to_string(&ontology_path).expect("Failed to read ontology file");

    let ontology = Ontology::from_yaml(&ontology_content).expect("Failed to parse ontology");

    println!(
        "✓ Loaded ontology with {} object types",
        ontology.object_types().count()
    );

    // Create store backends (using placeholder implementations)
    let search_store: Arc<dyn indexing::store::SearchStore> = Arc::new(
        ElasticsearchStore::new("http://localhost:9200".to_string())
            .expect("Failed to create Elasticsearch store"),
    );
    let graph_store: Arc<dyn indexing::store::GraphStore> = Arc::new(
        DgraphStore::new("http://localhost:9080".to_string())
            .await
            .expect("Failed to create Dgraph store"),
    );
    let columnar_store: Arc<dyn indexing::store::ColumnarStore> =
        Arc::new(ParquetStore::new("data/parquet".to_string()));

    // Create time query
    let event_log = EventLog::new();
    let time_query = Arc::new(TimeQuery::new(event_log));

    // Create hydrator
    let hydrator = ObjectHydrator::new();

    // Create function result cache
    let function_cache: Arc<tokio::sync::RwLock<HashMap<u64, ontology_engine::PropertyValue>>> =
        Arc::new(tokio::sync::RwLock::new(HashMap::new()));

    // Create GraphQL schema
    let schema = Schema::build(
        QueryRoot::default(),
        AdminMutations::default(),
        EmptySubscription,
    )
    .data(Arc::new(ontology))
    .data(search_store.clone() as Arc<dyn indexing::store::SearchStore>)
    .data(graph_store.clone() as Arc<dyn indexing::store::GraphStore>)
    .data(columnar_store.clone() as Arc<dyn indexing::store::ColumnarStore>)
    .data(time_query.clone())
    .data(hydrator)
    .data(DATA_STORE.clone())
    .data(function_cache)
    .finish();

    // GraphQL handler
    async fn graphql_handler(
        State(schema): State<Schema<QueryRoot, AdminMutations, EmptySubscription>>,
        body: Body,
    ) -> impl IntoResponse {
        // Read request body
        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default();
        let body_str = String::from_utf8(bytes.to_vec()).unwrap_or_default();

        // Parse JSON request
        let request: Value = serde_json::from_str(&body_str).unwrap_or(Value::Null);

        // Extract query
        let query = request
            .get("query")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        // Extract variables
        let variables = request
            .get("variables")
            .cloned()
            .unwrap_or(Value::Object(serde_json::Map::new()));

        // Execute GraphQL query
        let request = async_graphql::Request::new(query)
            .variables(async_graphql::Variables::from_json(variables));

        let response = schema.execute(request).await;
        let response_json = serde_json::to_string(&response).unwrap_or_default();

        axum::response::Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(Body::from(response_json))
            .unwrap()
    }

    // Playground handler
    async fn graphql_playground() -> impl IntoResponse {
        axum::response::Response::builder()
            .header("content-type", "text/html")
            .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
            .unwrap()
    }

    // Create router with CORS
    let app = Router::new()
        .route(
            "/graphql",
            axum::routing::post(graphql_handler).get(graphql_playground),
        )
        .route(
            "/",
            get(|| async {
                "Ontology GraphQL API\n\nGraphQL endpoint: /graphql\nPlayground: /graphql"
            }),
        )
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(tower_http::cors::Any)
                .allow_methods(tower_http::cors::Any)
                .allow_headers(tower_http::cors::Any),
        )
        .with_state(schema);

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("Invalid PORT");

    println!("Starting GraphQL server on http://localhost:{}", port);
    println!("GraphQL endpoint: http://localhost:{}/graphql", port);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to port");

    axum::serve(listener, app).await.expect("Server failed");
}
