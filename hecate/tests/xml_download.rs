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
    fn xml_download() {
        { // Reset Database:
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
            let mut resp = client.post("http://0.0.0.0:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "message": "Create Point",
                    "action": "create",
                    "properties": { "addr:housenumber": "1234", "addr:street": "Main St" },
                    "geometry": { "type": "Point", "coordinates": [ -79.46014970541, 43.67263458218963 ] }
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create MultiPoint
            let client = reqwest::Client::new();
            let mut resp = client.post("http://0.0.0.0:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "message": "Create MultiPoint",
                    "action": "create",
                    "properties": { "addr:housenumber": "1234", "addr:street": "yet another street" },
                    "geometry": { "type": "MultiPoint", "coordinates": [[ -79.45843040943144, 43.67243669841148 ], [ -79.45821315050125, 43.67242699820951 ] ] }
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create LineString
            let client = reqwest::Client::new();
            let mut resp = client.post("http://0.0.0.0:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "message": "Create LineString",
                    "action": "create",
                    "properties": { "highway": "residential", "name": "Main St E" },
                    "geometry": { "type": "LineString", "coordinates": [ [ -79.46089804172516, 43.67312928878038 ], [ -79.46036696434021, 43.67187602416343 ] ] }
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create MultiLineString
            let client = reqwest::Client::new();
            let mut resp = client.post("http://0.0.0.0:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "message": "Create MultiLineString",
                    "action": "create",
                    "properties": { "highway": "service", "name": "Don't drive on me" },
                    "geometry": { "type": "MultiLineString", "coordinates": [[ [ -79.4596266746521, 43.672062269477344 ], [ -79.45907950401306, 43.67215539191757 ], [ -79.45853233337401, 43.6720661495819 ] ],[ [ -79.4583123922348, 43.67200406787885 ], [ -79.45751309394836, 43.67179066153475 ] ]] }
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create Polygon
            let client = reqwest::Client::new();
            let mut resp = client.post("http://0.0.0.0:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "message": "Create Polygon",
                    "action": "create",
                    "properties": { "building": true },
                    "geometry": { "type": "Polygon", "coordinates": [[ [ -79.46098119020462, 43.6734687909438 ], [ -79.46066468954086, 43.6734687909438 ], [ -79.46066468954086, 43.673674431320244 ], [ -79.46098119020462, 43.673674431320244 ], [ -79.46098119020462, 43.6734687909438 ] ]] }
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create Polygon-Inner
            let client = reqwest::Client::new();
            let mut resp = client.post("http://0.0.0.0:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "message": "Create Polygon-Inner",
                    "action": "create",
                    "properties": { "building": true },
                    "geometry": {
                        "type": "Polygon",
                        "coordinates": [ [
                            [ -79.45962399244308, 43.67299542739944 ],
                            [ -79.45887833833694, 43.67299542739944 ],
                            [ -79.45887833833694, 43.67349207102181 ],
                            [ -79.45962399244308, 43.67349207102181 ],
                            [ -79.45962399244308, 43.67299542739944 ]
                        ], [
                            [ -79.45944160223007, 43.67311376863557 ],
                            [ -79.45905536413191, 43.67311376863557 ],
                            [ -79.45905536413191, 43.673360150460475 ],
                            [ -79.45944160223007, 43.673360150460475 ],
                            [ -79.45944160223007, 43.67311376863557 ]
                        ] ]
                    }
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { //Create MultiPolygon
            let client = reqwest::Client::new();
            let mut resp = client.post("http://0.0.0.0:8000/api/data/feature")
                .body(r#"{
                    "type": "Feature",
                    "message": "Create MultiPolygon",
                    "action": "create",
                    "properties": { "building": true, "amenity": "hospital" },
                    "geometry": {
                        "type": "MultiPolygon",
                        "coordinates": [
                            [ [
                                [ -79.45878982543945, 43.67362593129495 ],
                                [ -79.45830166339874, 43.67362593129495 ],
                                [ -79.45830166339874, 43.67394021076283 ],
                                [ -79.45878982543945, 43.67394021076283 ],
                                [ -79.45878982543945, 43.67362593129495 ]
                            ], [
                                [ -79.45868790149689, 43.67371517131118 ],
                                [ -79.45868790149689, 43.673866491035405 ],
                                [ -79.45843577384949, 43.673866491035405 ],
                                [ -79.45843577384949, 43.67371517131118 ],
                                [ -79.45868790149689, 43.67371517131118 ]
                            ] ], [ [
                                [ -79.45853769779205, 43.67316032905796 ],
                                [ -79.45803344249725, 43.67316032905796 ],
                                [ -79.45803344249725, 43.67347461096418 ],
                                [ -79.45853769779205, 43.67347461096418 ],
                                [ -79.45853769779205, 43.67316032905796 ]
                            ], [
                                [ -79.45838212966919, 43.673284490007696 ],
                                [ -79.45821583271027, 43.673284490007696 ],
                                [ -79.45821583271027, 43.67340089066479 ],
                                [ -79.45838212966919, 43.67340089066479 ],
                                [ -79.45838212966919, 43.673284490007696 ]
                            ] ]
                        ]
                    }
                }"#)
                .basic_auth("ingalls", Some("yeahehyeah"))
                .header(reqwest::header::CONTENT_TYPE, "application/json")
                .send()
                .unwrap();

            assert_eq!(resp.text().unwrap(), "true");
            assert!(resp.status().is_success());
        }

        { // Get XML By BBOX
            let mut resp = reqwest::get("http://0.0.0.0:8000/api/0.6/map?bbox=-79.463264,43.670270,-79.456344,43.674693").unwrap();
            assert!(resp.status().is_success());
            assert_eq!(resp.text().unwrap(), r#"<?xml version="1.0" encoding="UTF-8"?><osm version="0.6" generator="ROSM"><node id="1" version="1" lon="-79.460149705" lat="43.672634582"><tag k="addr:housenumber" v="1234"/><tag k="addr:street" v="Main St"/></node><node id="7000000000000000001" version="1" lat="43.672436698" lon="-79.458430409"/><node id="7000000000000000002" version="1" lat="43.672426998" lon="-79.458213151"/><node id="7000000000000000003" version="1" lat="43.673129289" lon="-79.460898042"/><node id="7000000000000000004" version="1" lat="43.671876024" lon="-79.460366964"/><node id="7000000000000000005" version="1" lat="43.672062269" lon="-79.459626675"/><node id="7000000000000000006" version="1" lat="43.672155392" lon="-79.459079504"/><node id="7000000000000000007" version="1" lat="43.67206615" lon="-79.458532333"/><node id="7000000000000000008" version="1" lat="43.672004068" lon="-79.458312392"/><node id="7000000000000000009" version="1" lat="43.671790662" lon="-79.457513094"/><node id="7000000000000000010" version="1" lat="43.673468791" lon="-79.46098119"/><node id="7000000000000000011" version="1" lat="43.673468791" lon="-79.46066469"/><node id="7000000000000000012" version="1" lat="43.673674431" lon="-79.46066469"/><node id="7000000000000000013" version="1" lat="43.673674431" lon="-79.46098119"/><node id="7000000000000000014" version="1" lat="43.672995427" lon="-79.459623992"/><node id="7000000000000000015" version="1" lat="43.672995427" lon="-79.458878338"/><node id="7000000000000000016" version="1" lat="43.673492071" lon="-79.458878338"/><node id="7000000000000000017" version="1" lat="43.673492071" lon="-79.459623992"/><node id="7000000000000000018" version="1" lat="43.673113769" lon="-79.459441602"/><node id="7000000000000000019" version="1" lat="43.673113769" lon="-79.459055364"/><node id="7000000000000000020" version="1" lat="43.67336015" lon="-79.459055364"/><node id="7000000000000000021" version="1" lat="43.67336015" lon="-79.459441602"/><node id="7000000000000000022" version="1" lat="43.673625931" lon="-79.458789825"/><node id="7000000000000000023" version="1" lat="43.673625931" lon="-79.458301663"/><node id="7000000000000000024" version="1" lat="43.673940211" lon="-79.458301663"/><node id="7000000000000000025" version="1" lat="43.673940211" lon="-79.458789825"/><node id="7000000000000000026" version="1" lat="43.673715171" lon="-79.458687901"/><node id="7000000000000000027" version="1" lat="43.673866491" lon="-79.458687901"/><node id="7000000000000000028" version="1" lat="43.673866491" lon="-79.458435774"/><node id="7000000000000000029" version="1" lat="43.673715171" lon="-79.458435774"/><node id="7000000000000000030" version="1" lat="43.673160329" lon="-79.458537698"/><node id="7000000000000000031" version="1" lat="43.673160329" lon="-79.458033442"/><node id="7000000000000000032" version="1" lat="43.673474611" lon="-79.458033442"/><node id="7000000000000000033" version="1" lat="43.673474611" lon="-79.458537698"/><node id="7000000000000000034" version="1" lat="43.67328449" lon="-79.45838213"/><node id="7000000000000000035" version="1" lat="43.67328449" lon="-79.458215833"/><node id="7000000000000000036" version="1" lat="43.673400891" lon="-79.458215833"/><node id="7000000000000000037" version="1" lat="43.673400891" lon="-79.45838213"/><way id="3" version="1"><nd ref="7000000000000000003"/><nd ref="7000000000000000004"/><tag k="highway" v="residential"/><tag k="name" v="Main St E"/></way><way id="8000000000000000001" version="1"><nd ref="7000000000000000005"/><nd ref="7000000000000000006"/><nd ref="7000000000000000007"/></way><way id="8000000000000000002" version="1"><nd ref="7000000000000000008"/><nd ref="7000000000000000009"/></way><way id="5" version="1"><nd ref="7000000000000000010"/><nd ref="7000000000000000011"/><nd ref="7000000000000000012"/><nd ref="7000000000000000013"/><nd ref="7000000000000000010"/><tag k="building" v="yes"/></way><way id="8000000000000000003" version="1"><nd ref="7000000000000000014"/><nd ref="7000000000000000015"/><nd ref="7000000000000000016"/><nd ref="7000000000000000017"/><nd ref="7000000000000000014"/></way><way id="8000000000000000004" version="1"><nd ref="7000000000000000018"/><nd ref="7000000000000000019"/><nd ref="7000000000000000020"/><nd ref="7000000000000000021"/><nd ref="7000000000000000018"/></way><way id="8000000000000000005" version="1"><nd ref="7000000000000000022"/><nd ref="7000000000000000023"/><nd ref="7000000000000000024"/><nd ref="7000000000000000025"/><nd ref="7000000000000000022"/></way><way id="8000000000000000006" version="1"><nd ref="7000000000000000026"/><nd ref="7000000000000000027"/><nd ref="7000000000000000028"/><nd ref="7000000000000000029"/><nd ref="7000000000000000026"/></way><way id="8000000000000000007" version="1"><nd ref="7000000000000000030"/><nd ref="7000000000000000031"/><nd ref="7000000000000000032"/><nd ref="7000000000000000033"/><nd ref="7000000000000000030"/></way><way id="8000000000000000008" version="1"><nd ref="7000000000000000034"/><nd ref="7000000000000000035"/><nd ref="7000000000000000036"/><nd ref="7000000000000000037"/><nd ref="7000000000000000034"/></way><relation id="2" version="1"><tag k="addr:housenumber" v="1234"/><tag k="addr:street" v="yet another street"/><tag k="type" v="multipoint"/><member ref="7000000000000000001" type="node" role="point"/><member ref="7000000000000000002" type="node" role="point"/></relation><relation id="4" version="1"><tag k="highway" v="service"/><tag k="name" v="Don&apos;t drive on me"/><tag k="type" v="multilinestring"/><member ref="8000000000000000001" role="line" type="way"/><member ref="8000000000000000002" role="line" type="way"/></relation><relation id="6" version="1"><tag k="building" v="yes"/><tag k="type" v="multipolygon"/><member ref="8000000000000000003" role="outer" type="way"/><member ref="8000000000000000004" role="inner" type="way"/></relation><relation id="7" version="1"><tag k="amenity" v="hospital"/><tag k="building" v="yes"/><tag k="type" v="multipolygon"/><member ref="8000000000000000005" role="outer" type="way"/><member ref="8000000000000000006" role="inner" type="way"/><member ref="8000000000000000007" role="outer" type="way"/><member ref="8000000000000000008" role="inner" type="way"/></relation></osm>"#);
        }

        server.kill().unwrap();
    }
}
