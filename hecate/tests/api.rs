extern crate reqwest;
extern crate postgres;
extern crate hecate;

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::prelude::*;
    use postgres::{Connection, TlsMode};
    use std::process::Command;
    use std::time::Duration;
    use std::thread;
    use reqwest;

    #[test]
    fn  api() {
        { // Reset Database:
            let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();

            conn.execute("
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE
                    pg_stat_activity.datname = 'hecate'
                    AND pid <> pg_backend_pid();
            ", &[]).unwrap();

            conn.execute("DROP DATABASE IF EXISTS hecate;", &[]).unwrap();
            conn.execute("CREATE DATABASE hecate;", &[]).unwrap();

            let conn = Connection::connect("postgres://postgres@localhost:5432/hecate", TlsMode::None).unwrap();

            let mut file = File::open("./src/schema.sql").unwrap();
            let mut table_sql = String::new();
            file.read_to_string(&mut table_sql).unwrap();
            conn.batch_execute(&*table_sql).unwrap();
        }

        let mut server = Command::new("cargo").args(&[ "run" ]).spawn().unwrap();
        thread::sleep(Duration::from_secs(1));

        { //API Meta
            let mut resp = reqwest::get("http://localhost:8000/api").unwrap();
            assert!(resp.text().unwrap().contains("version"));
            assert!(resp.status().is_success());
        }

        server.kill().unwrap();
    }
}
