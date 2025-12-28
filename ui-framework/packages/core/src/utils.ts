/**
 * Utility functions for handling GraphQL responses
 * These functions provide backward compatibility for properties that may be
 * returned as either JSON strings or JSON objects
 */

/**
 * Safely parse properties from GraphQL response
 * Handles both string (legacy) and object (new) formats
 */
export function parseProperties(properties: string | object | null | undefined): Record<string, any> {
  if (!properties) {
    return {};
  }
  
  // If it's already an object, return it
  if (typeof properties === 'object' && !Array.isArray(properties)) {
    return properties as Record<string, any>;
  }
  
  // If it's a string, parse it
  if (typeof properties === 'string') {
    try {
      return JSON.parse(properties);
    } catch (e) {
      console.warn('Failed to parse properties as JSON:', e);
      return {};
    }
  }
  
  return {};
}

/**
 * Check if properties is a string (legacy format)
 */
export function isPropertiesString(properties: any): properties is string {
  return typeof properties === 'string';
}

/**
 * Get properties as an object, handling both formats
 */
export function getPropertiesAsObject(properties: any): Record<string, any> {
  return parseProperties(properties);
}

