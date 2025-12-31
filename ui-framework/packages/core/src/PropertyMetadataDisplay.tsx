import React from 'react';

export interface PropertyMetadata {
  id: string;
  displayName?: string;
  description?: string;
  unit?: string;
  format?: {
    type: string;
    [key: string]: any;
  };
  type: string;
  required?: boolean;
}

export interface PropertyMetadataDisplayProps {
  property: PropertyMetadata;
  value: any;
  className?: string;
}

function formatValue(value: any, format?: PropertyMetadata['format']): string {
  if (value == null) return '—';

  if (!format) return String(value);

  switch (format.type) {
    case 'currency':
      const symbol = format.symbol || '$';
      return `${symbol}${Number(value).toLocaleString('en-US', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
      })}`;
    case 'percentage':
      const decimals = format.decimals ?? 2;
      return `${(Number(value) * 100).toFixed(decimals)}%`;
    case 'number_format':
      const numDecimals = format.decimals ?? 0;
      const separator = format.separator;
      let formatted = Number(value).toLocaleString('en-US', {
        minimumFractionDigits: numDecimals,
        maximumFractionDigits: numDecimals,
      });
      if (separator) {
        formatted = formatted.replace(/,/g, separator);
      }
      return formatted;
    case 'date_format':
      // For date formatting, we'd need a date library, but for now just return as-is
      return String(value);
    default:
      return String(value);
  }
}

export function PropertyMetadataDisplay({
  property,
  value,
  className = '',
}: PropertyMetadataDisplayProps) {
  const formattedValue = formatValue(value, property.format);
  const displayName = property.displayName || property.id;

  return (
    <div className={`property-metadata ${className}`}>
      <div className="flex items-start justify-between">
        <div className="flex-1">
          <div className="flex items-center gap-2">
            <span className="font-medium text-gray-900">{displayName}</span>
            {property.required && (
              <span className="text-xs text-red-500 font-semibold">*</span>
            )}
            {property.unit && (
              <span className="text-xs text-gray-500">({property.unit})</span>
            )}
          </div>
          {property.description && (
            <div className="text-sm text-gray-600 mt-1">{property.description}</div>
          )}
          <div className="text-xs text-gray-400 mt-1">
            Type: <span className="font-mono">{property.type}</span>
          </div>
        </div>
        <div className="text-right">
          <div className="text-base font-medium text-gray-900">{formattedValue}</div>
        </div>
      </div>
    </div>
  );
}


