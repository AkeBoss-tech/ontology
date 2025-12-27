use crate::store::{StoreBackend, IndexedObject, StoreError};
use ontology_engine::PropertyMap;
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Sync service that maintains consistency across search, graph, and columnar stores
pub struct SyncService {
    backend: Arc<StoreBackend>,
    event_tx: mpsc::Sender<SyncEvent>,
    event_rx: Option<mpsc::Receiver<SyncEvent>>,
}

/// Events that trigger sync operations
#[derive(Debug, Clone)]
pub enum SyncEvent {
    ObjectCreated {
        object_type: String,
        object_id: String,
        properties: PropertyMap,
    },
    ObjectUpdated {
        object_type: String,
        object_id: String,
        properties: PropertyMap,
    },
    ObjectDeleted {
        object_type: String,
        object_id: String,
    },
    LinkCreated {
        link_type_id: String,
        source_id: String,
        target_id: String,
        properties: PropertyMap,
    },
    LinkDeleted {
        link_id: String,
    },
}

impl SyncService {
    /// Create a new sync service
    pub fn new(backend: Arc<StoreBackend>) -> Self {
        let (tx, rx) = mpsc::channel(1000);
        Self {
            backend,
            event_tx: tx,
            event_rx: Some(rx),
        }
    }
    
    /// Get the event sender for external components
    pub fn event_sender(&self) -> mpsc::Sender<SyncEvent> {
        self.event_tx.clone()
    }
    
    /// Start the sync service loop
    pub async fn start(&mut self) -> Result<(), StoreError> {
        let mut rx = self.event_rx.take()
            .ok_or_else(|| StoreError::Unknown("Sync service already started".to_string()))?;
        
        let backend = Arc::clone(&self.backend);
        
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                if let Err(e) = Self::handle_event(&backend, event).await {
                    eprintln!("Error handling sync event: {}", e);
                    // In production, might want to retry or queue for later
                }
            }
        });
        
        Ok(())
    }
    
    /// Handle a sync event and update all stores
    async fn handle_event(
        backend: &StoreBackend,
        event: SyncEvent,
    ) -> Result<(), StoreError> {
        match event {
            SyncEvent::ObjectCreated { object_type, object_id, properties } => {
                // Update search index
                backend.search_store()
                    .index_object(&object_type, &object_id, &properties)
                    .await?;
                
                // Write to columnar store (in batch, but for now individual)
                let indexed_obj = IndexedObject::new(
                    object_type.clone(),
                    object_id.clone(),
                    properties.clone(),
                );
                backend.columnar_store()
                    .write_batch(&object_type, vec![indexed_obj])
                    .await?;
                
                Ok(())
            }
            SyncEvent::ObjectUpdated { object_type, object_id, properties } => {
                // Update search index
                backend.search_store()
                    .index_object(&object_type, &object_id, &properties)
                    .await?;
                
                // Update columnar store
                let indexed_obj = IndexedObject::new(
                    object_type.clone(),
                    object_id.clone(),
                    properties.clone(),
                );
                backend.columnar_store()
                    .write_batch(&object_type, vec![indexed_obj])
                    .await?;
                
                Ok(())
            }
            SyncEvent::ObjectDeleted { object_type, object_id } => {
                // Remove from search index
                backend.search_store()
                    .delete_object(&object_type, &object_id)
                    .await?;
                
                // Note: Columnar stores typically don't delete - they append new records
                // with deletion markers, or rely on time-based partitioning
                
                Ok(())
            }
            SyncEvent::LinkCreated { link_type_id, source_id, target_id, properties } => {
                // Create link in graph store
                backend.graph_store()
                    .create_link(&link_type_id, &source_id, &target_id, &properties)
                    .await?;
                
                Ok(())
            }
            SyncEvent::LinkDeleted { link_id } => {
                // Delete link from graph store
                backend.graph_store()
                    .delete_link(&link_id)
                    .await?;
                
                Ok(())
            }
        }
    }
    
    /// Sync an object to all stores (transactional)
    pub async fn sync_object(
        &self,
        object_type: &str,
        object_id: &str,
        properties: &PropertyMap,
    ) -> Result<(), StoreError> {
        // Create indexed object
        let indexed_obj = IndexedObject::new(
            object_type.to_string(),
            object_id.to_string(),
            properties.clone(),
        );
        
        // Update search index
        self.backend.search_store()
            .index_object(object_type, object_id, properties)
            .await?;
        
        // Update columnar store
        self.backend.columnar_store()
            .write_batch(object_type, vec![indexed_obj])
            .await?;
        
        // Note: In a production system, you might want to:
        // 1. Use distributed transactions (2PC, Saga pattern, etc.)
        // 2. Implement retry logic with exponential backoff
        // 3. Add idempotency keys
        // 4. Use event sourcing for eventual consistency
        
        Ok(())
    }
    
    /// Sync a link to the graph store
    pub async fn sync_link(
        &self,
        link_type_id: &str,
        source_id: &str,
        target_id: &str,
        properties: &PropertyMap,
    ) -> Result<String, StoreError> {
        self.backend.graph_store()
            .create_link(link_type_id, source_id, target_id, properties)
            .await
    }
}

