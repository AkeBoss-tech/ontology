use std::process::Command;
use std::path::Path;
use std::fs;

#[test]
fn test_compiler_end_to_end() {
    // We assume tests are run from the rust-core/ontology-compiler directory (via cargo test)
    let cwd = std::env::current_dir().unwrap();
    println!("Current dir: {:?}", cwd);

    // Paths relative to rust-core directory
    let input_arg = "../ontology-definitions";
    let output_arg = "../generated_ontology_test.json";
    let sidecar_arg = "../ontology-definitions/sidecar.yaml";

    // Path to the output file relative to HERE (rust-core/ontology-compiler)
    let output_file_path = Path::new("../../generated_ontology_test.json");

    // Clean up previous output
    if output_file_path.exists() {
        fs::remove_file(output_file_path).unwrap();
    }

    // Run the compiler using cargo run, executing from rust-core directory
    let status = Command::new("cargo")
        .args(&[
            "run",
            "-p",
            "ontology-compiler",
            "--",
            "--input",
            input_arg,
            "--output",
            output_arg,
            "--sidecar",
            sidecar_arg,
        ])
        .current_dir("..") // Switch to rust-core
        .status()
        .expect("Failed to execute compiler");

    assert!(status.success(), "Compiler execution failed");
    assert!(output_file_path.exists(), "Output file not created at {:?}", output_file_path);

    // Verify content
    let content = fs::read_to_string(output_file_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&content).expect("Invalid JSON output");

    // Check ontology structure
    let ontology = json.get("ontology").expect("Missing ontology field");

    // Check object types
    let object_types = ontology.get("objectTypes").expect("Missing objectTypes").as_array().expect("objectTypes not array");

    // We expect at least Aircraft and MaintenanceEvent
    let aircraft = object_types.iter().find(|o| o["id"] == "Aircraft").expect("Aircraft object type not found");
    let office = object_types.iter().find(|o| o["id"] == "Office").expect("Office object type not found");

    // Check Aircraft properties
    let props = aircraft["properties"].as_array().expect("properties not array");

    // Check tail_number (string)
    let tail_number = props.iter().find(|p| p["id"] == "tail_number").expect("tail_number not found");
    assert_eq!(tail_number["type"], "string");

    // Check flight_hours (double)
    let flight_hours = props.iter().find(|p| p["id"] == "flight_hours").expect("flight_hours not found");
    assert_eq!(flight_hours["type"], "double");

    // Check Office implements Location
    let implements = office["implements"].as_array().expect("implements not array");
    let implements_location = implements.iter().any(|v| v.as_str() == Some("Location"));
    assert!(implements_location, "Office should implement Location");

    // Check Location interface exists
    let interfaces = ontology.get("interfaces").expect("Missing interfaces").as_array().expect("interfaces not array");
    let location = interfaces.iter().find(|i| i["id"] == "Location").expect("Location interface not found");

    // Check Location properties
    let loc_props = location["properties"].as_array().expect("Location properties not array");
    let latitude = loc_props.iter().find(|p| p["id"] == "latitude").expect("latitude not found in Location");
    assert_eq!(latitude["type"], "double");

    // Check MilitaryAircraft (Inheritance via rdfs:subClassOf)
    let military_aircraft = object_types.iter().find(|o| o["id"] == "MilitaryAircraft").expect("MilitaryAircraft object type not found");
    let implements_ma = military_aircraft["implements"].as_array().expect("implements not array");
    assert!(implements_ma.iter().any(|v| v.as_str() == Some("Aircraft")), "MilitaryAircraft should implement Aircraft via subClassOf");

    // Check link types
    let link_types = ontology.get("linkTypes").expect("Missing linkTypes").as_array().expect("linkTypes not array");
    let has_history = link_types.iter().find(|l| l["id"] == "aircraft_has_history").expect("aircraft_has_history link not found");

    // Corrected fields: source and target
    assert_eq!(has_history["source"], "Aircraft");
    assert_eq!(has_history["target"], "MaintenanceEvent");

    // Clean up
    fs::remove_file(output_file_path).unwrap();
}
