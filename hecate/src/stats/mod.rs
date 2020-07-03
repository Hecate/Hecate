use crate::err::HecateError;

pub async fn get_json(conn: &mut tokio_postgres::Client) -> Result<serde_json::Value, HecateError> {
    match conn.query("
        SELECT COALESCE(row_to_json(d), 'false'::JSON)
        FROM (
            SELECT
                json_build_array(
                    ST_XMin(extent.extent),
                    ST_YMin(extent.extent),
                    ST_XMax(extent.extent),
                    ST_YMax(extent.extent)
                ) AS bbox,
                total.total AS total,
                time.time AS last_calc
            FROM
                (
                    SELECT
                        to_char(last_analyze, 'YYYY-MM-DD HH24:MI:SS') AS time
                    FROM
                        pg_stat_all_tables
                    WHERE
                        relname = 'geo'
                ) as time,
                (
                    SELECT
                        ST_EstimatedExtent('geo', 'geom') AS extent
                ) as extent,
                (
                    SELECT
                        pg_class.reltuples::bigint as total
                    FROM
                        pg_class
                    WHERE
                        oid = 'public.geo'::regclass
                ) as total
        ) d;
    ", &[]).await {
        Err(err) => Err(HecateError::from_db(err)),
        Ok(res) => {
            let d_json: serde_json::Value = res.get(0).unwrap().get(0);
            Ok(d_json)
        }
    }
}

pub async fn regen(conn: &mut tokio_postgres::Client) -> Result<bool, HecateError> {
    match conn.execute("
        ANALYZE geo;
    ", &[]).await {
        Err(err) => Err(HecateError::from_db(err)),
        _ => Ok(true)
    }
}
