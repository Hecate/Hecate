use crate::err::HecateError;
use serde_json::Value;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Layer {
    pub id: i64,
    pub name: String,
    pub value: Value
}

impl Layer {
    pub fn new(id: i64, name: String, value: Value) -> Self {
        Layer {
            id,
            name,
            value
        }
    }

    pub fn get(conn: &impl postgres::GenericConnection, name: &str) -> Result<Self, HecateError> {
        match conn.query("
            SELECT
                id,
                name,
                value::JSON
            FROM
                layers
            WHERE
                name = 1;
        ", &[&name]) {
            Ok(rows) => {
                if rows.is_empty() {
                    Err(HecateError::new(404, String::from("Layer not found"), None))
                } else {
                    Ok(Layer::new(
                        rows.get(0).get(0),
                        rows.get(0).get(1),
                        rows.get(0).get(2)
                    ))
                }
            },
            Err(err) => Err(HecateError::from_db(err))
        }
    }
}

pub fn list(conn: &impl postgres::GenericConnection) -> Result<Vec<String>, HecateError> {
    match conn.query("
        SELECT
            name
        FROM
            layers
        ORDER BY
            name
    ", &[]) {
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
