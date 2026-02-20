use sqlx::Row;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};

use crate::config::DatabaseConfig;
use crate::db::models::{CreateUser, UpdateUser, User};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.connection_string())
            .await?;

        Ok(Self { pool })
    }

    pub async fn connect(config: &DatabaseConfig) -> Result<Self, sqlx::Error> {
        Self::new(config).await
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn init_schema(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id BIGSERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (name, email)
            VALUES ($1, $2)
            RETURNING id, name, email, created_at, updated_at
            "#,
        )
        .bind(&user.name)
        .bind(&user.email)
        .fetch_one(&self.pool)
        .await?;

        Ok(Self::row_to_user(row))
    }

    pub async fn get_user(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(Self::row_to_user))
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            ORDER BY id
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(Self::row_to_user).collect())
    }

    pub async fn update_user(
        &self,
        id: i64,
        update: UpdateUser,
    ) -> Result<Option<User>, sqlx::Error> {
        if let (Some(name), Some(email)) = (&update.name, &update.email) {
            let row = sqlx::query(
                r#"
                UPDATE users 
                SET name = $1, email = $2, updated_at = NOW()
                WHERE id = $3 
                RETURNING id, name, email, created_at, updated_at
                "#,
            )
            .bind(name)
            .bind(email)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
            return Ok(row.map(Self::row_to_user));
        }

        if let Some(name) = &update.name {
            let row = sqlx::query(
                r#"
                UPDATE users 
                SET name = $1, updated_at = NOW()
                WHERE id = $2 
                RETURNING id, name, email, created_at, updated_at
                "#,
            )
            .bind(name)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
            return Ok(row.map(Self::row_to_user));
        }

        if let Some(email) = &update.email {
            let row = sqlx::query(
                r#"
                UPDATE users 
                SET email = $1, updated_at = NOW()
                WHERE id = $2 
                RETURNING id, name, email, created_at, updated_at
                "#,
            )
            .bind(email)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
            return Ok(row.map(Self::row_to_user));
        }

        self.get_user(id).await
    }

    pub async fn delete_user(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    fn row_to_user(row: PgRow) -> User {
        User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
