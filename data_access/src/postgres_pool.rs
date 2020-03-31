use coi::Inject;
use mobc_postgres::{
    mobc::{Connection, Error as MobcError, Manager, Pool},
    tokio_postgres::NoTls,
    PgConnectionManager,
};
use shared::Settings;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Inject)]
#[coi(provides pub PostgresPool with PostgresPool::new(settings))]
pub struct PostgresPool {
    pool: Pool<PgConnectionManager<NoTls>>,
    #[coi(inject)]
    settings: Arc<Settings>,
}

impl PostgresPool {
    pub fn new(settings: Arc<Settings>) -> Self {
        let config = "postgresql://doadmin:y8w39xbc555s4mvt@journal-db-do-user-2475037-0.a.db.ondigitalocean.com:25060/defaultdb?sslmode=require"
            .parse()
            .expect("Unable to parse connection string");
        let manager = PgConnectionManager::new(config, NoTls);
        let pool = Pool::builder().max_open(20).build(manager);

        Self { pool, settings }
    }

    pub async fn get(
        &self,
    ) -> Result<
        Connection<PgConnectionManager<NoTls>>,
        MobcError<<PgConnectionManager<NoTls> as Manager>::Error>,
    > {
        self.pool.get().await
    }
}
