use ontology_engine::{PropertyType, Property, PropertyValue};

#[test]
fn test_geojson_property_type() {
    assert_eq!(
        PropertyType::from_str("geojson").unwrap(),
        PropertyType::GeoJSON
    );
    assert_eq!(
        PropertyType::from_str("geo_json").unwrap(),
        PropertyType::GeoJSON
    );
}

#[test]
fn test_geojson_validation() {
    let prop = Property {
        id: "geoshape".to_string(),
        display_name: None,
        property_type: PropertyType::GeoJSON,
        required: false,
        default: None,
        validation: None,
    };

    // Valid GeoJSON
    let valid_geojson = r#"{"type":"Point","coordinates":[100.0,0.0]}"#;
    assert!(prop.validate_value(&PropertyValue::GeoJSON(valid_geojson.to_string())).is_ok());

    // Invalid GeoJSON
    let invalid_geojson = r#"{"type":"Invalid"}"#;
    assert!(prop.validate_value(&PropertyValue::GeoJSON(invalid_geojson.to_string())).is_err());
}



