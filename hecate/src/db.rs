use crate::err::HecateError;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::Client;

pub type ConnectionManager = bb8_postgres::PostgresConnectionManager<tokio_postgres::NoTls>;
pub type Pool = bb8::Pool<ConnectionManager>;

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

pub fn init_pool(
    database: &str
) -> Pool {
    let manager = PostgresConnectionManager::new(
        format!("postgres://{}", database).parse().unwrap(),
        tokio_postgres::NoTls
    );

    futures::executor::block_on(
        Pool::builder()
            .max_size(15)
            .build(manager)
    ).unwrap()
}

#[derive(Clone)]
pub struct DbReplica(pub Option<Vec<Pool>>);
impl DbReplica {
    pub fn new(database: Option<Vec<Pool>>) -> Self {
        DbReplica(database)
    }

    pub async fn dedicated(&self) -> Result<Client, HecateError> {
        match self.0 {
            None => Err(HecateError::new(503, String::from("No Database Replica Connection"), None)),
            Some(ref db_replica) => {
                let mut rng = thread_rng();
                let db_replica_it = rng.gen_range(0, db_replica.len());

                match db_replica.get(db_replica_it).unwrap().dedicated_connection().await {
                    Ok(conn) => Ok(conn),
                    Err(_) => Err(HecateError::new(503, String::from("Could not connect to database"), None))
                }
            }
        }
    }

    pub async fn get(&self) -> Result<bb8::PooledConnection<'_, bb8_postgres::PostgresConnectionManager<tokio_postgres::tls::NoTls>>, HecateError> {
        match self.0 {
            None => Err(HecateError::new(503, String::from("No Database Replica Connection"), None)),
            Some(ref db_replica) => {
                let mut rng = thread_rng();
                let db_replica_it = rng.gen_range(0, db_replica.len());

                match db_replica.get(db_replica_it).unwrap().get().await {
                    Ok(conn) => Ok(conn),
                    Err(_) => Err(HecateError::new(503, String::from("Could not connect to database"), None))
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct DbSandbox(pub Option<Vec<Pool>>);
impl DbSandbox {
    pub fn new(database: Option<Vec<Pool>>) -> Self {
        DbSandbox(database)
    }

    pub async fn dedicated(&self) -> Result<Client, HecateError> {
        match self.0 {
            None => Err(HecateError::new(503, String::from("No Database Replica Connection"), None)),
            Some(ref db_replica) => {
                let mut rng = thread_rng();
                let db_replica_it = rng.gen_range(0, db_replica.len());

                match db_replica.get(db_replica_it).unwrap().dedicated_connection().await {
                    Ok(conn) => Ok(conn),
                    Err(_) => Err(HecateError::new(503, String::from("Could not connect to database"), None))
                }
            }
        }
    }

    pub async fn get(&self) -> Result<bb8::PooledConnection<'_, bb8_postgres::PostgresConnectionManager<tokio_postgres::tls::NoTls>>, HecateError> {
        match self.0 {
            None => Err(HecateError::new(503, String::from("No Database Sandbox Connection"), None)),
            Some(ref db_sandbox) => {
                let mut rng = thread_rng();
                let db_sandbox_it = rng.gen_range(0, db_sandbox.len());

                match db_sandbox.get(db_sandbox_it).unwrap().get().await {
                    Ok(conn) => Ok(conn),
                    Err(_) => Err(HecateError::new(503, String::from("Could not connect to database"), None))
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct DbReadWrite(pub Pool); //Read & Write DB Connection
impl DbReadWrite {
    pub fn new(database: Pool) -> Self {
        DbReadWrite(database)
    }

    pub async fn get(&self) -> Result<bb8::PooledConnection<'_, bb8_postgres::PostgresConnectionManager<tokio_postgres::tls::NoTls>>, HecateError> {
        match self.0.get().await {
            Ok(conn) => Ok(conn),
            Err(_) => Err(HecateError::new(503, String::from("Could not connect to database"), None))
        }
    }
}
