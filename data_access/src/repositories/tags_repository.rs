use crate::{error::Error, postgres_pool::PostgresPool};
use async_trait::async_trait;
use coi::Inject;
use mobc_postgres::tokio_postgres::NoTls;
use serde::Deserialize;
use serde_tokio_postgres::from_row;
use std::sync::Arc;

#[async_trait]
pub trait TagRepositoryTrait: Inject {
    async fn get_tags(&self) -> Result<Vec<DbTag>, Error>;
}

#[derive(Inject)]
#[coi(provides pub dyn TagRepositoryTrait with TagRepository::new(pool))]
struct TagRepository {
    #[coi(inject)]
    pool: Arc<PostgresPool<NoTls>>,
}

#[async_trait]
impl TagRepositoryTrait for TagRepository {
    async fn get_tags(&self) -> Result<Vec<DbTag>, Error> {
        let client = self.pool.get().await?;
        let statement = client.prepare("SELECT id, name FROM Tags").await?;
        let rows = client.query(&statement, &[]).await?;
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
    fn new(pool: Arc<PostgresPool<NoTls>>) -> Self {
        Self { pool }
    }
}

#[derive(Deserialize)]
pub struct DbTag {
    pub id: i32,
    pub name: String,
}
