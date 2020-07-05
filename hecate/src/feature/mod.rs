use crate::stream::PGStream;
use crate::err::HecateError;
use crate::validate;

#[derive(PartialEq, Debug)]
pub enum Action {
    Create,
    Modify,
    Delete,
    Restore
}

#[derive(PartialEq, Debug)]
pub struct Response {
    pub old: Option<i64>,
    pub new: Option<i64>,
    pub version: Option<i64>
}

pub fn import_error(feat: &geojson::Feature, error: String, full: Option<String>) -> HecateError {
    HecateError::new(400, String::from("Import Error"), full)
        .set_json(json!({
            "id": feat.id,
            "message": error,
            "feature": feat
        }))
}

///
/// Check if the feature has the force: true flag set and if so
/// validate that it meets the force:true acceptions
///
pub fn is_force(feat: &geojson::Feature) -> Result<bool, HecateError> {
    match feat.foreign_members {
        None => Ok(false),
        Some(ref members) => match members.get("force") {
            Some(force) => {
                match force.as_bool() {
                    Some(true) => {
                        if get_action(&feat)? != Action::Create {
                            return Err(import_error(&feat, String::from("force can only be used on create"), None));
                        }

                        match get_key(&feat)? {
                            None => {
                                Err(import_error(&feat, String::from("force can only be used with a key value"), None))
                            },
                            Some(_) => {
                                Ok(true)
                            }
                        }
                    },
                    Some(false) => Ok(false),
                    None => Err(import_error(&feat, String::from("force must be a boolean"), None))
                }
            },
            None => Ok(false)
        }
    }
}

pub fn del_version(feat: &mut geojson::Feature) {
    match feat.foreign_members {
        None => (),
        Some(ref mut members) => {
            members.remove("version");
        }
    }
}

pub fn get_version(feat: &geojson::Feature) -> Result<i64, HecateError> {
    match feat.foreign_members {
        None => Err(import_error(&feat, String::from("Version Required"), None)),
        Some(ref members) => match members.get("version") {
            Some(version) => {
                match version.as_i64() {
                    Some(version) => Ok(version),
                    None => Err(import_error(&feat, String::from("Version Required"), None))
                }
            },
            None => Err(import_error(&feat, String::from("Version Required"), None))
        }
    }
}

pub fn get_geom_str(feat: &geojson::Feature) -> Result<String, HecateError> {
    let geom = match feat.geometry {
        None => { return Err(import_error(&feat, String::from("Geometry Required"), None)); },
        Some(ref geom) => geom
    };

    fn is_valid_coord(feat: &geojson::Feature, pt: &geojson::PointType) -> Result<bool, HecateError> {
        if pt.len() > 3 {
            return Err(import_error(&feat, String::from("Coordinate Array has > 3 coords"), None));
        }

        let lon = pt[0];
        let lat = pt[1];

        if lon < -180.0 {
            Err(import_error(&feat, String::from("longitude < -180"), None))
        } else if lon > 180.0 {
            Err(import_error(&feat, String::from("longitude > 180"), None))
        } else if lat < -90.0 {
            Err(import_error(&feat, String::from("latitude < -90"), None))
        } else if lat > 90.0 {
            Err(import_error(&feat, String::from("latitude > 90"), None))
        } else {
            Ok(true)
        }
    }

    //Ensure coordinates are within bounds
    match geom.value {
        geojson::Value::Point(ref pt) => {
            is_valid_coord(&feat, &pt)?;
        },
        geojson::Value::MultiPoint(ref pts) => {
            for pt in pts {
                is_valid_coord(&feat, &pt)?;
            }
        },
        geojson::Value::LineString(ref ln) => {
            for pt in ln {
                is_valid_coord(&feat, &pt)?;
            }
        },
        geojson::Value::MultiLineString(ref lns) => {
            for ln in lns {
                for pt in ln {
                    is_valid_coord(&feat, &pt)?;
                }
            }
        },
        geojson::Value::Polygon(ref ply) => {
            for pl in ply {
                for pt in pl {
                    is_valid_coord(&feat, &pt)?;
                }
            }
        },
        geojson::Value::MultiPolygon(ref plys) => {
            for ply in plys {
                for pl in ply {
                    for pt in pl {
                        is_valid_coord(&feat, &pt)?;
                    }
                }
            }
        },
        _ => ()
    };

    match serde_json::to_string(&geom) {
        Ok(geom) => Ok(geom),
        Err(err) => Err(import_error(&feat, String::from("Failed to stringify geometry"), Some(err.to_string())))
    }
}

