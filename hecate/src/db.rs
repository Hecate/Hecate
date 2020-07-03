use crate::err::HecateError;
use r2d2::ManageConnection;
use postgres::{Config, Client};
use tokio_postgres::Error;
use tokio_postgres::NoTls;

pub type PgPool = r2d2::Pool<PostgresConnectionManager<Client>>;
pub type PgPoolConn = r2d2::PooledConnection<PostgresConnectionManager<Client>>;

pub struct PostgresConnectionManager<Client> {
    config: Config,
    tls_connector: Box<dyn Fn(&Config) -> Result<Client, Error> + Send + Sync>,
}

impl PostgresConnectionManager<Client> {
    pub fn new(config: Config, tls_connector: Box<dyn Fn(&Config) -> Result<Client, Error> + Send + Sync>) -> PostgresConnectionManager<Client> {
        PostgresConnectionManager {
            config,
            tls_connector,
        }
    }
}

impl ManageConnection for PostgresConnectionManager<Client> {
    type Connection = Client;
    type Error = Error;

    fn connect(&self) -> Result<Client, Error> {
        (self.tls_connector)(&self.config)
    }

    fn is_valid(&self, client: &mut Client) -> Result<(), Error> {
        client.simple_query("").map(|_| ())
    }

    fn has_broken(&self, client: &mut Client) -> bool {
        client.is_closed()
    }
}

use rand::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub struct Database {
    pub main: String,
    pub replica: Vec<String>,
    pub sandbox: Vec<String>
}



impl Database {
    pub fn new(main: String, replica: Vec<String>, sandbox: Vec<String>) -> Self {
        Database {
            main,
            replica,
            sandbox
        }
    }
}

pub fn init_pool(database: &str) -> PgPool {
    let tls = NoTls;
    let manager = PostgresConnectionManager::new(
        format!("postgres://{}", database).parse().unwrap(),
        Box::new(move |config| config.connect(tls.clone()))
    );

    match r2d2::Pool::builder().max_size(15).build(manager) {
        Ok(pool) => pool,
        Err(err) => {
            println!("ERROR: Failed to connect to database: {}", err);
            std::process::exit(1);
        }
    }
}

#[derive(Clone)]
pub struct DbReplica(pub Option<Vec<PgPool>>);
impl DbReplica {
    pub fn new(database: Option<Vec<PgPool>>) -> Self {
        DbReplica(database)
    }

    pub fn get(&self) -> Result<Client, HecateError> {
        match self.0 {
            None => Err(HecateError::new(503, String::from("No Database Replica Connection"), None)),
            Some(ref db_replica) => {
                let mut rng = thread_rng();
                let db_replica_it = rng.gen_range(0, db_replica.len());

                match db_replica.get(db_replica_it).unwrap().get() {
                    Ok(conn) => Ok(*conn),
                    Err(_) => Err(HecateError::new(503, String::from("Could not connect to database"), None))
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct DbSandbox(pub Option<Vec<PgPool>>);
impl DbSandbox {
    pub fn new(database: Option<Vec<PgPool>>) -> Self {
        DbSandbox(database)
    }

    pub fn get(&self) -> Result<postgres::Client, HecateError> {
        match self.0 {
            None => Err(HecateError::new(503, String::from("No Database Sandbox Connection"), None)),
            Some(ref db_sandbox) => {
                let mut rng = thread_rng();
                let db_sandbox_it = rng.gen_range(0, db_sandbox.len());

                match db_sandbox.get(db_sandbox_it).unwrap().get() {
                    Ok(conn) => Ok(*conn),
                    Err(_) => Err(HecateError::new(503, String::from("Could not connect to database"), None))
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct DbReadWrite(pub PgPool); //Read & Write DB Connection
impl DbReadWrite {
    pub fn new(database: PgPool) -> Self {
        DbReadWrite(database)
    }

    pub fn get(&self) -> Result<postgres::Client, HecateError> {
        match self.0.get() {
            Ok(conn) => Ok(*conn),
            Err(_) => Err(HecateError::new(503, String::from("Could not connect to database"), None))
        }
    }
}
