extern crate reqwest;
extern crate postgres;
#[macro_use] extern crate serde_json;

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::prelude::*;
    use postgres::{Connection, TlsMode};
    use std::process::Command;
    use std::time::Duration;
    use std::thread;
    use reqwest;
    use serde_json;

    #[test]
    fn feature_delete() {
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

        let mut server = Command::new("cargo").args(&[ "run" ]).spawn().unwrap();
        thread::sleep(Duration::from_secs(1));

        { //Create Username
            let mut resp = reqwest::get("http://0.0.0.0:8000/api/user/create?username=ingalls&password=yeahehyeah&email=ingalls@protonmail.com").unwrap();
            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create Point
            let client = reqwest::Client::new();
            let mut resp = client.post("http://0.0.0.0:8000/api/data/features")
                .body(r#"{
                    "type": "FeatureCollection",
                    "message": "Create Intial Delta",
                    "features": [{
                        "type": "Feature",
                        "action": "create",
                        "message": "Creating a Point",
                        "properties": { "number": "123" },
                        "geometry": { "type": "Point", "coordinates": [ 0, 0 ] }
                    },{
                        "type": "Feature",
                        "action": "create",
                        "message": "Creating a Point",
                        "properties": { "number": "123" },
                        "geometry": { "type": "Point", "coordinates": [ 1, 1 ] }
                    }]
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Modify Point
            let client = reqwest::Client::new();
            let mut resp = client.post("http://0.0.0.0:8000/api/data/feature")
                .body(r#"{
                    "id": 1,
                    "type": "Feature",
                    "version": 1,
                    "action": "modify",
                    "message": "Modify a Point",
                    "properties": { "number": "123", "test": true },
                    "geometry": { "type": "Point", "coordinates": [ 0, 0 ] }
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert!(resp.status().is_success());
            assert_eq!(resp.text().unwrap(), "true");
        }

        { // Retrieve delta 1 and ensure both points are present
            let client = reqwest::Client::new();
            let mut resp = client.get("http://0.0.0.0:8000/api/delta/1")
                .basic_auth("ingalls", Some("yeahehyeah"))
                .send()
                .unwrap();

            let json_body: serde_json::Value = resp.json().unwrap();

            assert_eq!(json_body["affected"], json!([1, 2]));
            assert!(resp.status().is_success());
        }

        { // Retrieve feature 1 history
            let client = reqwest::Client::new();
            let mut resp = client.get("http://0.0.0.0:8000/api/data/feature/1/history")
                .basic_auth("ingalls", Some("yeahehyeah"))
                .send()
                .unwrap();

            let json_body: serde_json::Value = resp.json().unwrap();

            assert_eq!(json_body.as_array().unwrap().len(), 2);
            assert!(resp.status().is_success());
        }

        { // Hard Delete Feature 1
            let client = reqwest::Client::new();
            let mut resp = client.delete("http://0.0.0.0:8000/api/data/feature/1")
                .basic_auth("ingalls", Some("yeahehyeah"))
                .send()
                .unwrap();

            let json_body: serde_json::Value = resp.json().unwrap();

            assert_eq!(json_body, json!(true));
            assert!(resp.status().is_success());
        }

        { // Retrieve delta 1 and ensure only #2 point is present
            let client = reqwest::Client::new();
            let mut resp = client.get("http://0.0.0.0:8000/api/delta/1")
                .basic_auth("ingalls", Some("yeahehyeah"))
                .send()
                .unwrap();

            let json_body: serde_json::Value = resp.json().unwrap();

            assert_eq!(json_body["affected"], json!([2]));
            assert!(resp.status().is_success());
        }

        { // Retrieve feature 1 history
            let client = reqwest::Client::new();
            let mut resp = client.get("http://0.0.0.0:8000/api/data/feature/1/history")
                .basic_auth("ingalls", Some("yeahehyeah"))
                .send()
                .unwrap();

            assert_eq!(resp.status().as_u16(), 200);
        }

        server.kill().unwrap();
    }
}
