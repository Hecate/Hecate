use crate::err::HecateError;
use crate::stream::PGStream;

pub async fn set(conn: &mut tokio_postgres::Client, name: &str, feat: &serde_json::Value) -> Result<bool, HecateError> {
    match conn.execute("
        INSERT INTO bounds (name, geom, props)
            VALUES (
                $1,
                ST_Multi(ST_SetSRID(ST_GeomFromGeoJSON($2::JSON->>'geometry'), 4326)),
                COALESCE(($2::JSON->>'properties')::JSON, '{}'::JSON)::JSONB
            )
            ON CONFLICT (name) DO
                UPDATE
                    SET geom = ST_Multi(ST_SetSRID(ST_GeomFromGeoJSON($2::JSON->>'geometry'), 4326))
                    WHERE bounds.name = $1;
    ", &[ &name, &feat ]).await {
        Ok(_) => Ok(true),
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn delete(conn: &mut tokio_postgres::Client, name: &str) -> Result<bool, HecateError> {
    match conn.execute("
        DELETE FROM bounds WHERE name = $1
    ", &[ &name ]).await {
        Ok(_) => Ok(true),
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn filter(conn: &mut tokio_postgres::Client, prefix: &str, limit: Option<i16>) -> Result<Vec<String>, HecateError> {
    let limit: i16 = match limit {
        None => 100,
        Some(limit) => if limit > 100 { 100 } else { limit }
    };

    match conn.query("
        SELECT name
            FROM bounds
            WHERE name iLIKE $1||'%'
            ORDER BY name
            LIMIT $2::SmallInt
    ", &[ &prefix, &limit ]).await {
        Ok(rows) => {
            let mut names = Vec::<String>::new();

            for row in rows.iter() {
                names.push(row.get(0));
            }

            Ok(names)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn list(conn: &mut tokio_postgres::Client, limit: Option<i16>) -> Result<Vec<String>, HecateError> {
    match conn.query("
        SELECT name
        FROM bounds
        ORDER BY name
        LIMIT $1::SmallInt
    ", &[ &limit ]).await {
        Ok(rows) => {
            let mut names = Vec::<String>::new();

            for row in rows.iter() {
                names.push(row.get(0));
            }

            Ok(names)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn get(conn: tokio_postgres::Client, bounds: String) -> Result<PGStream, HecateError> {
    Ok(PGStream::new(conn, String::from("next_bounds"), String::from(r#"
        DECLARE next_bounds CURSOR FOR
            SELECT
                row_to_json(t)::TEXT
            FROM (
                SELECT
                    geo.id AS id,
                    geo.key AS key,
                    'Feature' AS type,
                    geo.version AS version,
                    ST_AsGeoJSON(geo.geom)::JSON AS geometry,
                    geo.props AS properties
                FROM
                    geo,
                    (
                        SELECT
                            ST_Subdivide(bounds.geom) as subgeom
                        FROM
                            bounds
                        WHERE
                            name = $1
                    ) as b
                WHERE
                    ST_Intersects(geo.geom, b.subgeom)
            ) t
    "#), &[&bounds]).await?)
}

pub async fn meta(conn: &mut tokio_postgres::Client, name: String) -> Result<serde_json::Value, HecateError> {
    match conn.query("
        SELECT
            JSON_Build_Object(
                'id', bounds.id,
                'type', 'Feature',
                'properties', COALESCE(props, '{}'::JSONB),
                'geometry', ST_AsGeoJSON(bounds.geom)::JSON
            )::JSON
            FROM
                bounds
            WHERE
                name = $1
    ", &[ &name ]).await {
        Ok(rows) => {
            if rows.len() != 1 {
                return Err(HecateError::new(404, String::from("bound not found"), None));
            }

            let bound: serde_json::Value = rows.get(0).unwrap().get(0);

            Ok(bound)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub async fn stats_json(conn: &mut tokio_postgres::Client, bounds: String) -> Result<serde_json::Value, HecateError> {
    match conn.query("
        SELECT
            row_to_json(t)
        FROM (
            SELECT
                count(*) AS total,
                json_build_array(
                    ST_XMin(ST_Extent(bounds.geom)),
                    ST_YMin(ST_Extent(bounds.geom)),
                    ST_XMax(ST_Extent(bounds.geom)),
                    ST_YMax(ST_Extent(bounds.geom))
                ) AS bbox,
                to_char(now(), 'YYYY-MM-DD HH24:MI:SS') AS last_calc
            FROM
                geo,
                (
                    SELECT
                        ST_Subdivide(bounds.geom) as subgeom
                    FROM
                        bounds
                    WHERE
                        name = $1
                ) as b,
                bounds
            WHERE
                ST_Intersects(geo.geom, b.subgeom)
                AND bounds.name = $1
        ) t
    ", &[ &bounds ]).await {
        Ok(rows) => Ok(rows.get(0).unwrap().get(0)),
        Err(err) => Err(HecateError::from_db(err))
    }
}
