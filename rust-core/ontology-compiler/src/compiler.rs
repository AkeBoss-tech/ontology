use anyhow::{Context, Result};
use oxigraph::model::{NamedNode, NamedNodeRef, Term, Literal, Subject, SubjectRef, GraphNameRef};
use oxigraph::store::Store;
use ontology_engine::{
    ObjectType, Property, PropertyType, LinkTypeDef, LinkCardinality,
    OntologyDef, InterfaceDef
};
use std::collections::HashMap;
use std::path::Path;
use std::fs;

// Namespaces
const OWL: &str = "http://www.w3.org/2002/07/owl#";
const RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
const XSD: &str = "http://www.w3.org/2001/XMLSchema#";
const SYS: &str = "http://your-platform.com/ontology/system#";

pub struct Compiler {
    store: Store,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            store: Store::new().unwrap(),
        }
    }

    pub fn load_ttl_files(&self, dir: &Path) -> Result<()> {
        if !dir.exists() {
            return Err(anyhow::anyhow!("Directory not found: {:?}", dir));
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "ttl") {
                println!("Loading {:?}", path);
                let file = fs::File::open(&path)?;
                self.store.load_graph(std::io::BufReader::new(file), oxigraph::io::GraphFormat::Turtle, GraphNameRef::DefaultGraph, None)
                    .map_err(|e| anyhow::anyhow!("Failed to load {:?}: {}", path, e))?;
            }
        }
        Ok(())
    }

    pub fn compile(&self) -> Result<OntologyDef> {
        let object_types = self.compile_object_types()?;
        let link_types = self.compile_link_types()?;
        let interfaces = self.compile_interfaces()?;

        Ok(OntologyDef {
            object_types,
            link_types,
            action_types: vec![], // Will be filled from sidecar
            interfaces,
            function_types: vec![], // Will be filled from sidecar
        })
    }

    fn compile_object_types(&self) -> Result<Vec<ObjectType>> {
        let mut object_types = Vec::new();

        let owl_class = NamedNode::new(format!("{}Class", OWL)).unwrap();
        let rdf_type = NamedNode::new(format!("{}type", RDF)).unwrap();

        // Find all classes
        for quad in self.store.quads_for_pattern(None, Some(rdf_type.as_ref()), Some(owl_class.as_ref().into()), None) {
            let quad = quad?;
            if let Subject::NamedNode(subject) = quad.subject {
                // Check if it is an Interface (we'll assume anything not marked as Interface is an ObjectType for now, or check explicit annotations)
                // For this MVP, let's treat everything as ObjectType unless it starts with "Location" or "Address" which are ambiguous in the example.
                // Better approach: Check if it has a Primary Key. If yes, it's an ObjectType.

                let primary_key_prop = NamedNode::new(format!("{}primaryKey", SYS)).unwrap();
                let has_pk = self.store.quads_for_pattern(Some(subject.as_ref().into()), Some(primary_key_prop.as_ref()), None, None).next().is_some();

                if has_pk {
                    object_types.push(self.build_object_type(&subject)?);
                }
            }
        }

        Ok(object_types)
    }

    fn build_object_type(&self, subject: &NamedNode) -> Result<ObjectType> {
        let id = self.extract_name(subject);
        let display_name = self.get_label(subject).unwrap_or_else(|| id.clone());

        // Primary Key
        let pk_prop = NamedNode::new(format!("{}primaryKey", SYS)).unwrap();
        let primary_key_iri = self.get_object_resource(subject, &pk_prop)
            .ok_or_else(|| anyhow::anyhow!("Missing sys:primaryKey for {}", id))?;
        let primary_key = self.extract_name(&primary_key_iri);

        // Properties
        let properties = self.get_properties_for_domain(subject)?;

        // Backing Datasource
        let ds_prop = NamedNode::new(format!("{}backingDatasource", SYS)).unwrap();
        let backing_datasource = self.get_object_literal(subject, &ds_prop);

        // Title Key
        let tk_prop = NamedNode::new(format!("{}titleKey", SYS)).unwrap();
        let title_key_iri = self.get_object_resource(subject, &tk_prop);
        let title_key = title_key_iri.map(|iri| self.extract_name(&iri));

        // Implements
        let impl_prop = NamedNode::new(format!("{}implements", SYS)).unwrap();
        let sub_class_prop = NamedNode::new(format!("{}subClassOf", RDFS)).unwrap();
        let mut implements = Vec::new();

        // Check sys:implements
        for quad in self.store.quads_for_pattern(Some(subject.as_ref().into()), Some(impl_prop.as_ref()), None, None) {
            let quad = quad?;
            if let Term::NamedNode(obj) = quad.object {
                implements.push(self.extract_name(&obj));
            }
        }

        // Check rdfs:subClassOf
        for quad in self.store.quads_for_pattern(Some(subject.as_ref().into()), Some(sub_class_prop.as_ref()), None, None) {
            let quad = quad?;
            if let Term::NamedNode(obj) = quad.object {
                let name = self.extract_name(&obj);
                // Filter out standard OWL/RDF classes if they appear
                if name != "Thing" && name != "Resource" {
                    implements.push(name);
                }
            }
        }

        implements.sort();
        implements.dedup();

        Ok(ObjectType {
            schema_evolution: None,
            id,
            display_name,
            primary_key,
            properties,
            backing_datasource,
            title_key,
            implements,
        })
    }

    fn compile_interfaces(&self) -> Result<Vec<InterfaceDef>> {
        let mut interfaces = Vec::new();
        // Identify interfaces. In our example, 'Location' is an interface.
        // We can distinguish them by LACK of primaryKey, or explicit annotation.
        // For now, let's find classes without primaryKey but with properties.
        // OR better, let's look for things that are domains of properties but NOT object types.

        let owl_class = NamedNode::new(format!("{}Class", OWL)).unwrap();
        let rdf_type = NamedNode::new(format!("{}type", RDF)).unwrap();
        let primary_key_prop = NamedNode::new(format!("{}primaryKey", SYS)).unwrap();

        for quad in self.store.quads_for_pattern(None, Some(rdf_type.as_ref()), Some(owl_class.as_ref().into()), None) {
            let quad = quad?;
            if let Subject::NamedNode(subject) = quad.subject {
                let has_pk = self.store.quads_for_pattern(Some(subject.as_ref().into()), Some(primary_key_prop.as_ref()), None, None).next().is_some();

                let name = self.extract_name(&subject);
                if !has_pk && (name == "Location" || name == "Address") {
                     // In the YAML, Address is a struct property type, Location is an Interface.
                     // Let's treat "Location" as Interface.
                     if name == "Location" {
                         interfaces.push(InterfaceDef {
                             id: name.clone(),
                             display_name: self.get_label(&subject).unwrap_or(name),
                             properties: self.get_properties_for_domain(&subject)?,
                             required_link_types: vec![], // Not implemented in MVP
                         });
                     }
                }
            }
        }
        Ok(interfaces)
    }

    fn compile_link_types(&self) -> Result<Vec<LinkTypeDef>> {
        let mut links = Vec::new();
        let owl_obj_prop = NamedNode::new(format!("{}ObjectProperty", OWL)).unwrap();
        let rdf_type = NamedNode::new(format!("{}type", RDF)).unwrap();

        for quad in self.store.quads_for_pattern(None, Some(rdf_type.as_ref()), Some(owl_obj_prop.as_ref().into()), None) {
            let quad = quad?;
            if let Subject::NamedNode(subject) = quad.subject {
                let id = self.extract_name(&subject);
                let display_name = self.get_label(&subject);

                let domain_prop = NamedNode::new(format!("{}domain", RDFS)).unwrap();
                let source_iri = self.get_object_resource(&subject, &domain_prop)
                    .ok_or_else(|| anyhow::anyhow!("Missing rdfs:domain for link {}", id))?;
                let source = self.extract_name(&source_iri);

                let range_prop = NamedNode::new(format!("{}range", RDFS)).unwrap();
                let target_iri = self.get_object_resource(&subject, &range_prop)
                    .ok_or_else(|| anyhow::anyhow!("Missing rdfs:range for link {}", id))?;
                let target = self.extract_name(&target_iri);

                // Bidirectional
                let bidi_prop = NamedNode::new(format!("{}bidirectional", SYS)).unwrap();
                let bidirectional = self.get_object_literal(&subject, &bidi_prop)
                    .map(|v| v == "true")
                    .unwrap_or(false);

                links.push(LinkTypeDef {
                    id,
                    display_name,
                    source,
                    target,
                    cardinality: LinkCardinality::OneToMany, // Default, hard to infer from standard OWL without constraints
                    properties: vec![], // Link properties not in MVP TTL
                    bidirectional,
                });
            }
        }
        Ok(links)
    }

    fn get_properties_for_domain(&self, domain: &NamedNode) -> Result<Vec<Property>> {
        let mut properties = Vec::new();
        let owl_dp = NamedNode::new(format!("{}DatatypeProperty", OWL)).unwrap();
        let rdf_type = NamedNode::new(format!("{}type", RDF)).unwrap();

        // Scan all DatatypeProperties
        // In a real triplestore we'd query ?p rdfs:domain ?domain.
        // Here we iterate all DPs and check their domain.

        for quad in self.store.quads_for_pattern(None, Some(rdf_type.as_ref()), Some(owl_dp.as_ref().into()), None) {
             let quad = quad?;
             if let Subject::NamedNode(prop_subject) = quad.subject {
                 let domain_prop = NamedNode::new(format!("{}domain", RDFS)).unwrap();
                 // Check if this property belongs to the domain
                 if self.store.quads_for_pattern(Some(prop_subject.as_ref().into()), Some(domain_prop.as_ref()), Some(domain.as_ref().into()), None).next().is_some() {

                     let id = self.extract_name(&prop_subject);
                     let range_prop = NamedNode::new(format!("{}range", RDFS)).unwrap();
                     let range_iri = self.get_object_resource(&prop_subject, &range_prop);

                     let property_type = if let Some(range) = range_iri {
                         self.map_rdf_type_to_property_type(&range)
                     } else {
                         PropertyType::String
                     };

                     let unit_prop = NamedNode::new(format!("{}unit", SYS)).unwrap();
                     let unit = self.get_object_literal(&prop_subject, &unit_prop);

                     properties.push(Property {
                         id,
                         display_name: self.get_label(&prop_subject),
                         property_type,
                         required: false, // Default to false for MVP
                         default: None,
                         validation: None,
                         description: self.get_label(&prop_subject), // Use label as description
                         annotations: HashMap::new(),
                         unit,
                         format: None,
                         sensitivity_tags: vec![],
                         pii: false,
                         deprecated: None,
                         statistics: None,
                     });
                 }
             }
        }
        Ok(properties)
    }

    fn map_rdf_type_to_property_type(&self, range: &NamedNode) -> PropertyType {
        let uri = range.as_str();
        match uri {
            u if u == format!("{}string", XSD) => PropertyType::String,
            u if u == format!("{}double", XSD) => PropertyType::Double,
            u if u == format!("{}integer", XSD) => PropertyType::Integer,
            u if u == format!("{}boolean", XSD) => PropertyType::Boolean,
            u if u == format!("{}date", XSD) => PropertyType::Date,
            u if u == format!("{}dateTime", XSD) => PropertyType::Timestamp,
            u if u == format!("{}List", RDF) => PropertyType::Array { element_type: Box::new(PropertyType::String) }, // Simplified
            // Check for custom Struct types (Address)
            _ => {
                // If it points to :Address, it is an object/struct type
                // Simplified: assuming anything else is a string map or struct
                 PropertyType::Map { key_type: Box::new(PropertyType::String), value_type: Box::new(PropertyType::String) }
            }
        }
    }

    fn extract_name(&self, node: &NamedNode) -> String {
        // Get the fragment or the last part of path
        let s = node.as_str();
        if let Some(fragment) = s.rsplit('#').next() {
             if !fragment.is_empty() && fragment != s { return fragment.to_string(); }
        }
        s.rsplit('/').next().unwrap_or(s).to_string()
    }

    fn get_label(&self, subject: &NamedNode) -> Option<String> {
        let label_prop = NamedNode::new(format!("{}label", RDFS)).unwrap();
        self.get_object_literal(subject, &label_prop)
    }

    fn get_object_literal(&self, subject: &NamedNode, predicate: &NamedNode) -> Option<String> {
        if let Some(quad) = self.store.quads_for_pattern(Some(subject.as_ref().into()), Some(predicate.as_ref()), None, None).next() {
             if let Ok(q) = quad {
                 if let Term::Literal(lit) = q.object {
                     return Some(lit.value().to_string());
                 }
             }
        }
        None
    }

    fn get_object_resource(&self, subject: &NamedNode, predicate: &NamedNode) -> Option<NamedNode> {
        if let Some(quad) = self.store.quads_for_pattern(Some(subject.as_ref().into()), Some(predicate.as_ref()), None, None).next() {
             if let Ok(q) = quad {
                 if let Term::NamedNode(node) = q.object {
                     return Some(node);
                 }
             }
        }
        None
    }
}
