import React, { useState, useEffect } from 'react';
import { PlusIcon, TrashIcon } from '@heroicons/react/24/outline';

// Array Editor Component
function ArrayEditor({ value, onChange, elementType }: { value: any[]; onChange: (value: any[]) => void; elementType: string }) {
  const addItem = () => {
    const defaultValue = elementType === 'number' || elementType === 'integer' ? 0 : '';
    onChange([...value, defaultValue]);
  };

  const removeItem = (index: number) => {
    onChange(value.filter((_, i) => i !== index));
  };

  const updateItem = (index: number, newValue: any) => {
    const newArray = [...value];
    newArray[index] = newValue;
    onChange(newArray);
  };

  return (
    <div className="array-editor space-y-2">
      {value.map((item, index) => (
        <div key={index} className="flex gap-2 items-center">
          <input
            type={elementType === 'number' || elementType === 'integer' ? 'number' : 'text'}
            value={item}
            onChange={(e) => {
              const newValue = elementType === 'number' || elementType === 'integer' 
                ? (elementType === 'integer' ? parseInt(e.target.value) || 0 : parseFloat(e.target.value) || 0)
                : e.target.value;
              updateItem(index, newValue);
            }}
            className="flex-1 px-3 py-2 border rounded"
            placeholder={`Item ${index + 1}`}
          />
          <button
            onClick={() => removeItem(index)}
            className="p-2 text-red-500 hover:bg-red-50 rounded"
          >
            <TrashIcon className="w-4 h-4" />
          </button>
        </div>
      ))}
      <button
        onClick={addItem}
        className="flex items-center gap-1 px-3 py-1 text-sm border rounded hover:bg-gray-50"
      >
        <PlusIcon className="w-4 h-4" />
        Add Item
      </button>
    </div>
  );
}

// Map/Object Editor Component
function MapEditor({ value, onChange }: { value: Record<string, any>; onChange: (value: Record<string, any>) => void }) {
  const [entries, setEntries] = useState<Array<{ key: string; value: any }>>(
    Object.entries(value).map(([k, v]) => ({ key: k, value: v }))
  );

  useEffect(() => {
    const newValue: Record<string, any> = {};
    entries.forEach(({ key, value: val }) => {
      if (key) {
        newValue[key] = val;
      }
    });
    onChange(newValue);
  }, [entries, onChange]);

  const addEntry = () => {
    setEntries([...entries, { key: '', value: '' }]);
  };

  const removeEntry = (index: number) => {
    setEntries(entries.filter((_, i) => i !== index));
  };

  const updateEntry = (index: number, field: 'key' | 'value', newValue: any) => {
    const newEntries = [...entries];
    newEntries[index] = { ...newEntries[index], [field]: newValue };
    setEntries(newEntries);
  };

  return (
    <div className="map-editor space-y-2">
      {entries.map((entry, index) => (
        <div key={index} className="flex gap-2 items-center">
          <input
            type="text"
            value={entry.key}
            onChange={(e) => updateEntry(index, 'key', e.target.value)}
            placeholder="Key"
            className="w-1/3 px-3 py-2 border rounded"
          />
          <input
            type="text"
            value={entry.value}
            onChange={(e) => updateEntry(index, 'value', e.target.value)}
            placeholder="Value"
            className="flex-1 px-3 py-2 border rounded"
          />
          <button
            onClick={() => removeEntry(index)}
            className="p-2 text-red-500 hover:bg-red-50 rounded"
          >
            <TrashIcon className="w-4 h-4" />
          </button>
        </div>
      ))}
      <button
        onClick={addEntry}
        className="flex items-center gap-1 px-3 py-1 text-sm border rounded hover:bg-gray-50"
      >
        <PlusIcon className="w-4 h-4" />
        Add Entry
      </button>
    </div>
  );
}

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
      case 'array':
        return (
          <ArrayEditor
            value={Array.isArray(value) ? value : []}
            onChange={(newArray) => handleChange(prop.id, newArray)}
            elementType={prop.validation?.enumValues?.[0] || 'string'}
          />
        );
      case 'map':
      case 'object':
        return (
          <MapEditor
            value={typeof value === 'object' && value !== null && !Array.isArray(value) ? value : {}}
            onChange={(newMap) => handleChange(prop.id, newMap)}
          />
        );
      default:
        // Check if type contains "array" or "map"
        if (prop.type.toLowerCase().includes('array')) {
          return (
            <ArrayEditor
              value={Array.isArray(value) ? value : []}
              onChange={(newArray) => handleChange(prop.id, newArray)}
              elementType={prop.type.toLowerCase().replace('array', '').trim() || 'string'}
            />
          );
        }
        if (prop.type.toLowerCase().includes('map') || prop.type.toLowerCase().includes('object')) {
          return (
            <MapEditor
              value={typeof value === 'object' && value !== null && !Array.isArray(value) ? value : {}}
              onChange={(newMap) => handleChange(prop.id, newMap)}
            />
          );
        }
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

