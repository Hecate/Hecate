#[derive(PartialEq, Debug)]
pub struct HecateError {
    code: u16,
    safe_error: String,
    full_error: String
}

impl HecateError {
    pub fn new(code: u16, safe_error: String, full_error: Option<String>) -> Self {
        let full_error = match full_error {
            Some(err) => err,
            None => safe_error.clone()
        };

        HecateError {
            code: code,
            safe_error: safe_error,
            full_error: full_error
        }
    }

    pub fn from_db(error: postgres::error::Error) -> Self {
        match error.as_db() {
            Some(db_err) => HecateError {
                code: 500,
                safe_error: String::from("Database Error"),
                full_error: format!("{}", db_err)
            },
            None => HecateError {
                code: 500,
                safe_error: String::from("Database Error"),
                full_error: format!("{}", error)
            }
        }
    }

    pub fn as_string(&self) -> String {
        String::new()
    }

    pub fn as_json(&self) -> serde_json::Value {
        json!({
            "code": self.code,
            "status": "Service Unavailable", //TODO: Lookup Status from code
            "reason": self.safe_error
        })
    }
}

use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;

impl <'r> Responder<'r> for HecateError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(self.as_json().to_string()))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}
