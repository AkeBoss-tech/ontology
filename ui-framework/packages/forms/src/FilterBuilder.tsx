import React, { useState } from 'react';

export interface Filter {
  property: string;
  operator: string;
  value: string;
  distance?: number;
}

export interface FilterBuilderProps {
  availableProperties: string[];
  availableOperators?: string[];
  filters: Filter[];
  onChange: (filters: Filter[]) => void;
}

const DEFAULT_OPERATORS = [
  'equals',
  'notequals',
  'greaterthan',
  'lessthan',
  'contains',
  'startswith',
  'endswith',
  'in',
];

export function FilterBuilder({
  availableProperties,
  availableOperators = DEFAULT_OPERATORS,
  filters,
  onChange,
}: FilterBuilderProps) {
  const addFilter = () => {
    onChange([
      ...filters,
      {
        property: availableProperties[0] || '',
        operator: availableOperators[0] || 'equals',
        value: '',
      },
    ]);
  };

  const updateFilter = (index: number, updates: Partial<Filter>) => {
    const newFilters = [...filters];
    newFilters[index] = { ...newFilters[index], ...updates };
    onChange(newFilters);
  };

  const removeFilter = (index: number) => {
    onChange(filters.filter((_, i) => i !== index));
  };

  return (
    <div className="filter-builder space-y-2">
      <div className="flex justify-between items-center mb-2">
        <h3 className="font-semibold">Filters</h3>
        <button
          type="button"
          onClick={addFilter}
          className="px-3 py-1 text-sm bg-gray-200 rounded hover:bg-gray-300"
        >
          Add Filter
        </button>
      </div>

      {filters.map((filter, index) => (
        <div key={index} className="flex gap-2 items-center p-2 border rounded">
          <select
            value={filter.property}
            onChange={(e) => updateFilter(index, { property: e.target.value })}
            className="px-2 py-1 border rounded"
          >
            {availableProperties.map((prop) => (
              <option key={prop} value={prop}>
                {prop}
              </option>
            ))}
          </select>

          <select
            value={filter.operator}
            onChange={(e) => updateFilter(index, { operator: e.target.value })}
            className="px-2 py-1 border rounded"
          >
            {availableOperators.map((op) => (
              <option key={op} value={op}>
                {op}
              </option>
            ))}
          </select>

          <input
            type="text"
            value={filter.value}
            onChange={(e) => updateFilter(index, { value: e.target.value })}
            placeholder="Value"
            className="flex-1 px-2 py-1 border rounded"
          />

          {filter.operator === 'withindistance' && (
            <input
              type="number"
              value={filter.distance || ''}
              onChange={(e) =>
                updateFilter(index, { distance: parseFloat(e.target.value) || undefined })
              }
              placeholder="Distance (m)"
              className="w-24 px-2 py-1 border rounded"
            />
          )}

          <button
            type="button"
            onClick={() => removeFilter(index)}
            className="px-2 py-1 text-red-500 hover:bg-red-50 rounded"
          >
            ×
          </button>
        </div>
      ))}

      {filters.length === 0 && (
        <div className="text-sm text-gray-500 text-center py-4">
          No filters. Click "Add Filter" to add one.
        </div>
      )}
    </div>
  );
}



