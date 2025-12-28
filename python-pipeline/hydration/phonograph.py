"""High-throughput ingestion service (the 'Phonograph')."""
from typing import Optional, Callable, Dict, Any
import asyncio
import logging
import json
from .schema_mapper import SchemaMapper, SchemaMapping
from .adapters.base import SourceAdapter

logger = logging.getLogger(__name__)

try:
    from confluent_kafka import Consumer, KafkaError, KafkaException
    KAFKA_AVAILABLE = True
except ImportError:
    KAFKA_AVAILABLE = False
    logger.warning("confluent-kafka not available. Kafka streaming will be disabled.")


class Phonograph:
    """
    High-throughput data ingestion service.
    
    Reads from source adapters, maps data using schema mappings,
    and sends transformed objects to the indexing layer.
    """
    
    def __init__(
        self,
        schema_mapper: SchemaMapper,
        sink_callback: Optional[Callable[[str, str, Dict[str, Any]], None]] = None
    ):
        """
        Initialize Phonograph.
        
        Args:
            schema_mapper: Schema mapper instance
            sink_callback: Optional callback function for processed objects
                          Signature: (object_type, object_id, properties) -> None
        """
        self.schema_mapper = schema_mapper
        self.sink_callback = sink_callback
        self.running = False
    
    def ingest_from_source(
        self,
        source_adapter: SourceAdapter,
        mapping: SchemaMapping,
        batch_size: int = 1000,
        limit: Optional[int] = None
    ) -> int:
        """
        Ingest data from a source adapter.
        
        Args:
            source_adapter: Source adapter instance
            mapping: Schema mapping to use
            batch_size: Number of rows to process in each batch
            limit: Optional limit on total rows to process
            
        Returns:
            Number of objects ingested
        """
        try:
            source_adapter.connect()
            
            batch = []
            total_ingested = 0
            row_count = 0
            
            for source_row in source_adapter.read_rows(limit=limit):
                # Map source row to ontology properties
                properties = self.schema_mapper.map_row(mapping, source_row)
                primary_key = self.schema_mapper.extract_primary_key(mapping, source_row)
                
                # Add to batch
                batch.append((mapping.object_type_id, primary_key, properties))
                
                # Process batch when full
                if len(batch) >= batch_size:
                    self._process_batch(batch)
                    total_ingested += len(batch)
                    batch = []
                    logger.info(f"Ingested batch: {total_ingested} objects so far")
                
                row_count += 1
            
            # Process remaining batch
            if batch:
                self._process_batch(batch)
                total_ingested += len(batch)
            
            logger.info(f"Ingestion complete: {total_ingested} objects from {row_count} rows")
            return total_ingested
            
        except Exception as e:
            logger.error(f"Error during ingestion: {e}", exc_info=True)
            raise
        finally:
            source_adapter.disconnect()
    
    def _process_batch(self, batch: list):
        """Process a batch of objects."""
        for object_type, object_id, properties in batch:
            try:
                if self.sink_callback:
                    self.sink_callback(object_type, object_id, properties)
                else:
                    # Default: log the object
                    logger.debug(f"Object: {object_type}/{object_id} with {len(properties)} properties")
            except Exception as e:
                logger.error(f"Error processing object {object_type}/{object_id}: {e}", exc_info=True)
                # Continue with next object
    
    async def ingest_from_source_async(
        self,
        source_adapter: SourceAdapter,
        mapping: SchemaMapping,
        batch_size: int = 1000,
        limit: Optional[int] = None
    ) -> int:
        """Async version of ingest_from_source."""
        # Run synchronous ingestion in executor
        loop = asyncio.get_event_loop()
        return await loop.run_in_executor(
            None,
            self.ingest_from_source,
            source_adapter,
            mapping,
            batch_size,
            limit
        )


class StreamingPhonograph(Phonograph):
    """
    Streaming version of Phonograph for Kafka/event streams.
    """
    
    async def stream_from_kafka(
        self,
        topic: str,
        mapping: SchemaMapping,
        kafka_config: Dict[str, Any]
    ):
        """
        Stream data from Kafka topic.
        
        Args:
            topic: Kafka topic name
            mapping: Schema mapping to use
            kafka_config: Kafka consumer configuration (bootstrap.servers, group.id, etc.)
        """
        if not KAFKA_AVAILABLE:
            raise RuntimeError("confluent-kafka is not installed. Install it with: pip install confluent-kafka")
        
        # Set up default consumer config
        consumer_config = {
            'bootstrap.servers': kafka_config.get('bootstrap.servers', 'localhost:9092'),
            'group.id': kafka_config.get('group.id', 'ontology_group'),
            'auto.offset.reset': kafka_config.get('auto.offset.reset', 'earliest'),
            'enable.auto.commit': kafka_config.get('enable.auto.commit', True),
        }
        
        # Add any additional config
        consumer_config.update({k: v for k, v in kafka_config.items() 
                               if k not in ['bootstrap.servers', 'group.id', 'auto.offset.reset', 'enable.auto.commit']})
        
        consumer = Consumer(consumer_config)
        consumer.subscribe([topic])
        
        self.running = True
        logger.info(f"Started Kafka consumer for topic: {topic}")
        
        try:
            while self.running:
                # Poll for messages (non-blocking)
                msg = consumer.poll(timeout=1.0)
                
                if msg is None:
                    continue
                
                if msg.error():
                    if msg.error().code() == KafkaError._PARTITION_EOF:
                        # End of partition event - not an error
                        logger.debug(f"Reached end of partition: {msg.topic()}[{msg.partition()}]")
                        continue
                    else:
                        logger.error(f"Consumer error: {msg.error()}")
                        continue
                
                try:
                    # Parse message value (assumed to be JSON)
                    message_data = json.loads(msg.value().decode('utf-8'))
                    
                    # Map the message data to ontology properties
                    properties = self.schema_mapper.map_row(mapping, message_data)
                    primary_key = self.schema_mapper.extract_primary_key(mapping, message_data)
                    
                    # Process the object
                    if self.sink_callback:
                        self.sink_callback(mapping.object_type_id, primary_key, properties)
                    else:
                        logger.debug(f"Processed message: {mapping.object_type_id}/{primary_key}")
                    
                    logger.debug(f"Processed message from {msg.topic()}[{msg.partition()}] offset {msg.offset()}")
                    
                except json.JSONDecodeError as e:
                    logger.error(f"Failed to parse message as JSON: {e}")
                    continue
                except Exception as e:
                    logger.error(f"Error processing message: {e}", exc_info=True)
                    continue
                    
        except KeyboardInterrupt:
            logger.info("Received interrupt signal, shutting down...")
        except Exception as e:
            logger.error(f"Error in Kafka consumer: {e}", exc_info=True)
            raise
        finally:
            self.running = False
            consumer.close()
            logger.info("Kafka consumer closed")
    
    def stop_streaming(self):
        """Stop the Kafka streaming loop."""
        self.running = False
        logger.info("Stopping Kafka consumer...")




