use coi::Inject;
use mobc_postgres::{
    mobc::{Connection, Error as MobcError, Manager, Pool},
    PgConnectionManager,
};
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use shared::Settings;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Inject)]
#[coi(provides pub PostgresPool with PostgresPool::new(settings))]
pub struct PostgresPool {
    pool: Pool<PgConnectionManager<MakeTlsConnector>>,
    #[coi(inject)]
    settings: Arc<Settings>,
}

impl PostgresPool {
    pub fn new(settings: Arc<Settings>) -> Self {
        let path = std::path::Path::new(&settings.database.cert_directory)
            .join("ca-certificate.pem");

        let cert = std::fs::read(path).unwrap();
        let cert = Certificate::from_pem(&cert).unwrap();
        let connector = TlsConnector::builder()
            .add_root_certificate(cert)
            .build()
            .unwrap();
        let connector = MakeTlsConnector::new(connector);
        let config = settings
            .as_ref()
            .database
            .connection_string
            .parse()
            .expect("Unable to parse connection string");
        let manager = PgConnectionManager::new(config, connector);
        let pool = Pool::builder().max_open(20).build(manager);

        Self { pool, settings }
    }

    pub async fn get(
        &self,
    ) -> Result<
        Connection<PgConnectionManager<MakeTlsConnector>>,
        MobcError<<PgConnectionManager<MakeTlsConnector> as Manager>::Error>,
    > {
        self.pool.get().await
    }
}
