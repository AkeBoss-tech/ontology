/// Test loading the census ontology
use ontology_engine::Ontology;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ontology_path = "examples/census/config/census_ontology.yaml";
    
    println!("Loading census ontology from: {}", ontology_path);
    
    let content = fs::read_to_string(ontology_path)?;
    let ontology = Ontology::from_yaml(&content)?;
    
    println!("✓ Ontology loaded successfully!");
    println!("\nObject Types:");
    for obj_type in ontology.object_types() {
        println!("  - {}: {}", obj_type.id, obj_type.display_name);
    }
    
    println!("\nLink Types:");
    for link_type in ontology.link_types() {
        println!("  - {}: {} -> {}", 
            link_type.id, 
            link_type.source, 
            link_type.target);
    }
    
    // Verify key object types exist
    assert!(ontology.get_object_type("census_tract_vintage").is_some());
    assert!(ontology.get_object_type("pums_person").is_some());
    assert!(ontology.get_object_type("boundary_crosswalk").is_some());
    
    // Verify key link types exist
    assert!(ontology.get_link_type("tract_to_county").is_some());
    assert!(ontology.get_link_type("household_to_person").is_some());
    
    println!("\n✓ All key types verified!");
    println!("\nOntology is ready for use.");
    
    Ok(())
}





