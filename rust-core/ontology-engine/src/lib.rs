pub mod meta_model;
pub mod property;
pub mod link;
pub mod action;
pub mod validation;
pub mod dynamic;

pub use meta_model::{ObjectType, LinkTypeDef, ActionTypeDef, OntologyRuntime as Ontology, OntologyConfig, OntologyDef};
pub use property::{PropertyType, Property, PropertyValue, PropertyMap};
pub use link::{Link, LinkCardinality, LinkDirection};
pub use action::{Action, ActionOperation, ActionSideEffect};

