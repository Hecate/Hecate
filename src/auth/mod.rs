extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;
extern crate rocket;
extern crate base64;

extern crate serde_json;

use self::rocket::request::{self, FromRequest};
use self::rocket::http::Status;
use self::rocket::{Request, Outcome};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthSchema {
    get: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthUser {
    info: Option<String>,
    create: Option<String>,
    create_session: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthFeature {
    create: Option<String>,
    get: Option<String>,
    history: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthStyle {
    create: Option<String>,
    patch: Option<String>,
    set_public: Option<String>,
    set_private: Option<String>,
    delete: Option<String>,
    get: Option<String>,
    list: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthDelta {
    get: Option<String>,
    list: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthBounds {
    list: Option<String>,
    get: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthOSM {
    get: Option<String>,
    create: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomAuth {
    meta: Option<String>,
    schema: Option<AuthSchema>,
    user: Option<AuthUser>,
    feature: Option<AuthFeature>,
    style: Option<AuthStyle>,
    delta: Option<AuthDelta>,
    bounds: Option<AuthBounds>,
    osm: Option<AuthOSM>
}

impl CustomAuth {
    pub fn new() -> Self {
        CustomAuth {
            meta: Some(String::from("public")),
            schema: Some(AuthSchema {
                get: Some(String::from("public"))
            }),
            user: Some(AuthUser {
                info: Some(String::from("self")),
                create: Some(String::from("public")),
                create_session: Some(String::from("self"))
            }),
            feature: Some(AuthFeature {
                create: Some(String::from("user")),
                get: Some(String::from("public")),
                history: Some(String::from("public"))
            }),
            style: Some(AuthStyle {
                create: Some(String::from("self")),
                patch: Some(String::from("self")),
                set_public: Some(String::from("self")),
                set_private: Some(String::from("self")),
                delete: Some(String::from("self")),
                get: Some(String::from("public")),
                list: Some(String::from("public"))
            }),
            delta: Some(AuthDelta {
                get: Some(String::from("public")),
                list: Some(String::from("public"))
            }),
            bounds: Some(AuthBounds {
                list: Some(String::from("public")),
                get: Some(String::from("public"))
            }),
            osm: Some(AuthOSM {
                get: Some(String::from("public")),
                create: Some(String::from("user"))
            })
        }
    }

    pub fn validate(&self) {
        match self.meta {
            None => (),
            Some(ref meta) => {

            }
        }

        match self.schema {
            None => (),
            Some(ref schema) => {

            }
        }

        match self.user {
            None => (),
            Some(ref user) => {

            }
        }

        match self.feature {
            None => (),
            Some(ref feature) => {

            }
        }

        match self.style {
            None => (),
            Some(ref style) => {

            }
        }

        match self.delta {
            None => (),
            Some(ref delta) => {

            }
        }

        match self.bounds {
            None => (),
            Some(ref bounds) => {

            }
        }

        match self.osm {
            None => (),
            Some(ref osm) => {

            }
        }
    }

    pub fn is_meta(&self, auth: Auth) -> bool {
        true
    }
}

pub struct Auth {
    token: Option<String>,
    basic: Option<(String, String)>
}

pub fn auth(conn: &r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>, auth: Auth) -> Option<i64> {
    if auth.basic != None {
        let (username, password): (String, String) = auth.basic.unwrap();

        match conn.query("
            SELECT id
                FROM users
                WHERE
                    username = $1
                    AND password = crypt($2, password)
        ", &[ &username, &password ]) {
            Ok(res) => {
                if res.len() != 1 { return None; }
                let uid: i64 = res.get(0).get(0);

                Some(uid)
            },
            Err(_) => None
        }
    } else if auth.token != None {
        let token: String = auth.token.unwrap();

        match conn.query("
            SELECT uid
            FROM users_tokens
            WHERE
                token = $1
                AND now() < expiry
        ", &[ &token ]) {
            Ok(res) => {
                if res.len() == 0 { return None; }
                let uid: i64 = res.get(0).get(0);
                Some(uid)
            },
            Err(_) => None
        }
    } else {
        None
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Auth {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Auth, ()> {
        match request.cookies().get_private("session") {
            Some(token) => {
                return Outcome::Success(Auth {
                    token: Some(String::from(token.value())),
                    basic: None
                });
            },
            None => ()
        };

        let keys: Vec<_> = request.headers().get("Authorization").collect();

        if keys.len() != 1 || keys[0].len() < 7 {
            return Outcome::Success(Auth {
                token: None,
                basic: None
            });
        }

        let mut authtype = String::from(keys[0]);
        let auth = authtype.split_off(6);

        if authtype != "Basic " {
            return Outcome::Success(Auth {
                token: None,
                basic: None
            });
        }

        match base64::decode(&auth) {
            Ok(decoded) => match String::from_utf8(decoded) {
                Ok(decoded_str) => {

                    let split = decoded_str.split(":").collect::<Vec<&str>>();

                    if split.len() != 2 { return Outcome::Failure((Status::Unauthorized, ())); }

                    Outcome::Success(Auth {
                        token: None,
                        basic: Some((String::from(split[0]), String::from(split[1])))
                    })
                },
                Err(_) => Outcome::Failure((Status::Unauthorized, ()))
            },
            Err(_) => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
