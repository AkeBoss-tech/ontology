import React, { useState, useEffect } from 'react';

export interface PropertyDefinition {
  id: string;
  displayName?: string;
  type: string;
  required?: boolean;
  default?: any;
  validation?: {
    min?: number;
    max?: number;
    minLength?: number;
    maxLength?: number;
    pattern?: string;
    enumValues?: string[];
  };
}

export interface PropertyEditorProps {
  properties: PropertyDefinition[];
  values: Record<string, any>;
  onChange: (values: Record<string, any>) => void;
}

export function PropertyEditor({ properties, values, onChange }: PropertyEditorProps) {
  const [localValues, setLocalValues] = useState<Record<string, any>>(values);

  useEffect(() => {
    setLocalValues(values);
  }, [values]);

  const handleChange = (propertyId: string, value: any) => {
    const newValues = { ...localValues, [propertyId]: value };
    setLocalValues(newValues);
    onChange(newValues);
  };

  const renderPropertyInput = (prop: PropertyDefinition) => {
    const value = localValues[prop.id] ?? prop.default ?? '';

    switch (prop.type.toLowerCase()) {
      case 'string':
      case 'geojson':
        return (
          <input
            type="text"
            value={value}
            onChange={(e) => handleChange(prop.id, e.target.value)}
            className="w-full px-3 py-2 border rounded"
            required={prop.required}
          />
        );
      case 'integer':
        return (
          <input
            type="number"
            value={value}
            onChange={(e) => handleChange(prop.id, parseInt(e.target.value) || 0)}
            className="w-full px-3 py-2 border rounded"
            required={prop.required}
          />
        );
      case 'double':
      case 'float':
        return (
          <input
            type="number"
            step="any"
            value={value}
            onChange={(e) => handleChange(prop.id, parseFloat(e.target.value) || 0)}
            className="w-full px-3 py-2 border rounded"
            required={prop.required}
          />
        );
      case 'boolean':
        return (
          <input
            type="checkbox"
            checked={value}
            onChange={(e) => handleChange(prop.id, e.target.checked)}
            className="w-4 h-4"
          />
        );
      case 'date':
        return (
          <input
            type="date"
            value={value}
            onChange={(e) => handleChange(prop.id, e.target.value)}
            className="w-full px-3 py-2 border rounded"
            required={prop.required}
          />
        );
      case 'datetime':
        return (
          <input
            type="datetime-local"
            value={value}
            onChange={(e) => handleChange(prop.id, e.target.value)}
            className="w-full px-3 py-2 border rounded"
            required={prop.required}
          />
        );
      case 'object_reference':
        return (
          <input
            type="text"
            value={value}
            onChange={(e) => handleChange(prop.id, e.target.value)}
            placeholder="object_type:object_id"
            className="w-full px-3 py-2 border rounded"
            required={prop.required}
          />
        );
      default:
        return (
          <input
            type="text"
            value={value}
            onChange={(e) => handleChange(prop.id, e.target.value)}
            className="w-full px-3 py-2 border rounded"
            required={prop.required}
          />
        );
    }
  };

  return (
    <div className="property-editor space-y-4">
      {properties.map((prop) => (
        <div key={prop.id}>
          <label className="block text-sm font-medium mb-1">
            {prop.displayName || prop.id}
            {prop.required && <span className="text-red-500 ml-1">*</span>}
          </label>
          {renderPropertyInput(prop)}
          {prop.validation?.enumValues && (
            <select
              value={localValues[prop.id] ?? prop.default ?? ''}
              onChange={(e) => handleChange(prop.id, e.target.value)}
              className="w-full px-3 py-2 border rounded mt-1"
            >
              <option value="">Select...</option>
              {prop.validation.enumValues.map((val) => (
                <option key={val} value={val}>
                  {val}
                </option>
              ))}
            </select>
          )}
        </div>
      ))}
    </div>
  );
}

