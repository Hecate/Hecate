extern crate hecate;
#[macro_use] extern crate clap;
extern crate serde_json;
extern crate postgres;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use hecate::auth::CustomAuth;
use hecate::auth::AuthModule;
use std::error::Error;
use clap::App;
use semver::Version;

fn main() {
    let cli_cnf = load_yaml!("cli.yml");
    let matched = App::from_yaml(cli_cnf).get_matches();

    let database = String::from(matched.value_of("database").unwrap_or("hecate@localhost:5432/hecate"));

    let database_sandbox = match matched.values_of("database_sandbox") {
        None => vec![String::from("hecate_read@localhost:5432/hecate")],
        Some(db_read) => db_read.map(|db| String::from(db)).collect()
    };

    let database_replica = match matched.values_of("database_replica") {
        None => vec![database.clone()],
        Some(db_read) => db_read.map(|db| String::from(db)).collect()
    };

    let schema: Option<serde_json::Value> = match matched.value_of("schema") {
        Some(schema_path) => {
            let mut schema_file = match File::open(&Path::new(schema_path)) {
                Ok(file) => file,
                Err(_) => panic!("Failed to open schema file at: {}", schema_path)
            };

            let mut schema_str = String::new();

            schema_file.read_to_string(&mut schema_str).unwrap();

            let schema_json: serde_json::Value = serde_json::from_str(&schema_str).unwrap();

            Some(schema_json)
        },
        None => None
    };

    let auth: Option<CustomAuth> = match matched.value_of("auth") {
        Some(auth_path) => {
            let mut auth_file = match File::open(&Path::new(auth_path)) {
                Ok(file) => file,
                Err(_) => panic!("Failed to open auth file at: {}", auth_path)
            };

            let mut auth = String::new();

            if let Err(err) = auth_file.read_to_string(&mut auth) {
                panic!("Could not read auth file: {}", err.to_string())
            }

            let auth: serde_json::Value = match serde_json::from_str(&*auth) {
                Ok(auth) => auth,
                Err(err) => panic!("Auth file is not valid JSON: {}", err.to_string())
            };

            let auth = match CustomAuth::parse(Some(&auth)) {
                Ok(auth) => auth,
                Err(err) => panic!("{}", err.as_log())
            };

            Some(*auth)
        },
        None => None
    };

    let port: Option<u16> = match matched.value_of("port") {
        Some(port) => match port.parse() {
            Ok(port) => Some(port),
            _ => { panic!("Port must be an integer > 1000") }
        },
        None => None
    };

    let workers: Option<u16> = match matched.value_of("workers") {
        Some(workers) => match workers.parse() {
            Ok(workers) => Some(workers),
            _ => { panic!("workers arg must be integer value") }
        },
        None => None
    };

    let ui: Option<String> = match matched.value_of("ui") {
        Some(ui) => match ui.parse::<String>() {
            Ok(ui) => {
                let path = Path::new(&env::current_dir().unwrap()).join(ui);
                let path: String = path.to_str().unwrap().into();
                Some(path)
            },
            _ => { panic!("web must be an path to the web folder") }
        },
        None => None
    };

    database_check(&database, false);

    for db_replica in &database_replica {
        database_check(db_replica, true);
    }

    for db_sandbox in &database_sandbox {
        database_check(db_sandbox, true);
    }

    hecate::start(
        hecate::db::Database::new(database, database_replica, database_sandbox),
        port,
        workers,
        schema,
        auth,
        ui
    );
}

fn database_check(conn_str: &String, is_read: bool) {
    match tokio_postgres::Client::connect(format!("postgres://{}", conn_str), postgres::TlsMode::None) {
        Ok(conn) => {
            let conn_type = match is_read {
                true => String::from("READ"),
                false => String::from("READ/WRITE")
            };

            if !is_read {
                match conn.query("
                    SELECT
                        (regexp_matches(version(), 'PostgreSQL (.*?) '))[1] AS postgres_v,
                        (regexp_matches(postgis_version(), '^(.*?) '))[1] AS postgis_v
                ", &[]) {
                    Ok(res) => {
                        if res.len() != 1 {
                            println!("ERROR: Connection unable obtain PostgreSQL version using ({}) {}", conn_type, conn_str);
                            std::process::exit(1);
                        }
                        let postgres_v: String = res.get(0).get(0);
                        let got_postgres_v = match Version::parse(&postgres_v){
                            Ok(version) => version,
                            Err(_) => {
                                let fix_semver = format!("{}.0", postgres_v);
                                Version::parse(&fix_semver).unwrap_or_else(|_| panic!("Failed to parse semver for PostgreSQL {}.", fix_semver))
                            }
                        };
                        if ! hecate::POSTGRES_VERSION.matches(&got_postgres_v) {
                            panic!(
                                "ERROR: Hecate requires PostgreSQL version {}, got {}.", 
                                hecate::POSTGRES_VERSION.to_string(),
                                got_postgres_v
                            );
                        }
                        let postgis_v: String = res.get(0).get(1);
                        let got_postgis_v = match Version::parse(&postgis_v){
                            Ok(version) => version,
                            Err(_) => {
                                let fix_semver = format!("{}.0", postgis_v);
                                Version::parse(&fix_semver).unwrap_or_else(|_| panic!("Failed to parse semver for PostGIS {}", fix_semver))
                            }
                        };
                        if ! hecate::POSTGIS_VERSION.matches(&got_postgis_v) {
                            panic!(
                                "ERROR: Hecate requires PostGIS version {}, got {}.",
                                hecate::POSTGIS_VERSION.to_string(),
                                got_postgis_v
                            );
                        }
                    },
                    Err(err) => {
                        println!("ERROR: Connection unable obtain PostgreSQL version using ({}) {}", conn_type, conn_str);
                        println!("ERROR: {}", err);
                        println!("ERROR: Caused by: {}", err.source().unwrap());
                        std::process::exit(1);
                    }
                }
            }

            match conn.query("
                SELECT id FROM geo LIMIT 1
            ", &[]) {
                Ok(_) => (),
                Err(err) => {
                    println!("ERROR: Connection unable to {} geo table using {}", conn_type, conn_str);
                    println!("ERROR: {}", err);
                    println!("ERROR: Caused by: {}", err.source().unwrap());
                    std::process::exit(1);
                }
            }
        },
        Err(err) => {
            println!("ERROR: Unable to connect to {}", conn_str);
            println!("ERROR: {}", err);
            println!("ERROR: caused by: {}", err.source().unwrap());

            std::process::exit(1);
        }
    }
}
