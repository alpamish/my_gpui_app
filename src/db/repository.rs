use sqlx::Row;
use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};

use crate::config::DatabaseConfig;
use crate::db::models::{CreateUser, UpdateUser, User};
use crate::error::{AppError, Result};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(config: &DatabaseConfig) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.connection_string())
            .await
            .map_err(AppError::Database)?;

        Ok(Self { pool })
    }

    pub async fn connect(config: &DatabaseConfig) -> Result<Self> {
        Self::new(config).await
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn init_schema(&self) -> Result<()> {
        // Read and execute the schema SQL file
        let schema_sql = include_str!("schema.sql");
        
        // Split the schema file into individual statements and execute them
        let statements = schema_sql.split("-- =====================================================")
            .filter(|s| !s.trim().is_empty())
            .flat_map(|section| {
                // Extract CREATE statements from each section
                let mut statements = Vec::new();
                let mut current_statement = String::new();
                
                for line in section.lines() {
                    if line.trim().is_empty() && !current_statement.trim().is_empty() {
                        statements.push(current_statement.trim().to_string());
                        current_statement.clear();
                    } else {
                        if !line.trim().starts_with("--") && !line.trim().starts_with("CREATE EXTENSION") &&
                           !line.trim().starts_with("CREATE TYPE") && !line.trim().starts_with("CREATE TABLE") &&
                           !line.trim().starts_with("CREATE INDEX") && !line.trim().starts_with("ALTER TABLE") &&
                           !line.trim().starts_with("CREATE OR REPLACE FUNCTION") {
                            current_statement.push_str(line);
                            current_statement.push('\n');
                        } else if line.trim().starts_with("CREATE") || line.trim().starts_with("ALTER") {
                            if !current_statement.trim().is_empty() {
                                statements.push(current_statement.trim().to_string());
                                current_statement.clear();
                            }
                            current_statement.push_str(line);
                            current_statement.push('\n');
                        } else if !line.trim().starts_with("--") {
                            current_statement.push_str(line);
                            current_statement.push('\n');
                        }
                    }
                }
                
                if !current_statement.trim().is_empty() {
                    statements.push(current_statement.trim().to_string());
                }
                
                statements
            })
            .filter(|s: &String| {
                let trimmed = s.trim();
                !trimmed.is_empty() && 
                (trimmed.starts_with("CREATE") || 
                 trimmed.starts_with("ALTER") || 
                 trimmed.starts_with("INSERT") ||
                 trimmed.starts_with("UPDATE") ||
                 trimmed.starts_with("DELETE"))
            })
            .collect::<Vec<String>>();

        for statement in statements {
            let stmt = statement.trim();
            if !stmt.is_empty() {
                sqlx::query(stmt)
                    .execute(&self.pool)
                    .await
                    .map_err(AppError::Database)?;
            }
        }

        Ok(())
    }

    pub async fn create_user(&self, user: CreateUser) -> Result<User> {
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
        .await
        .map_err(AppError::Database)?;

        Ok(Self::row_to_user(row))
    }

    pub async fn get_user(&self, id: i64) -> Result<Option<User>> {
        let row = sqlx::query(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(row.map(Self::row_to_user))
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, email, created_at, updated_at
            FROM users
            ORDER BY id
            "#,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::Database)?;

        Ok(rows.into_iter().map(Self::row_to_user).collect())
    }

    pub async fn update_user(
        &self,
        id: i64,
        update: UpdateUser,
    ) -> Result<Option<User>> {
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
            .await
            .map_err(AppError::Database)?;
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
            .await
            .map_err(AppError::Database)?;
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
            .await
            .map_err(AppError::Database)?;
            return Ok(row.map(Self::row_to_user));
        }

        self.get_user(id).await
    }

    pub async fn delete_user(&self, id: i64) -> Result<bool> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::Database)?;

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