pub fn get_id(feat: &geojson::Feature) -> Result<i64, HecateError> {
    match feat.id {
        None => Err(import_error(&feat, String::from("ID Required"), None)),
        Some(ref id) => match id {
            geojson::feature::Id::Number(id) => {
                if id.is_i64() {
                    Ok(id.as_i64().unwrap())
                } else {
                    Err(import_error(&feat, String::from("Integer ID Required"), None))
                }
            },
            _ => Err(import_error(&feat, String::from("Integer ID Required"), None))
        }
    }
}

pub fn get_action(feat: &geojson::Feature) -> Result<Action, HecateError> {
    match feat.foreign_members {
        None => Err(import_error(&feat, String::from("Action Required"), None)),
        Some(ref members) => match members.get("action") {
            Some(action) => {
                match action.as_str() {
                    Some("create") => Ok(Action::Create),
                    Some("modify") => Ok(Action::Modify),
                    Some("delete") => Ok(Action::Delete),
                    Some("restore") => Ok(Action::Restore),
                    _ => Err(import_error(&feat, String::from("Action Required"), None))
                }
            },
            None => Err(import_error(&feat, String::from("Action Required"), None))
        }
    }
}

pub fn get_key(feat: &geojson::Feature) -> Result<Option<String>, HecateError> {
    match feat.foreign_members {
        None => Ok(None),
        Some(ref members) => {
            match members.get("key") {
                None => Ok(None),
                Some(key) => {
                    if key.is_null() { return Ok(None); }

                    match key.as_str() {
                        Some(ref key) => Ok(Some(String::from(*key))),
                        None => Err(import_error(&feat, String::from("key must be a string value"), None))
                    }
                }
            }
        }
    }
}

pub async fn action(trans: &mut tokio_postgres::Transaction<'_>, schema_json: &Option<serde_json::value::Value>, feat: &geojson::Feature, delta: &Option<i64>) -> Result<Response, HecateError> {
    let action = get_action(&feat)?;

    let mut scope = valico::json_schema::Scope::new();
    let schema = match *schema_json {
        Some(ref schema) => {
            match scope.compile_and_return(schema.clone(), false) {
                Ok(schema) => Some(schema),
                Err(_) => { return Err(HecateError::new(400, String::from("Schema Error"), None)); }
            }
        },
        None => None
    };

    let res = match action {
        Action::Create => create(trans, &schema, &feat, &delta).await?,
        Action::Modify => modify(trans, &schema, &feat, &delta).await?,
        Action::Restore => restore(trans, &schema, &feat, &delta).await?,
        Action::Delete => delete(trans, &feat, &delta).await?
    };

    Ok(res)
}

