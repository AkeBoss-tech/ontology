use async_graphql::{Schema, EmptySubscription};
use crate::resolvers::QueryRoot;
use crate::admin::AdminMutations;

/// Create the GraphQL schema dynamically from ontology
pub fn create_schema() -> Schema<QueryRoot, AdminMutations, EmptySubscription> {
    Schema::build(QueryRoot::default(), AdminMutations::default(), EmptySubscription)
        .finish()
}

