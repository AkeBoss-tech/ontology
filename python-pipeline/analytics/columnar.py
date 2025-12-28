"""Columnar analytics helpers for Parquet/S3 data."""
import pyarrow.parquet as pq
import pyarrow as pa
import pandas as pd
from typing import Dict, Any, List, Optional
from pathlib import Path


class ColumnarAnalytics:
    """Helper class for columnar analytics operations."""
    
    def __init__(self, base_path: str):
        """
        Initialize columnar analytics.
        
        Args:
            base_path: Base path for Parquet files (local or S3)
        """
        self.base_path = Path(base_path)
    
    def read_parquet_table(self, object_type: str) -> pd.DataFrame:
        """
        Read a Parquet table for an object type.
        
        Args:
            object_type: Object type identifier
            
        Returns:
            pandas DataFrame
        """
        file_path = self.base_path / f"{object_type}.parquet"
        if not file_path.exists():
            raise FileNotFoundError(f"Parquet file not found: {file_path}")
        
        return pd.read_parquet(file_path)
    
    def aggregate(
        self,
        object_type: str,
        aggregations: Dict[str, str],
        filters: Optional[Dict[str, Any]] = None,
        group_by: Optional[List[str]] = None
    ) -> pd.DataFrame:
        """
        Perform aggregations on columnar data.
        
        Args:
            object_type: Object type to query
            aggregations: Dictionary mapping column names to aggregation functions
                         e.g., {"cost": "sum", "count": "count"}
            filters: Optional filters to apply
            group_by: Optional list of columns to group by
            
        Returns:
            Aggregated pandas DataFrame
        """
        df = self.read_parquet_table(object_type)
        
        # Apply filters
        if filters:
            for column, value in filters.items():
                if column in df.columns:
                    df = df[df[column] == value]
        
        # Group by if specified
        if group_by:
            grouped = df.groupby(group_by)
            result = grouped.agg(aggregations)
        else:
            result = df.agg(aggregations)
        
        return result
    
    def compute_statistics(self, object_type: str, column: str) -> Dict[str, float]:
        """
        Compute statistics for a column.
        
        Args:
            object_type: Object type to query
            column: Column name
            
        Returns:
            Dictionary with statistics (min, max, mean, std, etc.)
        """
        df = self.read_parquet_table(object_type)
        
        if column not in df.columns:
            raise ValueError(f"Column '{column}' not found in {object_type}")
        
        col_data = df[column]
        return {
            'min': float(col_data.min()),
            'max': float(col_data.max()),
            'mean': float(col_data.mean()),
            'std': float(col_data.std()),
            'count': int(col_data.count()),
        }




