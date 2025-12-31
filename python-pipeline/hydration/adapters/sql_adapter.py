"""SQL database adapter."""
from typing import Iterator, Dict, Any, Optional
from sqlalchemy import create_engine, text, inspect
from sqlalchemy.engine import Engine
from .base import SourceAdapter


class SQLAdapter(SourceAdapter):
    """Adapter for SQL databases (PostgreSQL, MySQL, etc.)."""
    
    def __init__(self, connection_string: str, table_name: str):
        """
        Initialize SQL adapter.
        
        Args:
            connection_string: SQLAlchemy connection string
            table_name: Name of the table to read from
        """
        self.connection_string = connection_string
        self.table_name = table_name
        self.engine: Optional[Engine] = None
        self._schema: Optional[Dict[str, str]] = None
    
    def connect(self) -> None:
        """Establish database connection."""
        self.engine = create_engine(self.connection_string)
        # Test connection
        with self.engine.connect() as conn:
            conn.execute(text("SELECT 1"))
    
    def disconnect(self) -> None:
        """Close database connection."""
        if self.engine:
            self.engine.dispose()
            self.engine = None
    
    def read_rows(self, limit: Optional[int] = None) -> Iterator[Dict[str, Any]]:
        """Read rows from the SQL table."""
        if not self.engine:
            raise RuntimeError("Adapter not connected. Call connect() first.")
        
        query = f"SELECT * FROM {self.table_name}"
        if limit:
            query = f"{query} LIMIT {limit}"
        
        with self.engine.connect() as conn:
            result = conn.execute(text(query))
            columns = result.keys()
            for row in result:
                yield dict(zip(columns, row))
    
    def get_schema(self) -> Dict[str, str]:
        """Get the schema of the SQL table."""
        if self._schema:
            return self._schema
        
        if not self.engine:
            raise RuntimeError("Adapter not connected. Call connect() first.")
        
        inspector = inspect(self.engine)
        columns = inspector.get_columns(self.table_name)
        
        self._schema = {
            col['name']: str(col['type']) for col in columns
        }
        return self._schema