pub async fn create(trans: &mut tokio_postgres::Transaction<'_>, schema: &Option<valico::json_schema::schema::ScopedSchema<'_>>, feat: &geojson::Feature, delta: &Option<i64>) -> Result<Response, HecateError> {
    if get_version(&feat).is_ok() {
        return Err(import_error(&feat, String::from("Cannot have Version"), None));
    }

    let props = match feat.properties {
        None => { return Err(import_error(&feat, String::from("Properties Required"), None)); },
        Some(ref props) => props
    };

    let valid = match *schema {
        Some(ref schema) => schema.validate(&json!(props)).is_valid(),
        None => true
    };

    if !valid { return Err(import_error(&feat, String::from("Failed to Match Schema"), None)) };

    let geom_str = get_geom_str(&feat)?;

    let props_str = match serde_json::to_string(&props) {
        Ok(props) => props,
        Err(err) => { return Err(import_error(&feat, String::from("Failed to stringify properties"), Some(err.to_string()))) }
    };

    let key = get_key(&feat)?;

    let id: Option<i64> = match get_id(&feat) {
        Err(_) => None,
        Ok(id) => Some(id)
    };

    if is_force(&feat)? {
        match trans.query("
            INSERT INTO geo (version, geom, props, deltas, key)
                VALUES (
                    1,
                    ST_SetSRID(ST_GeomFromGeoJSON($1), 4326),
                    $2::TEXT::JSON,
                    array[COALESCE($3, currval('deltas_id_seq')::BIGINT)],
                    $4
                )
                ON CONFLICT (key) DO UPDATE
                    SET
                        version = geo.version + 1,
                        geom = ST_SetSRID(ST_GeomFromGeoJSON($1), 4326),
                        props = $2::TEXT::JSON,
                        deltas = array_append(geo.deltas, COALESCE($3, currval('deltas_id_seq')::BIGINT))
                RETURNING id, version;
        ", &[&geom_str, &props_str, &delta, &key]).await {
            Ok(res) => {
                let new: i64 = res.get(0).unwrap().get(0);
                let version: i64 = res.get(0).unwrap().get(1);

                match trans.query("
                    INSERT INTO geo_history (action, geom, props, delta, key, id, version)
                        VALUES (
                            'create',
                            ST_SetSRID(ST_GeomFromGeoJSON($1), 4326),
                            $2::TEXT::JSON,
                            COALESCE($3, currval('deltas_id_seq')::BIGINT),
                            $4,
                            $5,
                            $6
                        )
                ", &[&geom_str, &props_str, &delta, &key, &new, &version]).await {
                    Err(err) => Err(import_error(&feat, err.to_string(), None)),
                    Ok(_) => {
                        Ok(Response {
                            old: id,
                            new: Some(new),
                            version: Some(version)
                        })
                    }
                }
            },
            Err(err) => Err(import_error(&feat, err.to_string(), None)),
        }
    } else {
        match trans.query("
            INSERT INTO geo (version, geom, props, deltas, key)
                VALUES (
                    1,
                    ST_SetSRID(ST_GeomFromGeoJSON($1), 4326),
                    $2::TEXT::JSON,
                    array[COALESCE($3, currval('deltas_id_seq')::BIGINT)],
                    $4
                ) RETURNING id, version;
        ", &[&geom_str, &props_str, &delta, &key]).await {
            Ok(res) => {
                let new: i64 = res.get(0).unwrap().get(0);
                let version: i64 = res.get(0).unwrap().get(1);

                match trans.query("
                    INSERT INTO geo_history (action, geom, props, delta, key, id, version)
                        VALUES (
                            'create',
                            ST_SetSRID(ST_GeomFromGeoJSON($1), 4326),
                            $2::TEXT::JSON,
                            COALESCE($3, currval('deltas_id_seq')::BIGINT),
                            $4,
                            $5,
                            $6
                        ) RETURNING version;
                ", &[&geom_str, &props_str, &delta, &key, &new, &version]).await {
                    Err(err) => Err(import_error(&feat, err.to_string(), None)),
                    Ok(_) => {
                        Ok(Response {
                            old: id,
                            new: Some(new),
                            version: Some(version)
                        })
                    }
                }
            },
            Err(err) => {
                let err = err.to_string();
                if err == String::from("duplicate key value violates unique constraint \"geo_key_key\"") {
                    Err(import_error(&feat, String::from("Duplicate Key Value"), None))
                } else {
                    Err(import_error(&feat, err, None))
                }
            }
        }
    }
}

