pub mod meta_model;
pub mod property;
pub mod link;
pub mod action;
pub mod validation;
pub mod dynamic;
pub mod reference;
pub mod action_executor;
pub mod crosswalk;

pub use meta_model::{ObjectType, LinkTypeDef, ActionTypeDef, OntologyRuntime as Ontology, OntologyConfig, OntologyDef};
pub use property::{PropertyType, Property, PropertyValue, PropertyMap};
pub use link::{Link, LinkCardinality, LinkDirection};
pub use action::{Action, ActionOperation, ActionSideEffect};
pub use reference::{ReferenceManager, CascadeDeleteBehavior};
pub use action_executor::{ActionExecutor, ActionExecutionResult};
pub use crosswalk::{CrosswalkTraverser, CrosswalkLink};

