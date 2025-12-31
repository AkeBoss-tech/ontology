"""Base adapter interface for source data."""
from abc import ABC, abstractmethod
from typing import Iterator, Dict, Any, Optional


class SourceAdapter(ABC):
    """Abstract base class for source data adapters."""
    
    @abstractmethod
    def connect(self) -> None:
        """Establish connection to the data source."""
        pass
    
    @abstractmethod
    def disconnect(self) -> None:
        """Close connection to the data source."""
        pass
    
    @abstractmethod
    def read_rows(self, limit: Optional[int] = None) -> Iterator[Dict[str, Any]]:
        """
        Read rows from the source.
        
        Args:
            limit: Optional limit on number of rows to read
            
        Yields:
            Dictionary representing a row of data
        """
        pass
    
    @abstractmethod
    def get_schema(self) -> Dict[str, str]:
        """
        Get the schema of the source data.
        
        Returns:
            Dictionary mapping column names to types
        """
        pass





