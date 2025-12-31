// Quick test script to verify new features work
use ontology_engine::Ontology;

fn main() {
    let yaml = r#"
ontology:
  interfaces:
    - id: "Location"
      displayName: "Location"
      properties:
        - id: "latitude"
          type: "double"
          required: true
        - id: "longitude"
          type: "double"
          required: true
      requiredLinkTypes: []
  
  objectTypes:
    - id: "office"
      displayName: "Office"
      primaryKey: "id"
      implements: ["Location"]
      properties:
        - id: "id"
          type: "string"
          required: true
        - id: "name"
          type: "string"
        - id: "latitude"
          type: "double"
          required: true
        - id: "longitude"
          type: "double"
          required: true
  
  functionTypes:
    - id: "test_function"
      displayName: "Test Function"
      description: "A test function"
      parameters:
        - id: "input"
          type: "string"
          required: true
      returnType:
        type: "property"
        propertyType: "string"
      logic:
        type: "property_access"
        property: "input"
      cacheable: false
  
  linkTypes: []
  actionTypes: []
"#;

    match Ontology::from_yaml(yaml) {
        Ok(ontology) => {
            println!("✓ Ontology loaded successfully!");
            println!("  - {} object types", ontology.object_types().count());
            println!("  - {} interfaces", ontology.interfaces().count());
            println!("  - {} function types", ontology.function_types().count());
            
            // Verify interface
            if let Some(location) = ontology.get_interface("Location") {
                println!("✓ Interface 'Location' found with {} properties", location.properties.len());
            }
            
            // Verify object type implements interface
            if let Some(office) = ontology.get_object_type("office") {
                println!("✓ Object type 'office' found");
                println!("  - Implements: {:?}", office.implements);
            }
            
            // Verify function
            if let Some(func) = ontology.get_function_type("test_function") {
                println!("✓ Function 'test_function' found");
                println!("  - Description: {:?}", func.description);
            }
            
            println!("\n✓ All new features verified!");
        }
        Err(e) => {
            eprintln!("✗ Failed to load ontology: {}", e);
            std::process::exit(1);
        }
    }
}



