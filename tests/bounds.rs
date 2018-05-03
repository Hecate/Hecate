extern crate reqwest;
extern crate postgres;

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
    fn bounds() {
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

        let mut server = Command::new("cargo").arg("run").spawn().unwrap();
        thread::sleep(Duration::from_secs(1));

        {
            let conn = Connection::connect("postgres://postgres@localhost:5432/hecate", TlsMode::None).unwrap();

            conn.execute(r#"
                INSERT INTO bounds(geom, name) VALUES (
                    ST_Multi(ST_SetSRID(ST_GeomFromGeoJSON('{ "type": "Polygon", "coordinates": [ [ [ -77.13363647460938, 38.83542884007305 ], [ -76.96403503417969, 38.83542884007305 ], [ -76.96403503417969, 38.974891064341726 ], [ -77.13363647460938, 38.974891064341726 ], [ -77.13363647460938, 38.83542884007305 ] ] ] }'), 4326)),
                    'dc'
                );
            "#, &[]).unwrap();
        }

        { //Create Username
            let mut resp = reqwest::get("http://localhost:8000/api/user/create?username=ingalls&password=yeaheh&email=ingalls@protonmail.com").unwrap();
            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create Point Inside of Bounds
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "action": "create",
                    "message": "Create Point Inside of Bounds",
                    "properties": { "indc": true },
                    "geometry": { "type": "Point", "coordinates": [ -77.01210021972656,38.925763232374514 ] }
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::ContentType::json())
                .send()
                .unwrap();

            assert!(resp.status().is_success());
            assert_eq!(resp.text().unwrap(), "true");
        }

        { //Create Point Outside of Bounds
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "action": "create",
                    "message": "Create Point Outside of Bounds",
                    "properties": { "indc": false },
                    "geometry": { "type": "Point", "coordinates": [ -76.94755554199219,38.90385833966778 ] }
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::ContentType::json())
                .send()
                .unwrap();

            assert!(resp.status().is_success());
            assert_eq!(resp.text().unwrap(), "true");
        }

        { //List Bounds
            let mut resp = reqwest::get("http://localhost:8000/api/data/bounds").unwrap();
            assert_eq!(resp.text().unwrap(), r#"["dc"]"#);
            assert!(resp.status().is_success());
        }

        { //Get Bounds
            let mut resp = reqwest::get("http://localhost:8000/api/data/bounds/dc").unwrap();
            let mut body_str = String::from(resp.text().unwrap());
            body_str.pop();
            assert_eq!(&*body_str, r#"{"id":1,"type":"Feature","version":1,"geometry":{"type":"Point","coordinates":[-77.0121002197266,38.9257632323745]},"properties":{"indc": true}}"#);
            assert!(resp.status().is_success());
        }

        { //Delete first point inside of bounds
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "id": 1,
                    "type": "Feature",
                    "action": "delete",
                    "version": 1,
                    "message": "Create Point Inside of Bounds",
                    "properties": null,
                    "geometry": null
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::ContentType::json())
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create Point Inside of Bounds with very large props (4k is the default read iterator)
            let client = reqwest::Client::new();
            let mut resp = client.post("http://localhost:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "action": "create",
                    "message": "Create Point Inside of Bounds",
                    "properties": { "large_props": "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" },
                    "geometry": { "type": "Point", "coordinates": [ -77.01210021972656,38.925763232374514 ] }
                }"#)
                .basic_auth("ingalls", Some("yeaheh"))
                .header(reqwest::header::ContentType::json())
                .send()
                .unwrap();

            assert!(resp.status().is_success());
            assert_eq!(resp.text().unwrap(), "true");
        }

        { //Get Bounds
            let mut resp = reqwest::get("http://localhost:8000/api/data/bounds/dc").unwrap();
            let mut body_str = String::from(resp.text().unwrap());
            body_str.pop();
            assert_eq!(&*body_str, r#"{"id":3,"type":"Feature","version":1,"geometry":{"type":"Point","coordinates":[-77.0121002197266,38.9257632323745]},"properties":{"large_props": "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"}}"#);
            assert!(resp.status().is_success());
        }

        server.kill().unwrap();
    }
}
