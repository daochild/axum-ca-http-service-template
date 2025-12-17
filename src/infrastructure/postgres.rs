use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use crate::domain::entities::Message;
use crate::domain::repositories::MessageRepository;

#[derive(Clone)]
pub struct PostgresMessageRepository {
    pool: PgPool,
}

impl PostgresMessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepository for PostgresMessageRepository {
    async fn save(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        sqlx::query(
            r#"
            INSERT INTO messages (id, content, user_id, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(message.id)
        .bind(&message.content)
        .bind(&message.user_id)
        .bind(message.created_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>, Box<dyn std::error::Error>> {
        let row = sqlx::query(
            r#"
            SELECT id, content, user_id, created_at
            FROM messages
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| Message {
            id: r.get("id"),
            content: r.get("content"),
            user_id: r.get("user_id"),
            created_at: r.get("created_at"),
        }))
    }

    async fn find_recent(&self, limit: i64) -> Result<Vec<Message>, Box<dyn std::error::Error>> {
        let rows = sqlx::query(
            r#"
            SELECT id, content, user_id, created_at
            FROM messages
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .iter()
            .map(|r| Message {
                id: r.get("id"),
                content: r.get("content"),
                user_id: r.get("user_id"),
                created_at: r.get("created_at"),
            })
            .collect())
    }

    async fn health_check(&self) -> Result<bool, Box<dyn std::error::Error>> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(true)
    }
}
