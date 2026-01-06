use async_graphql::{Schema, EmptySubscription, MergedObject};
use crate::resolvers::QueryRoot;
use crate::admin::AdminMutations;
use crate::model_resolvers::{ModelQueries, ModelMutations};

/// Combined query root with model queries
#[derive(MergedObject, Default)]
pub struct Query(QueryRoot, ModelQueries);

/// Combined mutation root with admin and model mutations
#[derive(MergedObject, Default)]
pub struct Mutation(AdminMutations, ModelMutations);

/// Create the GraphQL schema dynamically from ontology
pub fn create_schema() -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .finish()
}
