import React, { useState } from 'react';
import { BookmarkIcon, BookmarkSlashIcon, BoltIcon } from '@heroicons/react/24/outline';

export interface Filter {
  property: string;
  operator: string;
  value: string;
  distance?: number;
}

export interface FilterGroup {
  id: string;
  logic: 'AND' | 'OR';
  filters: Filter[];
  groups?: FilterGroup[];
}

export interface FilterBuilderProps {
  availableProperties: string[];
  availableOperators?: string[];
  filters: Filter[];
  onChange: (filters: Filter[]) => void;
  enableGroups?: boolean;
  enableSavedPresets?: boolean;
  onSavePreset?: (name: string, filters: Filter[]) => void;
  savedPresets?: Array<{ name: string; filters: Filter[] }>;
  quickFilters?: Array<{ label: string; filters: Filter[] }>;
}

const DEFAULT_OPERATORS = [
  'equals',
  'notequals',
  'greaterthan',
  'lessthan',
  'greaterthanorequal',
  'lessthanorequal',
  'contains',
  'notcontains',
  'startswith',
  'endswith',
  'in',
  'notin',
  'between',
  'isnull',
  'isnotnull',
  'withindistance',
];

export function FilterBuilder({
  availableProperties,
  availableOperators = DEFAULT_OPERATORS,
  filters,
  onChange,
  enableGroups = false,
  enableSavedPresets = false,
  onSavePreset,
  savedPresets = [],
  quickFilters = [],
}: FilterBuilderProps) {
  const [showPresets, setShowPresets] = useState(false);
  const [presetName, setPresetName] = useState('');
  const [showQuickFilters, setShowQuickFilters] = useState(false);

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

  const applyQuickFilter = (quickFilter: { label: string; filters: Filter[] }) => {
    onChange(quickFilter.filters);
  };

  const loadPreset = (preset: { name: string; filters: Filter[] }) => {
    onChange(preset.filters);
    setShowPresets(false);
  };

  const savePreset = () => {
    if (presetName && onSavePreset) {
      onSavePreset(presetName, filters);
      setPresetName('');
      setShowPresets(false);
    }
  };

  const getInputType = (operator: string, property: string): string => {
    if (operator === 'between') return 'text'; // Will show two inputs
    if (operator === 'isnull' || operator === 'isnotnull') return 'hidden';
    if (operator === 'in' || operator === 'notin') return 'text'; // Comma-separated
    if (property.toLowerCase().includes('date') || property.toLowerCase().includes('time')) {
      return 'date';
    }
    if (property.toLowerCase().includes('age') || property.toLowerCase().includes('count')) {
      return 'number';
    }
    return 'text';
  };

  const renderValueInput = (filter: Filter, index: number) => {
    const inputType = getInputType(filter.operator, filter.property);

    if (filter.operator === 'isnull' || filter.operator === 'isnotnull') {
      return null;
    }

    if (filter.operator === 'between') {
      const [min, max] = filter.value.split(',').map((v) => v.trim());
      return (
        <div className="flex gap-2 items-center">
          <input
            type={inputType}
            value={min || ''}
            onChange={(e) => {
              const maxVal = max || '';
              updateFilter(index, { value: `${e.target.value},${maxVal}` });
            }}
            placeholder="Min"
            className="w-32 px-2 py-1 border rounded"
          />
          <span className="text-gray-500">to</span>
          <input
            type={inputType}
            value={max || ''}
            onChange={(e) => {
              const minVal = min || '';
              updateFilter(index, { value: `${minVal},${e.target.value}` });
            }}
            placeholder="Max"
            className="w-32 px-2 py-1 border rounded"
          />
        </div>
      );
    }

    if (filter.operator === 'in' || filter.operator === 'notin') {
      return (
        <input
          type="text"
          value={filter.value}
          onChange={(e) => updateFilter(index, { value: e.target.value })}
          placeholder="Comma-separated values"
          className="flex-1 px-2 py-1 border rounded"
        />
      );
    }

    return (
      <input
        type={inputType}
        value={filter.value}
        onChange={(e) => updateFilter(index, { value: e.target.value })}
        placeholder="Value"
        className="flex-1 px-2 py-1 border rounded"
      />
    );
  };

  return (
    <div className="filter-builder space-y-2">
      <div className="flex justify-between items-center mb-2">
        <h3 className="font-semibold">Filters</h3>
        <div className="flex gap-2">
          {quickFilters.length > 0 && (
            <div className="relative">
              <button
                type="button"
                onClick={() => setShowQuickFilters(!showQuickFilters)}
                className="px-3 py-1 text-sm bg-blue-100 text-blue-700 rounded hover:bg-blue-200 flex items-center gap-1"
              >
                <BoltIcon className="w-4 h-4" />
                Quick Filters
              </button>
              {showQuickFilters && (
                <div className="absolute right-0 mt-1 bg-white border rounded shadow-lg z-10 min-w-48">
                  {quickFilters.map((qf, idx) => (
                    <button
                      key={idx}
                      type="button"
                      onClick={() => applyQuickFilter(qf)}
                      className="w-full text-left px-3 py-2 hover:bg-gray-50"
                    >
                      {qf.label}
                    </button>
                  ))}
                </div>
              )}
            </div>
          )}
          {enableSavedPresets && (
            <div className="relative">
              <button
                type="button"
                onClick={() => setShowPresets(!showPresets)}
                className="px-3 py-1 text-sm bg-gray-200 rounded hover:bg-gray-300 flex items-center gap-1"
              >
                <BookmarkIcon className="w-4 h-4" />
                Presets
              </button>
              {showPresets && (
                <div className="absolute right-0 mt-1 bg-white border rounded shadow-lg z-10 min-w-64 p-2">
                  {savedPresets.length > 0 && (
                    <div className="mb-2 pb-2 border-b">
                      <div className="text-xs font-semibold text-gray-500 mb-1">Saved Presets</div>
                      {savedPresets.map((preset, idx) => (
                        <button
                          key={idx}
                          type="button"
                          onClick={() => loadPreset(preset)}
                          className="w-full text-left px-2 py-1 text-sm hover:bg-gray-50 rounded"
                        >
                          {preset.name}
                        </button>
                      ))}
                    </div>
                  )}
                  {onSavePreset && (
                    <div>
                      <input
                        type="text"
                        value={presetName}
                        onChange={(e) => setPresetName(e.target.value)}
                        placeholder="Preset name"
                        className="w-full px-2 py-1 text-sm border rounded mb-2"
                      />
                      <button
                        type="button"
                        onClick={savePreset}
                        disabled={!presetName || filters.length === 0}
                        className="w-full px-2 py-1 text-sm bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-300"
                      >
                        Save Current Filters
                      </button>
                    </div>
                  )}
                </div>
              )}
            </div>
          )}
          <button
            type="button"
            onClick={addFilter}
            className="px-3 py-1 text-sm bg-gray-200 rounded hover:bg-gray-300"
          >
            Add Filter
          </button>
        </div>
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

          {renderValueInput(filter, index)}

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



