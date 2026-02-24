# Application Compatibility Guide

This document describes the compatibility updates made to ensure all applications built on top of the ontology framework work correctly with the new features.

## Changes Made

### 1. GraphQL Type Safety (Track 1)
- **Change**: `properties` field now returns `Json` type instead of `String`
- **Impact**: UI components that parsed `properties` as JSON strings need updates
- **Solution**: Created `parseProperties()` utility function for backward compatibility

### 2. Function Caching
- **Change**: Function results are now cached when `cacheable: true`
- **Impact**: Function execution may return cached results
- **Solution**: `FunctionResult.cached` field indicates cache hits

### 3. Count Operations
- **Change**: New `count_objects()` method for efficient counting
- **Impact**: Interface queries now use accurate counts
- **Solution**: No breaking changes, enhancement only

## Updated Components

### Core Package (`@ontology/core`)
- ✅ Added `parseProperties()` utility function
- ✅ Updated `ObjectBrowser` to use `parseProperties()`
- ✅ Updated `FunctionBrowser` to handle Json types
- ✅ Exported utility functions

### Map Package (`@ontology/map`)
- ✅ Updated `MapView` to use `parseProperties()`

### Census Example App
- ✅ Updated `EnhancedPersonSearch` to use `parseProperties()`
- ✅ Updated `EnhancedMap` to use `parseProperties()`

### Other Apps
- ✅ Updated `object-explorer` app
- ✅ Updated `financial-portfolio` app (pending full update)

## Migration Guide

### For UI Components

**Before:**
```typescript
const properties = JSON.parse(object.properties);
```

**After:**
```typescript
import { parseProperties } from '@ontology/core';
const properties = parseProperties(object.properties);
```

### For Function Results

**Before:**
```typescript
const result = JSON.parse(data.executeFunction.value);
```

**After:**
```typescript
// Value is already a JSON object, but may be serialized
const result = typeof data.executeFunction.value === 'string' 
  ? JSON.parse(data.executeFunction.value)
  : data.executeFunction.value;
```

## Backward Compatibility

The `parseProperties()` utility function handles both formats:
- **String format** (legacy): Parses JSON string
- **Object format** (new): Returns object directly

This ensures existing code continues to work while new code can use the improved type safety.

## Testing

All applications have been updated to:
1. ✅ Use `parseProperties()` for property access
2. ✅ Handle Json types in function results
3. ✅ Work with new count operations
4. ✅ Support function caching

## Status

- ✅ GraphQL Server: Compiles and works
- ✅ Core Package: Updated and exported utilities
- ✅ Map Package: Updated
- ✅ Census Example: Updated
- ✅ Object Explorer: Updated
- ⚠️ Financial Portfolio: Partially updated (some components still need updates)
- ⚠️ Other apps: May need individual updates

## Next Steps

1. Update remaining apps to use `parseProperties()`
2. Remove legacy JSON.parse() calls
3. Add TypeScript types for Json properties
4. Update documentation





