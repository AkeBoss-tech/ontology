use ontology_engine::Ontology;

#[test]
fn test_full_ontology_lifecycle() {
    // Create an ontology configuration
    let yaml = r#"
ontology:
  objectTypes:
    - id: "employee"
      displayName: "Employee"
      primaryKey: "employee_id"
      properties:
        - id: "employee_id"
          type: "string"
          required: true
        - id: "name"
          type: "string"
        - id: "department"
          type: "string"
      titleKey: "name"
    - id: "department"
      displayName: "Department"
      primaryKey: "dept_id"
      properties:
        - id: "dept_id"
          type: "string"
          required: true
        - id: "name"
          type: "string"
      titleKey: "name"
  linkTypes:
    - id: "works_in"
      displayName: "Works In"
      source: "employee"
      target: "department"
      cardinality: "MANY_TO_ONE"
      bidirectional: true
  actionTypes: []
"#;
    
    // Load ontology
    let ontology = Ontology::from_yaml(yaml).expect("Failed to load ontology");
    
    // Verify object types
    assert!(ontology.get_object_type("employee").is_some());
    assert!(ontology.get_object_type("department").is_some());
    
    let employee_type = ontology.get_object_type("employee").unwrap();
    assert_eq!(employee_type.display_name, "Employee");
    assert_eq!(employee_type.primary_key, "employee_id");
    
    // Verify link types
    assert!(ontology.get_link_type("works_in").is_some());
    let link_type = ontology.get_link_type("works_in").unwrap();
    assert_eq!(link_type.source, "employee");
    assert_eq!(link_type.target, "department");
    
    // Verify we can iterate over types
    let object_type_count: usize = ontology.object_types().count();
    assert_eq!(object_type_count, 2);
    
    let link_type_count: usize = ontology.link_types().count();
    assert_eq!(link_type_count, 1);
}

#[test]
fn test_ontology_validation() {
    // Test invalid ontology - link type references non-existent object type
    let invalid_yaml = r#"
ontology:
  objectTypes:
    - id: "employee"
      displayName: "Employee"
      primaryKey: "employee_id"
      properties:
        - id: "employee_id"
          type: "string"
          required: true
  linkTypes:
    - id: "works_in"
      source: "employee"
      target: "nonexistent"
      cardinality: "MANY_TO_ONE"
  actionTypes: []
"#;
    
    let result = Ontology::from_yaml(invalid_yaml);
    assert!(result.is_err());
}

