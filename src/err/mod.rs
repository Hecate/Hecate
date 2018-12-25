#[derive(PartialEq, Debug)]
pub struct HecateError {
    code: u16,
    custom_json: Option<serde_json::Value>,
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
            custom_json: None,
            safe_error: safe_error,
            full_error: full_error
        }
    }

    pub fn from_json(code: u16, json: serde_json::Value, safe_error: String, full_error: Option<String>) -> Self {
        let full_error = match full_error {
            Some(err) => err,
            None => safe_error.clone()
        };

        HecateError {
            code: code,
            custom_json: Some(json),
            safe_error: safe_error,
            full_error: full_error
        }
    }

    pub fn from_db(error: postgres::error::Error) -> Self {
        match error.as_db() {
            Some(db_err) => HecateError {
                code: 500,
                custom_json: None,
                safe_error: String::from("Database Error"),
                full_error: format!("{}", db_err)
            },
            None => HecateError {
                code: 500,
                custom_json: None,
                safe_error: String::from("Database Error"),
                full_error: format!("{}", error)
            }
        }
    }

    pub fn as_string(&self) -> String {
        self.safe_error.clone()
    }

    pub fn as_json(self) -> serde_json::Value {
        match self.custom_json {
            Some(custom_json) => custom_json,
            None => {
                let status = rocket::http::Status::from_code(self.code).unwrap();

                json!({
                    "code": self.code,
                    "status": status.reason,
                    "reason": self.safe_error
                })
            }
        }
    }

    pub fn as_wfs(self) -> String {
        format!(r#"
            <?xml version="1.0" ?>
            <ExceptionReport version="1.1.0" xmlns="http://www.opengis.net/ogc" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:schemaLocation="owsExceptionReport.xsd">
                <Exception code="{}" locator="">
                <ExceptionText>{}</ExceptionText>
            </Exception>
        "#, &self.code, &self.safe_error)
    }
}

use std::io::Cursor;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;

impl <'r> Responder<'r> for HecateError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let status = rocket::http::Status::from_code(self.code).unwrap();
        let body = self.as_json().to_string();

        println!("HecateError: {:?}", &body);

        Ok(Response::build()
            .status(status)
            .sized_body(Cursor::new(body))
            .header(ContentType::new("application", "json"))
            .finalize())
    }
}
