use ontology_engine::PropertyMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::PgPool;

/// Write-back queue - stores user edits that overlay source data
pub struct WriteBackQueue {
    pool: PgPool,
}

/// A user edit record
#[derive(Debug, Clone)]
pub struct UserEdit {
    pub edit_id: String,
    pub object_type: String,
    pub object_id: String,
    pub property_name: String,
    pub property_value: ontology_engine::PropertyValue,
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
    pub deleted: bool, // True if this edit deletes the property
}

impl WriteBackQueue {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Initialize the database schema
    pub async fn initialize(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS user_edits (
                edit_id TEXT PRIMARY KEY,
                object_type TEXT NOT NULL,
                object_id TEXT NOT NULL,
                property_name TEXT NOT NULL,
                property_value JSONB NOT NULL,
                user_id TEXT NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                deleted BOOLEAN NOT NULL DEFAULT FALSE,
                UNIQUE(object_type, object_id, property_name)
            );
            CREATE INDEX IF NOT EXISTS idx_user_edits_object ON user_edits(object_type, object_id);
            CREATE INDEX IF NOT EXISTS idx_user_edits_timestamp ON user_edits(timestamp);
            "#,
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Record a user edit
    pub async fn record_edit(
        &self,
        object_type: &str,
        object_id: &str,
        property_name: &str,
        property_value: &ontology_engine::PropertyValue,
        user_id: &str,
    ) -> Result<String, sqlx::Error> {
        let edit_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        let json_value = serde_json::to_value(property_value)
            .map_err(|e| sqlx::Error::Decode(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Failed to serialize property value: {}", e)
            ))))?;
        
        sqlx::query(
            r#"
            INSERT INTO user_edits (edit_id, object_type, object_id, property_name, property_value, user_id, timestamp, deleted)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (object_type, object_id, property_name)
            DO UPDATE SET
                property_value = EXCLUDED.property_value,
                user_id = EXCLUDED.user_id,
                timestamp = EXCLUDED.timestamp,
                deleted = EXCLUDED.deleted
            "#,
        )
        .bind(&edit_id)
        .bind(object_type)
        .bind(object_id)
        .bind(property_name)
        .bind(json_value)
        .bind(user_id)
        .bind(timestamp)
        .bind(false)
        .execute(&self.pool)
        .await?;
        
        Ok(edit_id)
    }
    
    /// Delete a property (mark as deleted)
    pub async fn delete_property(
        &self,
        object_type: &str,
        object_id: &str,
        property_name: &str,
        user_id: &str,
    ) -> Result<(), sqlx::Error> {
        let edit_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        let null_value = serde_json::Value::Null;
        
        sqlx::query(
            r#"
            INSERT INTO user_edits (edit_id, object_type, object_id, property_name, property_value, user_id, timestamp, deleted)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (object_type, object_id, property_name)
            DO UPDATE SET
                user_id = EXCLUDED.user_id,
                timestamp = EXCLUDED.timestamp,
                deleted = TRUE
            "#,
        )
        .bind(&edit_id)
        .bind(object_type)
        .bind(object_id)
        .bind(property_name)
        .bind(null_value)
        .bind(user_id)
        .bind(timestamp)
        .bind(true)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get all edits for an object
    pub async fn get_edits_for_object(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<Vec<UserEdit>, sqlx::Error> {
        let rows = sqlx::query_as::<_, EditRow>(
            r#"
            SELECT edit_id, object_type, object_id, property_name, property_value, user_id, timestamp, deleted
            FROM user_edits
            WHERE object_type = $1 AND object_id = $2 AND deleted = FALSE
            ORDER BY timestamp DESC
            "#,
        )
        .bind(object_type)
        .bind(object_id)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    /// Get all active (non-deleted) edits for an object
    pub async fn get_active_edits(
        &self,
        object_type: &str,
        object_id: &str,
    ) -> Result<Vec<UserEdit>, sqlx::Error> {
        self.get_edits_for_object(object_type, object_id).await
    }
    
    /// Revert an edit (delete it)
    pub async fn revert_edit(
        &self,
        edit_id: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM user_edits WHERE edit_id = $1",
        )
        .bind(edit_id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct EditRow {
    edit_id: String,
    #[sqlx(rename = "object_type")]
    object_type: String,
    #[sqlx(rename = "object_id")]
    object_id: String,
    #[sqlx(rename = "property_name")]
    property_name: String,
    #[sqlx(rename = "property_value")]
    property_value: sqlx::types::Json<serde_json::Value>,
    #[sqlx(rename = "user_id")]
    user_id: String,
    timestamp: DateTime<Utc>,
    deleted: bool,
}

impl From<EditRow> for UserEdit {
    fn from(row: EditRow) -> Self {
        // Try to deserialize property_value back to PropertyValue
        let property_value: ontology_engine::PropertyValue = serde_json::from_value(row.property_value.0)
            .unwrap_or(ontology_engine::PropertyValue::Null);
        
        Self {
            edit_id: row.edit_id,
            object_type: row.object_type,
            object_id: row.object_id,
            property_name: row.property_name,
            property_value,
            user_id: row.user_id,
            timestamp: row.timestamp,
            deleted: row.deleted,
        }
    }
}

