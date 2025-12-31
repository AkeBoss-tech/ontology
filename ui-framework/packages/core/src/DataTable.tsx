import React, { useState, useMemo } from 'react';
import { ChevronUpIcon, ChevronDownIcon, ArrowDownTrayIcon } from '@heroicons/react/24/outline';

export interface Column<T> {
  key: string;
  label: string;
  sortable?: boolean;
  render?: (value: any, row: T) => React.ReactNode;
  width?: string;
}

export interface DataTableProps<T> {
  data: T[];
  columns: Column<T>[];
  keyExtractor: (row: T) => string;
  onRowClick?: (row: T) => void;
  defaultSort?: { key: string; direction: 'asc' | 'desc' };
  filterable?: boolean;
  exportable?: boolean;
  exportFilename?: string;
  className?: string;
}

type SortState = {
  key: string;
  direction: 'asc' | 'desc';
};

export function DataTable<T extends Record<string, any>>({
  data,
  columns,
  keyExtractor,
  onRowClick,
  defaultSort,
  filterable = false,
  exportable = false,
  exportFilename = 'export',
  className = '',
}: DataTableProps<T>) {
  const [sortState, setSortState] = useState<SortState | null>(
    defaultSort ? { key: defaultSort.key, direction: defaultSort.direction } : null
  );
  const [filters, setFilters] = useState<Record<string, string>>({});
  const [showFilters, setShowFilters] = useState(false);

  // Apply filters
  const filteredData = useMemo(() => {
    if (!filterable || Object.keys(filters).length === 0) return data;

    return data.filter((row) => {
      return columns.every((col) => {
        const filterValue = filters[col.key];
        if (!filterValue) return true;

        const cellValue = row[col.key];
        const searchStr = String(cellValue ?? '').toLowerCase();
        return searchStr.includes(filterValue.toLowerCase());
      });
    });
  }, [data, filters, columns, filterable]);

  // Apply sorting
  const sortedData = useMemo(() => {
    if (!sortState) return filteredData;

    return [...filteredData].sort((a, b) => {
      const aVal = a[sortState.key];
      const bVal = b[sortState.key];

      // Handle null/undefined
      if (aVal == null && bVal == null) return 0;
      if (aVal == null) return 1;
      if (bVal == null) return -1;

      // Compare values
      let comparison = 0;
      if (typeof aVal === 'number' && typeof bVal === 'number') {
        comparison = aVal - bVal;
      } else {
        comparison = String(aVal).localeCompare(String(bVal));
      }

      return sortState.direction === 'asc' ? comparison : -comparison;
    });
  }, [filteredData, sortState]);

  const handleSort = (key: string) => {
    if (sortState?.key === key) {
      // Toggle direction
      setSortState({
        key,
        direction: sortState.direction === 'asc' ? 'desc' : 'asc',
      });
    } else {
      // New sort
      setSortState({ key, direction: 'asc' });
    }
  };

  const handleExport = () => {
    // Convert to CSV
    const headers = columns.map((col) => col.label);
    const rows = sortedData.map((row) =>
      columns.map((col) => {
        const value = row[col.key];
        // Escape commas and quotes in CSV
        const str = String(value ?? '');
        if (str.includes(',') || str.includes('"') || str.includes('\n')) {
          return `"${str.replace(/"/g, '""')}"`;
        }
        return str;
      })
    );

    const csv = [headers.join(','), ...rows.map((r) => r.join(','))].join('\n');
    const blob = new Blob([csv], { type: 'text/csv' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `${exportFilename}.csv`;
    a.click();
    URL.revokeObjectURL(url);
  };

  return (
    <div className={`data-table ${className}`}>
      <div className="flex justify-between items-center mb-4">
        <div className="text-sm text-gray-600">
          Showing {sortedData.length} of {data.length} rows
        </div>
        <div className="flex gap-2">
          {filterable && (
            <button
              onClick={() => setShowFilters(!showFilters)}
              className="px-3 py-1 text-sm border rounded hover:bg-gray-50"
            >
              {showFilters ? 'Hide' : 'Show'} Filters
            </button>
          )}
          {exportable && (
            <button
              onClick={handleExport}
              className="px-3 py-1 text-sm border rounded hover:bg-gray-50 flex items-center gap-1"
            >
              <ArrowDownTrayIcon className="w-4 h-4" />
              Export CSV
            </button>
          )}
        </div>
      </div>

      {showFilters && filterable && (
        <div className="mb-4 p-4 bg-gray-50 rounded border">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
            {columns.map((col) => (
              <div key={col.key}>
                <label className="block text-sm font-medium mb-1">{col.label}</label>
                <input
                  type="text"
                  value={filters[col.key] || ''}
                  onChange={(e) =>
                    setFilters({ ...filters, [col.key]: e.target.value })
                  }
                  placeholder="Filter..."
                  className="w-full px-2 py-1 border rounded text-sm"
                />
              </div>
            ))}
          </div>
        </div>
      )}

      <div className="overflow-x-auto border rounded">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              {columns.map((col) => (
                <th
                  key={col.key}
                  style={{ width: col.width }}
                  className={`px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider ${
                    col.sortable !== false ? 'cursor-pointer hover:bg-gray-100' : ''
                  }`}
                  onClick={() => col.sortable !== false && handleSort(col.key)}
                >
                  <div className="flex items-center gap-2">
                    <span>{col.label}</span>
                    {col.sortable !== false && sortState?.key === col.key && (
                      <span>
                        {sortState.direction === 'asc' ? (
                          <ChevronUpIcon className="w-4 h-4" />
                        ) : (
                          <ChevronDownIcon className="w-4 h-4" />
                        )}
                      </span>
                    )}
                  </div>
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {sortedData.length === 0 ? (
              <tr>
                <td colSpan={columns.length} className="px-6 py-4 text-center text-gray-500">
                  No data available
                </td>
              </tr>
            ) : (
              sortedData.map((row) => (
                <tr
                  key={keyExtractor(row)}
                  onClick={() => onRowClick?.(row)}
                  className={onRowClick ? 'cursor-pointer hover:bg-gray-50' : ''}
                >
                  {columns.map((col) => (
                    <td key={col.key} className="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                      {col.render ? col.render(row[col.key], row) : String(row[col.key] ?? '')}
                    </td>
                  ))}
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}


