"""CSV file adapter."""
from typing import Iterator, Dict, Any, Optional
import csv
import os
from .base import SourceAdapter


class CSVAdapter(SourceAdapter):
    """Adapter for CSV files."""
    
    def __init__(self, file_path: str, delimiter: str = ','):
        """
        Initialize CSV adapter.
        
        Args:
            file_path: Path to the CSV file
            delimiter: CSV delimiter character
        """
        self.file_path = file_path
        self.delimiter = delimiter
        self._file = None
        self._reader = None
        self._schema: Optional[Dict[str, str]] = None
    
    def connect(self) -> None:
        """Open the CSV file."""
        if not os.path.exists(self.file_path):
            raise FileNotFoundError(f"CSV file not found: {self.file_path}")
        
        self._file = open(self.file_path, 'r', encoding='utf-8')
        self._reader = csv.DictReader(self._file, delimiter=self.delimiter)
        
        # Infer schema from first row
        if self._reader.fieldnames:
            self._schema = {field: 'string' for field in self._reader.fieldnames}
    
    def disconnect(self) -> None:
        """Close the CSV file."""
        if self._file:
            self._file.close()
            self._file = None
            self._reader = None
    
    def read_rows(self, limit: Optional[int] = None) -> Iterator[Dict[str, Any]]:
        """Read rows from the CSV file."""
        if not self._reader:
            raise RuntimeError("Adapter not connected. Call connect() first.")
        
        count = 0
        for row in self._reader:
            if limit and count >= limit:
                break
            yield row
            count += 1
    
    def get_schema(self) -> Dict[str, str]:
        """Get the schema of the CSV file."""
        if not self._schema:
            raise RuntimeError("Adapter not connected. Call connect() first.")
        return self._schema



