use crate::{error::Error, postgres_pool::PostgresPool};
use async_trait::async_trait;
use coi::Inject;
use serde::Deserialize;
use serde_tokio_postgres::from_row;
use std::sync::Arc;

#[async_trait]
pub trait TagRepositoryTrait: Inject {
    async fn get_tags(&self) -> Result<Vec<DbTag>, Error>;
}

#[derive(Inject)]
#[coi(provides pub dyn TagRepositoryTrait with TagRepository::new(database_pool))]
struct TagRepository {
    #[coi(inject)]
    database_pool: Arc<PostgresPool>,
}

#[async_trait]
impl TagRepositoryTrait for TagRepository {
    async fn get_tags(&self) -> Result<Vec<DbTag>, Error> {
        let client = self.database_pool.get().await?;
        let rows = client.query("SELECT id, name FROM tags", &[]).await?;
        let data = rows
            .into_iter()
            .map(|x| {
                from_row::<DbTag>(x)
                    .map(Into::into)
                    .map_err(Into::<Error>::into)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(data)
    }
}

impl TagRepository {
    fn new(database_pool: Arc<PostgresPool>) -> Self {
        Self { database_pool }
    }
}

#[derive(Deserialize)]
pub struct DbTag {
    pub id: i32,
    pub name: String,
}
