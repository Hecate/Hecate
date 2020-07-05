use crate::stream::PGStream;
use crate::err::HecateError;

pub async fn get(conn: tokio_postgres::Client) -> Result<PGStream, HecateError> {
    match PGStream::new(conn, String::from("next_clone"), String::from(r#"
        DECLARE next_clone CURSOR FOR
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
                    geo
            ) t
    "#), &[]).await {
        Ok(stream) => Ok(stream),
        Err(err) =>  Err(err)
    }
}

pub async fn query(read_conn: tokio_postgres::Client, query: &str, limit: &Option<i64>) -> Result<PGStream, HecateError> {
    Ok(PGStream::new(read_conn, String::from("next_clone_query"), format!(r#"
        DECLARE next_clone_query CURSOR FOR
            SELECT
                row_to_json(t)::TEXT
            FROM (
                {}
            ) t
            LIMIT $1

    "#, query), &[&limit]).await?)
}
