import React from 'react';
import { ChevronRightIcon, HomeIcon } from '@heroicons/react/24/outline';

export interface BreadcrumbItem {
  label: string;
  path?: string;
  onClick?: () => void;
}

export interface BreadcrumbNavProps {
  items: BreadcrumbItem[];
  onHomeClick?: () => void;
  className?: string;
}

export function BreadcrumbNav({ items, onHomeClick, className = '' }: BreadcrumbNavProps) {
  return (
    <nav className={`breadcrumb-nav flex items-center space-x-2 text-sm ${className}`}>
      {onHomeClick && (
        <>
          <button
            onClick={onHomeClick}
            className="flex items-center text-gray-500 hover:text-gray-700"
            aria-label="Home"
          >
            <HomeIcon className="w-4 h-4" />
          </button>
          {items.length > 0 && (
            <ChevronRightIcon className="w-4 h-4 text-gray-400" />
          )}
        </>
      )}
      {items.map((item, index) => (
        <React.Fragment key={index}>
          {item.onClick || item.path ? (
            <button
              onClick={item.onClick}
              className="text-blue-600 hover:text-blue-800 hover:underline"
            >
              {item.label}
            </button>
          ) : (
            <span className="text-gray-700 font-medium">{item.label}</span>
          )}
          {index < items.length - 1 && (
            <ChevronRightIcon className="w-4 h-4 text-gray-400" />
          )}
        </React.Fragment>
      ))}
    </nav>
  );
}

