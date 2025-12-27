"""Schema mapping - converts source schemas to ontology property mappings."""
from typing import Dict, List, Optional, Any
from pydantic import BaseModel, Field
import yaml
import json


class PropertyMapping(BaseModel):
    """Maps a source column/field to an ontology property."""
    source_column: str
    target_property: str
    transformation: Optional[str] = None  # Optional transformation function name


class SchemaMapping(BaseModel):
    """Complete schema mapping configuration."""
    source_type: str  # "sql", "csv", "api", "kafka"
    source_config: Dict[str, Any]  # Source-specific configuration
    object_type_id: str
    property_mappings: List[PropertyMapping]
    primary_key_mapping: PropertyMapping


class SchemaMapper:
    """Maps source data schemas to ontology object types."""
    
    def __init__(self, mapping_config: Dict[str, Any]):
        """
        Initialize mapper with configuration.
        
        Args:
            mapping_config: Dictionary containing schema mappings
        """
        self.mappings: Dict[str, SchemaMapping] = {}
        self._load_mappings(mapping_config)
    
    @classmethod
    def from_yaml(cls, yaml_path: str) -> 'SchemaMapper':
        """Load schema mappings from a YAML file."""
        with open(yaml_path, 'r') as f:
            config = yaml.safe_load(f)
        return cls(config)
    
    @classmethod
    def from_json(cls, json_path: str) -> 'SchemaMapper':
        """Load schema mappings from a JSON file."""
        with open(json_path, 'r') as f:
            config = json.load(f)
        return cls(config)
    
    def _load_mappings(self, config: Dict[str, Any]):
        """Load mappings from configuration dictionary."""
        mappings = config.get('mappings', [])
        for mapping_data in mappings:
            mapping = SchemaMapping(**mapping_data)
            # Use source identifier as key (e.g., source_table_name)
            key = mapping.source_config.get('identifier', mapping.object_type_id)
            self.mappings[key] = mapping
    
    def get_mapping(self, source_identifier: str) -> Optional[SchemaMapping]:
        """Get schema mapping for a source identifier."""
        return self.mappings.get(source_identifier)
    
    def map_row(self, mapping: SchemaMapping, source_row: Dict[str, Any]) -> Dict[str, Any]:
        """
        Map a source row to ontology properties.
        
        Args:
            mapping: Schema mapping to use
            source_row: Source data row as dictionary
            
        Returns:
            Dictionary of ontology properties
        """
        properties = {}
        
        # Map each property
        for prop_mapping in mapping.property_mappings:
            source_value = source_row.get(prop_mapping.source_column)
            if source_value is not None:
                # Apply transformation if specified
                if prop_mapping.transformation:
                    source_value = self._apply_transformation(
                        prop_mapping.transformation,
                        source_value
                    )
                properties[prop_mapping.target_property] = source_value
        
        return properties
    
    def _apply_transformation(self, transformation: str, value: Any) -> Any:
        """Apply a transformation function to a value."""
        # Simple transformation registry
        transformations = {
            'to_string': str,
            'to_int': int,
            'to_float': float,
            'to_lower': lambda x: str(x).lower(),
            'to_upper': lambda x: str(x).upper(),
        }
        
        if transformation in transformations:
            try:
                return transformations[transformation](value)
            except (ValueError, TypeError):
                return value
        
        return value
    
    def extract_primary_key(self, mapping: SchemaMapping, source_row: Dict[str, Any]) -> str:
        """Extract primary key value from source row."""
        return str(source_row.get(mapping.primary_key_mapping.source_column, ''))


