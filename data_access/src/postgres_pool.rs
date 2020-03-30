use coi::{Inject, Provide};
use mobc_postgres::{
    mobc::{Connection, Error as MobcError, Manager, Pool},
    tokio_postgres::NoTls,
    PgConnectionManager,
};

#[derive(Inject)]
pub struct PostgresPool<T>(Pool<PgConnectionManager<T>>)
where
    PgConnectionManager<T>: Manager;

impl<T> PostgresPool<T>
where
    PgConnectionManager<T>: Manager,
{
    pub async fn get(
        &self,
    ) -> Result<
        Connection<PgConnectionManager<T>>,
        MobcError<<PgConnectionManager<T> as Manager>::Error>,
    > {
        self.0.get().await
    }
}

#[derive(Provide)]
#[coi(provides PostgresPool<T> with PostgresPool(self.0.clone()))]
pub struct PostgresPoolProvider<T>(Pool<PgConnectionManager<T>>)
where
    PgConnectionManager<T>: Manager;

impl<T> PostgresPoolProvider<T>
where
    PgConnectionManager<T>: Manager,
{
    pub fn new(pool: Pool<PgConnectionManager<T>>) -> Self {
        Self(pool)
    }
}

pub fn create_pool_provider() -> PostgresPoolProvider<NoTls> {
    let config = "".parse().expect("Unable to parse connection string");
    let manager = PgConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_open(20).build(manager);
    PostgresPoolProvider::new(pool)
}
