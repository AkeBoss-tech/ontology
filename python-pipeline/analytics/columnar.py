"""Columnar analytics helpers for Parquet/S3 data."""
import pyarrow.parquet as pq
import pyarrow as pa
import pandas as pd
import numpy as np
from typing import Dict, Any, List, Optional
from pathlib import Path
try:
    from scipy import stats
    SCIPY_AVAILABLE = True
except ImportError:
    SCIPY_AVAILABLE = False


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
    
    def compute_distribution(
        self,
        object_type: str,
        column: str,
        bins: int = 10
    ) -> Dict[str, Any]:
        """
        Compute distribution statistics (histogram, percentiles).
        
        Args:
            object_type: Object type to query
            column: Column name
            bins: Number of bins for histogram
            
        Returns:
            Dictionary with distribution statistics
        """
        df = self.read_parquet_table(object_type)
        
        if column not in df.columns:
            raise ValueError(f"Column '{column}' not found in {object_type}")
        
        col_data = df[column].dropna()
        
        if len(col_data) == 0:
            return {
                'histogram': {'bins': {}, 'bin_edges': []},
                'percentiles': {},
                'skewness': None,
                'kurtosis': None,
            }
        
        # Only compute histogram for numeric data
        histogram_data = {}
        bin_edges = []
        if pd.api.types.is_numeric_dtype(col_data):
            hist, bin_edges = np.histogram(col_data, bins=bins)
            histogram_data = {
                'bins': {f"{bin_edges[i]:.2f}-{bin_edges[i+1]:.2f}": int(hist[i]) 
                        for i in range(len(hist))},
                'bin_edges': [float(edge) for edge in bin_edges]
            }
        
        return {
            'histogram': histogram_data,
            'percentiles': {
                'p25': float(col_data.quantile(0.25)),
                'p50': float(col_data.quantile(0.50)),
                'p75': float(col_data.quantile(0.75)),
                'p90': float(col_data.quantile(0.90)),
                'p95': float(col_data.quantile(0.95)),
                'p99': float(col_data.quantile(0.99))
            },
            'skewness': float(col_data.skew()) if pd.api.types.is_numeric_dtype(col_data) else None,
            'kurtosis': float(col_data.kurtosis()) if pd.api.types.is_numeric_dtype(col_data) else None,
        }
    
    def compute_correlations(
        self,
        object_type: str,
        columns: List[str]
    ) -> Dict[str, Dict[str, float]]:
        """
        Compute correlation matrix between numeric columns.
        
        Args:
            object_type: Object type to query
            columns: List of column names to correlate
            
        Returns:
            Nested dictionary with correlation values
        """
        df = self.read_parquet_table(object_type)
        
        # Filter to only numeric columns
        numeric_df = df[columns].select_dtypes(include=[np.number])
        
        if numeric_df.empty:
            return {}
        
        corr_matrix = numeric_df.corr()
        
        # Convert to nested dictionary
        result = {}
        for col1 in corr_matrix.columns:
            result[col1] = {}
            for col2 in corr_matrix.columns:
                result[col1][col2] = float(corr_matrix.loc[col1, col2])
        
        return result
    
    def detect_outliers(
        self,
        object_type: str,
        column: str,
        method: str = "iqr"  # "iqr" or "zscore"
    ) -> pd.DataFrame:
        """
        Detect outliers using IQR or Z-score method.
        
        Args:
            object_type: Object type to query
            column: Column name
            method: "iqr" or "zscore"
            
        Returns:
            DataFrame with outlier rows
        """
        df = self.read_parquet_table(object_type)
        
        if column not in df.columns:
            raise ValueError(f"Column '{column}' not found in {object_type}")
        
        col_data = df[column]
        
        if not pd.api.types.is_numeric_dtype(col_data):
            raise ValueError(f"Column '{column}' must be numeric for outlier detection")
        
        if method == "iqr":
            Q1 = col_data.quantile(0.25)
            Q3 = col_data.quantile(0.75)
            IQR = Q3 - Q1
            outliers = df[(col_data < (Q1 - 1.5 * IQR)) | (col_data > (Q3 + 1.5 * IQR))]
        elif method == "zscore":
            if not SCIPY_AVAILABLE:
                raise ImportError("scipy is required for zscore outlier detection")
            z_scores = np.abs(stats.zscore(col_data.dropna()))
            outlier_indices = np.where(z_scores > 3)[0]
            outliers = df.iloc[outlier_indices]
        else:
            raise ValueError(f"Unknown method: {method}. Use 'iqr' or 'zscore'")
        
        return outliers
    
    def time_series_analysis(
        self,
        object_type: str,
        value_column: str,
        time_column: str,
        frequency: str = "D"  # Daily
    ) -> Dict[str, Any]:
        """
        Perform time series analysis (trends, seasonality, etc.).
        
        Args:
            object_type: Object type to query
            value_column: Column with values to analyze
            time_column: Column with timestamps
            frequency: Resampling frequency ("D"=daily, "M"=monthly, "Y"=yearly)
            
        Returns:
            Dictionary with time series analysis results
        """
        df = self.read_parquet_table(object_type)
        
        if value_column not in df.columns or time_column not in df.columns:
            raise ValueError(f"Columns not found in {object_type}")
        
        df[time_column] = pd.to_datetime(df[time_column])
        df = df.set_index(time_column).sort_index()
        
        ts = df[value_column]
        
        if len(ts) < 2:
            return {
                'trend': {'slope': 0.0, 'direction': 'unknown'},
                'seasonality': {},
                'volatility': 0.0,
                'growth_rate': 0.0
            }
        
        # Compute trend
        x = np.arange(len(ts))
        y = ts.values
        slope = float(np.polyfit(x, y, 1)[0])
        direction = 'increasing' if ts.iloc[-1] > ts.iloc[0] else 'decreasing'
        
        # Compute seasonality
        monthly = ts.resample('M').mean()
        yearly = ts.resample('Y').mean()
        
        # Compute volatility (coefficient of variation)
        volatility = float(ts.pct_change().std()) if ts.pct_change().std() > 0 else 0.0
        
        # Compute growth rate
        growth_rate = float((ts.iloc[-1] / ts.iloc[0] - 1) * 100) if ts.iloc[0] != 0 else 0.0
        
        return {
            'trend': {
                'slope': slope,
                'direction': direction
            },
            'seasonality': {
                'monthly': {str(k): float(v) for k, v in monthly.items()},
                'yearly': {str(k): float(v) for k, v in yearly.items()}
            },
            'volatility': volatility,
            'growth_rate': growth_rate
        }






