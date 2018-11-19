extern crate reqwest;
extern crate postgres;
#[macro_use] extern crate serde_json;

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::env;
    use std::io::prelude::*;
    use postgres::{Connection, TlsMode};
    use std::process::Command;
    use std::time::Duration;
    use std::thread;
    use reqwest;
    use serde_json;

    #[test]
    fn meta() {
        {
            let conn = Connection::connect("postgres://postgres@localhost:5432", TlsMode::None).unwrap();

            conn.execute("
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE
                    pg_stat_activity.datname = 'hecate'
                    AND pid <> pg_backend_pid();
            ", &[]).unwrap();

            conn.execute("
                DROP DATABASE IF EXISTS hecate;
            ", &[]).unwrap();

            conn.execute("
                CREATE DATABASE hecate;
            ", &[]).unwrap();

            let conn = Connection::connect("postgres://postgres@localhost:5432/hecate", TlsMode::None).unwrap();

            let mut file = File::open("./src/schema.sql").unwrap();
            let mut table_sql = String::new();
            file.read_to_string(&mut table_sql).unwrap();
            conn.batch_execute(&*table_sql).unwrap();
        }

        let mut server = Command::new("cargo").args(&[
            "run",
            "--",
            "--auth", env::current_dir().unwrap().join("tests/fixtures/auth.closed.json").to_str().unwrap()
        ]).spawn().unwrap();
        thread::sleep(Duration::from_secs(1));

        { //Create Username (ingalls)
            let mut resp = reqwest::get("http://localhost:8000/api/user/create?username=ingalls&password=yeaheh&email=ingalls@protonmail.com").unwrap();
            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            
            let mut resp = client.get("http://localhost:8000/api/meta")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!([]));

            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            
            let mut resp = client.get("http://localhost:8000/api/meta/key")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!(null));

            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            
            let mut resp = client.post("http://localhost:8000/api/meta/key")
                .body(r#"{ "value": "test" }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!(true));
            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            
            let mut resp = client.get("http://localhost:8000/api/meta")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!([ "key" ]));

            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            
            let mut resp = client.get("http://localhost:8000/api/meta/key")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!({
                "value": "test"
            }));

            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            
            let mut resp = client.post("http://localhost:8000/api/meta/key")
                .body(r#"{ "value": false }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!(true));
            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            
            let mut resp = client.get("http://localhost:8000/api/meta")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!([ "key" ]));

            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            
            let mut resp = client.get("http://localhost:8000/api/meta/key")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!({
                "value": false    
            }));

            assert!(resp.status().is_success());
        }

        server.kill().unwrap();
    }
}
