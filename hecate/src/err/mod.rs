#[derive(PartialEq, Debug)]
pub struct HecateError {
    pub code: u16,
    pub invalidate: bool,
    pub custom_json: Option<serde_json::Value>,
    pub safe_error: String,
    pub full_error: String
}

impl HecateError {
    pub fn new(code: u16, safe_error: String, full_error: Option<String>) -> Self {
        let full_error = match full_error {
            Some(err) => err,
            None => safe_error.to_string()
        };

        HecateError {
            code,
            invalidate: false,
            custom_json: None,
            safe_error,
            full_error
        }
    }

    pub fn set_json(mut self, json: serde_json::Value) -> Self {
        self.custom_json = Some(json);
        self
    }

    pub fn set_invalidate(mut self, invalidate: bool) -> Self {
        self.invalidate = invalidate;
        self
    }

    pub fn generic(code: u16) -> Self {
        let status = actix_web::http::StatusCode::from_u16(code).unwrap();

        let reason = status.canonical_reason().unwrap_or("Generic Error").to_string();

        HecateError {
            code,
            invalidate: false,
            custom_json: None,
            safe_error: reason.clone(),
            full_error: reason
        }

    }

    pub fn from_db(error: postgres::error::Error) -> Self {
        println!("Database Error: {:?}", &error);

        HecateError {
            code: 500,
            custom_json: None,
            invalidate: false,
            safe_error: String::from("Database Error"),
            full_error: format!("{}", error)
        }
    }

    pub fn as_json(&self) -> serde_json::Value {
        match self.custom_json {
            Some(ref custom_json) => custom_json.clone(),
            None => {
                let status = actix_web::http::StatusCode::from_u16(self.code).unwrap();

                json!({
                    "code": self.code,
                    "status": status.canonical_reason(),
                    "reason": self.safe_error
                })
            }
        }
    }

    pub fn as_log(&self) -> String {
        if self.full_error == self.safe_error {
            format!("HecateError: ({:?}): {}", &self.code, &self.safe_error)
        } else {
            format!("HecateError: ({:?}): {}: {}", &self.code, &self.safe_error, &self.full_error)
        }
    }
}

impl actix_http::ResponseError for HecateError {
    fn error_response(&self) -> actix_http::Response {
        println!("{}", self.as_log());

        let mut resp = actix_http::Response::build(actix_web::http::StatusCode::from_u16(self.code).unwrap())
            .json(self.as_json());

        if self.invalidate {
            let cookie = actix_http::http::Cookie::build("session", String::from(""))
                .path("/")
                .http_only(true)
                .finish();

            resp.add_cookie(&cookie).unwrap();
        }

        resp

    }
}

impl std::fmt::Display for HecateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.as_json(), f)
    }
}

impl std::convert::From<actix_http::error::PayloadError> for HecateError {
    fn from(payload: actix_http::error::PayloadError) -> Self {
        HecateError::new(500, String::from("Internal Server Error"), Some(payload.to_string()))
    }
}

impl std::convert::From<actix_threadpool::BlockingError<HecateError>> for HecateError {
    fn from(payload: actix_threadpool::BlockingError<HecateError>) -> Self {
        match payload {
            actix_threadpool::BlockingError::Error(err) => err,
            actix_threadpool::BlockingError::Canceled => HecateError::new(500, String::from("Internal Server Error"), None)
        }
    }
}
