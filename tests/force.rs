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
    use std::env;
    use reqwest;
    use serde_json;

    #[test]
    fn force() {
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

        { //Create Username
            let mut resp = reqwest::get("http://localhost:8000/api/user/create?username=ingalls&password=yeaheh&email=ingalls@protonmail.com").unwrap();
            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create Point - Force - Action Required
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "force": true,
                    "message": "Testing Force Option",
                    "properties": {
                        "street": "Main Street"
                    },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [0, 0]
                    }
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "{\"feature\":{\"force\":true,\"geometry\":{\"coordinates\":[0.0,0.0],\"type\":\"Point\"},\"message\":\"Testing Force Option\",\"properties\":{\"street\":\"Main Street\"},\"type\":\"Feature\"},\"id\":null,\"message\":\"Action Required\"}");
            assert!(resp.status().is_client_error());
        }

        { //Create Point - Force - Action Create required
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "action": "modify",
                    "force": true,
                    "message": "Testing Force Option",
                    "properties": {
                        "street": "Main Street"
                    },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [0, 0]
                    }
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "{\"feature\":{\"action\":\"modify\",\"force\":true,\"geometry\":{\"coordinates\":[0.0,0.0],\"type\":\"Point\"},\"message\":\"Testing Force Option\",\"properties\":{\"street\":\"Main Street\"},\"type\":\"Feature\"},\"id\":null,\"message\":\"force can only be used on create\"}");
            assert!(resp.status().is_client_error());
        }

        { //Create Point - Force - Must be used with key value
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "action": "create",
                    "force": true,
                    "message": "Testing Force Option",
                    "properties": {
                        "street": "Main Street"
                    },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [0, 0]
                    }
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "{\"feature\":{\"action\":\"create\",\"force\":true,\"geometry\":{\"coordinates\":[0.0,0.0],\"type\":\"Point\"},\"message\":\"Testing Force Option\",\"properties\":{\"street\":\"Main Street\"},\"type\":\"Feature\"},\"id\":null,\"message\":\"force can only be used with a key value\"}");
            assert!(resp.status().is_client_error());
        }

        { //Create Point - Success
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "key": "1",
                    "type": "Feature",
                    "action": "create",
                    "force": true,
                    "message": "Testing Force Option",
                    "properties": {
                        "street": "Main Street"
                    },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [0, 0]
                    }
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            let mut resp = client.get("http://localhost:8000/api/data/feature/1")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!({
                "id": 1,
                "key": "1",
                "type": "Feature",
                "version": 1,
                "properties": {
                    "street": "Main Street"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [ 0.0, 0.0 ]
                }
            }));

            assert!(resp.status().is_success());
        }

        { //Create Point - Force Success
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "key": "1",
                    "type": "Feature",
                    "action": "create",
                    "force": true,
                    "message": "Testing Force Option",
                    "properties": {
                        "street": "I AM A NEW FEAT"
                    },
                    "geometry": {
                        "type": "Point",
                        "coordinates": [1, 1]
                    }
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            let mut resp = client.get("http://localhost:8000/api/data/feature/1")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!({
                "id": 1,
                "key": "1",
                "type": "Feature",
                "version": 2,
                "properties": {
                    "street": "I AM A NEW FEAT"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [ 1.0, 1.0 ]
                }
            }));

            assert!(resp.status().is_success());
        }

        { //Create Point - Force Success FeatureCollection
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/features")
                .body(r#"{
                    "type": "FeatureCollection",
                    "message": "Testing Force Option",
                    "features": [{
                        "key": "1",
                        "type": "Feature",
                        "action": "create",
                        "force": true,
                        "properties": {
                            "street": "I AM AN EVEN NEWER FEAT"
                        },
                        "geometry": {
                            "type": "Point",
                            "coordinates": [1, 1]
                        }
                    }]
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        {
            let client = reqwest::Client::new();
            let mut resp = client.get("http://localhost:8000/api/data/feature/1")
                .basic_auth("ingalls", Some("yeaheh"))
                .send()
                .unwrap();

            let json_body: serde_json::value::Value = resp.json().unwrap();

            assert_eq!(json_body, json!({
                "id": 1,
                "key": "1",
                "type": "Feature",
                "version": 3,
                "properties": {
                    "street": "I AM AN EVEN NEWER FEAT"
                },
                "geometry": {
                    "type": "Point",
                    "coordinates": [ 1.0, 1.0 ]
                }
            }));

            assert!(resp.status().is_success());
        }
        
        { //Create Point - Force Missing Key - FeatureCollection
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/features")
                .body(r#"{
                    "type": "FeatureCollection",
                    "message": "Testing Force Option",
                    "features": [{
                        "type": "Feature",
                        "action": "create",
                        "force": true,
                        "properties": {
                            "street": "I AM AN EVEN NEWER FEAT"
                        },
                        "geometry": {
                            "type": "Point",
                            "coordinates": [1, 1]
                        }
                    }]
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "{\"feature\":{\"action\":\"create\",\"force\":true,\"geometry\":{\"coordinates\":[1.0,1.0],\"type\":\"Point\"},\"properties\":{\"street\":\"I AM AN EVEN NEWER FEAT\"},\"type\":\"Feature\"},\"id\":null,\"message\":\"force can only be used with a key value\"}");
            assert!(resp.status().is_client_error());
        }

        server.kill().unwrap();
    }
}