pub async fn modify(trans: &mut tokio_postgres::Transaction<'_>, schema: &Option<valico::json_schema::schema::ScopedSchema<'_>>, feat: &geojson::Feature, delta: &Option<i64>) -> Result<Response, HecateError> {
    let props = match feat.properties {
        None => { return Err(import_error(&feat, String::from("Properties Required"), None)); },
        Some(ref props) => props
    };

    let valid = match *schema {
        Some(ref schema) => schema.validate(&json!(props)).is_valid(),
        None => true
    };

    if !valid { return Err(import_error(&feat, String::from("Failed to Match Schema"), None)) };

    let id = get_id(&feat)?;
    let version = get_version(&feat)?;
    let key = get_key(&feat)?;

    let geom_str = get_geom_str(&feat)?;

    let props_str = match serde_json::to_string(&props) {
        Ok(props) => props,
        Err(err) => { return Err(import_error(&feat, String::from("Failed to stringify properties"), Some(err.to_string()))) }
    };

    match trans.query("
            SELECT modify_geo($1, $2, COALESCE($5, currval('deltas_id_seq')::BIGINT), $3, $4, $6);
        ", &[&geom_str, &props_str, &id, &version, &delta, &key]).await {
        Ok(_) => {
            match trans.query("
                INSERT INTO geo_history (geom, props, id, version, delta, key, action)
                    VALUES (
                        ST_SetSRID(ST_GeomFromGeoJSON($1), 4326),
                        $2::TEXT::JSON,
                        $3,
                        $4::BIGINT + 1,
                        COALESCE($5, currval('deltas_id_seq')::BIGINT),
                        $6,
                        'modify'
                    ) RETURNING version;
            ", &[&geom_str, &props_str, &id, &version, &delta, &key]).await {
                Ok(res) => {
                    let new_version: i64 = res.get(0).unwrap().get(0);
                    Ok(Response {
                        old: Some(id),
                        new: Some(id),
                        version: Some(new_version)
                    })
                },
                Err(err) => {
                    let err = err.to_string();
                    if err == "MODIFY: ID or VERSION Mismatch" {
                        Err(import_error(&feat, String::from("Modify Version Mismatch"), None))
                    } else {
                        Err(import_error(&feat, err, None))
                    }
                }
            }
        },
        Err(err) => {
            let err = err.to_string();
            if err == String::from("MODIFY: ID or VERSION Mismatch") {
                Err(import_error(&feat, String::from("Modify Version Mismatch"), None))
            } else if err == String::from("duplicate key value violates unique constraint \"geo_key_key\"") {
                Err(import_error(&feat, String::from("Duplicate Key Value"), None))
            } else {
                Err(import_error(&feat, err, None))
            }
        }
    }
}

pub async fn delete(trans: &mut tokio_postgres::Transaction<'_>, feat: &geojson::Feature, delta: &Option<i64>) -> Result<Response, HecateError> {
    let id = get_id(&feat)?;
    let version = get_version(&feat)?;
    let key = get_key(&feat)?;

    match trans.query("SELECT delete_geo($1, $2);", &[&id, &version]).await {
        Ok(_) => {
            match trans.query("
                INSERT INTO geo_history (id, version, delta, key, action)
                    VALUES (
                        $1,
                        $2::BIGINT + 1,
                        COALESCE($3, currval('deltas_id_seq')::BIGINT),
                        $4,
                        'delete'
                    );
            ", &[&id, &version, &delta, &key]).await {
                Ok(_) => {
                    Ok(Response {
                        old: Some(id),
                        new: None,
                        version: None
                    })
                },
                Err(err) => {
                    let err = err.to_string();
                    if err == String::from("DELETE: ID or VERSION Mismatch") {
                        Err(import_error(&feat, String::from("Delete Version Mismatch"), None))
                    } else {
                        Err(import_error(&feat, err, None))
                    }
                }
            }
        },
        Err(err) => {
            let err = err.to_string();
            if err == String::from("DELETE: ID or VERSION Mismatch") {
                Err(import_error(&feat, String::from("Delete Version Mismatch"), None))
            } else {
                Err(import_error(&feat, err, None))
            }
        }
    }
}

