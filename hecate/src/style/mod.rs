use serde_json::Value;
use crate::err::HecateError;

/// Creates a new GL JS Style under a given user account
///
/// By default styles are private and can only be accessed by a single user
pub fn create(conn: &mut tokio_postgres::Client, uid: i64, style: &str) -> Result<i64, HecateError> {
    match conn.query("
        INSERT INTO styles (name, style, uid, public)
            VALUES (
                COALESCE($1::TEXT::JSON->>'name', 'New Style')::TEXT,
                COALESCE($1::TEXT::JSON->'style', '{}'::JSON),
                $2,
                false
            )
            RETURNING id;
    ", &[&style, &uid]) {
        Ok(rows) => {
            let id = rows.get(0).unwrap().get(0);
            Ok(id)
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

/// Get the style by id, if the style is public, the user need not be logged in,
/// if the style is private ensure the owner is the requester
pub fn get(conn: &mut tokio_postgres::Client, uid: &Option<i64>, style_id: i64) -> Result<Value, HecateError> {
    match conn.query("
        SELECT
            row_to_json(t) as style
        FROM (
            SELECT
                styles.id AS id,
                styles.uid AS uid,
                styles.name AS name,
                styles.style AS style,
                users.username AS username,
                styles.public AS public
            FROM
                styles,
                users
            WHERE
                styles.id = $1
                AND (
                    styles.public IS true
                    OR styles.uid = $2
                )
                AND users.id = styles.uid
        ) t
    ", &[&style_id, &uid]) {
        Ok(rows) => {
            if rows.len() != 1 {
                Err(HecateError::new(404, String::from("Style Not Found"), None))
            } else {
                let style: Value = rows.get(0).unwrap().get(0);

                Ok(style)
            }
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub fn update(conn: &mut tokio_postgres::Client, uid: i64, style_id: i64, style: &str) -> Result<bool, HecateError> {
    match conn.execute("
        UPDATE styles
            SET
                name = COALESCE($3::TEXT::JSON->>'name', name),
                style = COALESCE($3::TEXT::JSONB->'style', style)
            WHERE
                id = $1
                AND uid = $2
    ", &[&style_id, &uid, &style]) {
        Ok(updated) => {
            if updated == 0 {
                Err(HecateError::new(404, String::from("Style Not Found"), None))
            } else {
                Ok(true)
            }
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub fn access(conn: &mut tokio_postgres::Client, uid: i64, style_id: i64, access: bool) -> Result<bool, HecateError> {
    match conn.execute("
        UPDATE styles
            SET
                public = $3
            WHERE
                id = $1
                AND uid = $2
    ", &[&style_id, &uid, &access]) {
        Ok(updated) => {
            if updated == 0 {
                Err(HecateError::new(404, String::from("Style Not Found"), None))
            } else {
                Ok(true)
            }
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

///Allow the owner of a given style to delete it
pub fn delete(conn: &mut tokio_postgres::Client, uid: i64, style_id: i64) -> Result<bool, HecateError> {
    match conn.execute("
        DELETE
            FROM styles
            WHERE
                uid = $1
                AND id = $2
    ", &[&uid, &style_id]) {
        Ok(deleted) => {
            if deleted == 0 {
                Err(HecateError::new(404, String::from("Style Not Found"), None))
            } else {
                Ok(true)
            }
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

///Return a list of all styles (public and private) for a given user
pub fn list_user(conn: &mut tokio_postgres::Client, uid: i64) -> Result<Value, HecateError> {
    match conn.query("
        SELECT
            COALESCE(JSON_Agg(row_to_json(t)), '[]'::JSON)
        FROM (
            SELECT
                styles.id,
                styles.name,
                styles.public,
                styles.uid,
                users.username
            FROM
                styles,
                users
            WHERE
                uid = $1
                AND users.id = uid
            ORDER BY styles.id
        ) t;
    ", &[&uid]) {
        Ok(rows) => {
            if rows.is_empty() {
                Err(HecateError::new(404, String::from("Style Not Found"), None))
            } else {
                let list: Value = rows.get(0).unwrap().get(0);
                Ok(list)
            }
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

///Return a list of public styles for a given user
pub fn list_user_public(conn: &mut tokio_postgres::Client, uid: i64) -> Result<Value, HecateError> {
    match conn.query("
        SELECT
            COALESCE(JSON_Agg(row_to_json(t)), '[]'::JSON)
        FROM (
            SELECT
                styles.id,
                styles.name,
                styles.public,
                styles.uid,
                users.username
            FROM
                styles,
                users
            WHERE
                uid = $1
                AND public IS TRUE
                AND uid = users.id
            ORDER BY styles.id
        ) t;
    ", &[&uid]) {
        Ok(rows) => {
            if rows.is_empty() {
                Err(HecateError::new(404, String::from("Style Not Found"), None))
            } else {
                let list: Value = rows.get(0).unwrap().get(0);
                Ok(list)
            }
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}

pub fn list_public(conn: &mut tokio_postgres::Client) -> Result<Value, HecateError> {
    match conn.query("
        SELECT
            COALESCE(JSON_Agg(row_to_json(t)), '[]'::JSON)
        FROM (
            SELECT
                styles.id,
                styles.name,
                styles.public,
                styles.uid,
                users.username
            FROM
                styles,
                users
            WHERE
                public IS true
                AND uid = users.id
            ORDER BY styles.id
        ) t;
    ", &[]) {
        Ok(rows) => {
            if rows.is_empty() {
                Err(HecateError::new(404, String::from("Style Not Found"), None))
            } else {
                let list: Value = rows.get(0).unwrap().get(0);
                Ok(list)
            }
        },
        Err(err) => Err(HecateError::from_db(err))
    }
}