pub async fn query_by_key(conn: &mut tokio_postgres::Client, key: &str) -> Result<serde_json::value::Value, HecateError> {
    match conn.query("
        SELECT
            row_to_json(f)::JSON AS feature
        FROM (
            SELECT
                id AS id,
                key AS key,
                'Feature' AS type,
                version AS version,
                ST_AsGeoJSON(geom)::JSON AS geometry,
                props AS properties
            FROM geo
            WHERE key = $1
        ) f;
    ", &[&key]).await {
        Ok(res) => {
            if res.len() != 1 { return Err(HecateError::new(404, String::from("Feature not found"), None)); }

            let feat: serde_json::value::Value = res.get(0).unwrap().get(0);
            Ok(feat)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn query_by_point(conn: &mut tokio_postgres::Client, point: &str) -> Result<Vec<serde_json::value::Value>, HecateError> {
    let (lng, lat) = validate::point(point)?;

    match conn.query("
        SELECT
            row_to_json(f)::JSON AS feature
        FROM (
            SELECT
                id AS id,
                key AS key,
                'Feature' AS type,
                version AS version,
                ST_AsGeoJSON(geom)::JSON AS geometry,
                props AS properties
            FROM geo
            WHERE
                ST_DWithin(ST_SetSRID(ST_MakePoint($1, $2), 4326), geo.geom, 0.00005)
            ORDER BY
                ST_Distance(ST_SetSRID(ST_MakePoint($1, $2), 4326), geo.geom) DESC
        ) f
    ", &[&lng, &lat]).await {
        Ok(results) => {
            if results.is_empty() {
                return Err(HecateError::new(404, String::from("Feature not found"), None));
            }

            let mut feats: Vec<serde_json::value::Value> = Vec::with_capacity(results.len());

            for result in results.iter() {
                let feat: serde_json::value::Value = result.get(0);
                feats.push(feat);
            }

            Ok(feats)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn get(conn: &mut tokio_postgres::Client, id: i64) -> Result<geojson::Feature, HecateError> {
    match conn.query("
        SELECT
            row_to_json(f)::TEXT AS feature
        FROM (
            SELECT
                id AS id,
                key AS key,
                'Feature' AS type,
                version AS version,
                ST_AsGeoJSON(geom)::JSON AS geometry,
                props AS properties
            FROM geo
            WHERE id = $1
        ) f;
    ", &[&id]).await {
        Ok(res) => {
            if res.len() != 1 { return Err(HecateError::new(404, String::from("Not Found"), None)); }

            let feat: &postgres::row::Row = res.get(0).unwrap();
            let feat: String = feat.get(0);
            let feat: geojson::Feature = match feat.parse() {
                Ok(feat) => match feat {
                    geojson::GeoJson::Feature(feat) => feat,
                    _ => { return Err(HecateError::new(400, String::from("Invalid Feature"), None)); }
                },
                Err(_) => { return Err(HecateError::new(400, String::from("Invalid Feature"), None)); }
            };

            Ok(feat)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn restore(trans: &mut tokio_postgres::Transaction<'_>, schema: &Option<valico::json_schema::schema::ScopedSchema<'_>>, feat: &geojson::Feature, delta: &Option<i64>) -> Result<Response, HecateError> {
    let props = match feat.properties {
        None => { return Err(import_error(&feat, String::from("Properties Required"), None)); },
        Some(ref props) => props
    };

    let valid = match *schema {
        Some(ref schema) => schema.validate(&json!(props)).is_valid(),
        None => true
    };

    if !valid { return Err(import_error(&feat, String::from("Failed to Match Schema"), None)) };

    let id = get_id(&feat)?;
    let version = get_version(&feat)?;
    let key = get_key(&feat)?;

    let geom_str = get_geom_str(&feat)?;

    let props_str = match serde_json::to_string(&props) {
        Ok(props) => props,
        Err(_) => { return Err(import_error(&feat, String::from("Failed to stringify properties"), None)) }
    };

    //Get the previous version, action, and all deltas of a given feature
    match trans.query("
    SELECT
        ARRAY_AGG(delta ORDER BY delta) as delta_ids,
        MAX(version) as max_version,
        (ARRAY_AGG(action ORDER BY version DESC))[1] as action
    FROM (
        SELECT
            delta,
            version,
            action
        FROM
            geo_history
        WHERE
            id=$1
    ) t
    ", &[&id]).await {
        Ok(res) => {
            // Agg function returns row of NULLs if inner query doesn't return results
            let affected: Option<Vec<i64>> = res.get(0).unwrap().get(0);
            let affected = match affected {
                None => return Err(import_error(&feat, String::from("Feature Not Found"), None)),
                Some(deltas) => deltas
            };

            let prev_version: i64 = res.get(0).unwrap().get(1);
            // previous version of a feature must match version passed in request
            if prev_version != version {
                return Err(import_error(&feat, String::from("Restore Version Mismatch"), None));
            }

            let prev_action: String = res.get(0).unwrap().get(2);
            if prev_action != "delete" {
                return Err(import_error(&feat, String::from("Feature Not In Deleted State"), None));
            }

            //Create Delta History Array
            match trans.query("
                INSERT INTO geo (id, version, geom, props, deltas, key)
                    VALUES (
                        $1::BIGINT,
                        $2::BIGINT + 1,
                        ST_SetSRID(ST_GeomFromGeoJSON($3), 4326),
                        $4::TEXT::JSON,
                        array_append($5::BIGINT[], COALESCE($6, currval('deltas_id_seq')::BIGINT)),
                        $7
                    ) RETURNING version;
            ", &[&id, &prev_version, &geom_str, &props_str, &affected, &delta, &key]).await {
                Ok(res) => {
                    let new_version: i64 = res.get(0).unwrap().get(0);
                    match trans.query("
                        INSERT INTO geo_history (geom, props, id, version, delta, key, action)
                            VALUES (
                                ST_SetSRID(ST_GeomFromGeoJSON($1), 4326),
                                $2::TEXT::JSON,
                                $3,
                                $4,
                                COALESCE($5, currval('deltas_id_seq')::BIGINT),
                                $6,
                                'restore'
                            );
                    ", &[&geom_str, &props_str, &id, &new_version, &delta, &key]).await {
                        Ok(_) => {
                            Ok(Response {
                                old: Some(id),
                                new: Some(id),
                                version: Some(new_version)
                            })
                        },
                        Err(err) => Err(import_error(&feat, err.to_string(), None))
                    }
                },
                Err(err) => {
                    let err = err.to_string();
                    if err == String::from("duplicate key value violates unique constraint \"geo_id_key\"") {
                        Err(import_error(&feat, String::from("Feature Not In Deleted State"), None))
                    } else if err == String::from("duplicate key value violates unique constraint \"geo_key_key\"") {
                        Err(import_error(&feat, String::from("Duplicate Key Value"), None))
                    } else {
                        Err(import_error(&feat, err, None))
                    }
                }
            }
        },
        Err(err) => {
            Err(import_error(&feat, String::from("Error Fetching History"), Some(err.to_string())))
        }
    }
}

pub async fn get_point_stream(conn: tokio_postgres::Client, point: &str) -> Result<PGStream, HecateError> {
    let (lng, lat) = validate::point(point)?;

    Ok(PGStream::new(conn, String::from("next_features"), String::from(r#"
        DECLARE next_features CURSOR FOR
            SELECT
                row_to_json(f)::TEXT AS feature
            FROM (
                SELECT
                    id AS id,
                    key AS key,
                    'Feature' AS type,
                    version AS version,
                    ST_AsGeoJSON(geom)::JSON AS geometry,
                    props AS properties
                FROM geo
                WHERE
                    ST_DWithin(ST_SetSRID(ST_MakePoint($1, $2), 4326), geo.geom, 0.00005)
                ORDER BY
                    ST_Distance(ST_SetSRID(ST_MakePoint($1, $2), 4326), geo.geom) DESC
            ) f;
    "#), &[&lng, &lat]).await?)
}

pub async fn get_bbox_stream(conn: tokio_postgres::Client, bbox: &[f64]) -> Result<PGStream, HecateError> {
    validate::bbox(bbox)?;

    Ok(PGStream::new(conn, String::from("next_features"), String::from(r#"
        DECLARE next_features CURSOR FOR
            SELECT
                row_to_json(f)::TEXT AS feature
            FROM (
                SELECT
                    id AS id,
                    key AS key,
                    'Feature' AS type,
                    version AS version,
                    ST_AsGeoJSON(geom)::JSON AS geometry,
                    props AS properties
                FROM geo
                WHERE
                    ST_Intersects(geom, ST_MakeEnvelope($1, $2, $3, $4, 4326))
                    OR ST_Within(geom, ST_MakeEnvelope($1, $2, $3, $4, 4326))
            ) f;
    "#), &[&bbox[0], &bbox[1], &bbox[2], &bbox[3]]).await?)
}

pub async fn get_bbox(conn: &mut tokio_postgres::Client, bbox: Vec<f64>) -> Result<geojson::FeatureCollection, HecateError> {
    validate::bbox(&bbox)?;

    match conn.query("
        SELECT
            row_to_json(f)::TEXT AS feature
        FROM (
            SELECT
                id AS id,
                key AS key,
                'Feature' AS type,
                version AS version,
                ST_AsGeoJSON(geom)::JSON AS geometry,
                props AS properties
            FROM geo
            WHERE
                ST_Intersects(geom, ST_MakeEnvelope($1, $2, $3, $4, 4326))
                OR ST_Within(geom, ST_MakeEnvelope($1, $2, $3, $4, 4326))
        ) f;
    ", &[&bbox[0], &bbox[1], &bbox[2], &bbox[3]]).await {
        Ok(res) => {
            let mut fc = geojson::FeatureCollection {
                bbox: None,
                features: vec![],
                foreign_members: None
            };

            for row in res.iter() {
                let feat: String = row.get(0);
                let feat: geojson::Feature = match feat.parse() {
                    Ok(geojson::GeoJson::Feature(feat)) => feat,
                    _ => { return Err(HecateError::new(400, String::from("Invalid Feature"), None)); }
                };

                fc.features.push(feat);
            }

            Ok(fc)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

///Get the history of a particular feature
pub async fn history(conn: &mut tokio_postgres::Client, feat_id: i64) -> Result<serde_json::Value, HecateError> {
    match conn.query("
        SELECT json_agg (
            JSON_Build_Object(
                'id', deltas.id,
                'uid', deltas.uid,
                'feat', JSON_Build_Object(
                    'id', geo_history.id,
                    'action', geo_history.action,
                    'key', geo_history.key,
                    'type', 'Feature',
                    'version', geo_history.version,
                    'geometry', ST_AsGeoJSON(geom)::JSON,
                    'properties', geo_history.props
                ),
                'username', users.username
            )
            ORDER BY geo_history.version DESC
        )
        FROM
            geo_history,
            deltas,
            users
        WHERE
            geo_history.id = $1 AND
            geo_history.delta = deltas.id AND
            deltas.uid = users.id;
    ", &[&feat_id]).await {
        Ok(res) => {
            if res.is_empty() {
                return Err(HecateError::new(400, String::from("Could not find history for given id"), None))
            }
            let history: serde_json::Value = res.get(0).unwrap().get(0);

            Ok(history)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn get_point_history_stream(conn: tokio_postgres::Client, point: &str) -> Result<PGStream, HecateError> {
    let (lng, lat) = validate::point(point)?;

    Ok(PGStream::new(conn, String::from("next_features"), String::from(r#"
        DECLARE next_features CURSOR FOR
            SELECT
                row_to_json(f)::TEXT AS feature
            FROM (
                SELECT
                    id,
                    action,
                    key,
                    delta,
                    'Feature' AS type,
                    version,
                    ST_AsGeoJSON(geom)::JSON AS geometry,
                    props AS properties
                FROM geo_history
                WHERE
                    ST_DWithin(ST_SetSRID(ST_MakePoint($1, $2), 4326), geom, 0.00005)
                ORDER BY
                    ST_Distance(ST_SetSRID(ST_MakePoint($1, $2), 4326), geom) DESC
            ) f;
    "#), &[&lng, &lat]).await?)
}

pub async fn get_bbox_history_stream(conn: tokio_postgres::Client, bbox: &[f64]) -> Result<PGStream, HecateError> {
    validate::bbox(bbox)?;

    Ok(PGStream::new(conn, String::from("next_features"), String::from(r#"
        DECLARE next_features CURSOR FOR
            SELECT
                row_to_json(f)::TEXT AS feature
            FROM (
                SELECT
                    id,
                    action,
                    key,
                    delta,
                    'Feature' AS type,
                    version,
                    ST_AsGeoJSON(geom)::JSON AS geometry,
                    props AS properties
                FROM geo_history
                WHERE
                    ST_Intersects(geom, ST_MakeEnvelope($1, $2, $3, $4, 4326))
                    OR ST_Within(geom, ST_MakeEnvelope($1, $2, $3, $4, 4326))
            ) f;
    "#), &[&bbox[0], &bbox[1], &bbox[2], &bbox[3]]).await?)
}
