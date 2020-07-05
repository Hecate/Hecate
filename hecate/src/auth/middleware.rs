use actix_service::{Service, Transform};
use actix_web::{http, dev::ServiceRequest, dev::ServiceResponse, Error, HttpResponse};
use futures::future::{ok, Ready};
use std::task::Poll;
use crate::db::DbReplica;
use crate::user::token::Scope;
use std::task::Context;
use super::{Auth, ServerAuthDefault, AuthAccess};
use std::future::Future;
use std::pin::Pin;

#[derive(Clone)]
pub struct EnforceAuth {
    db: DbReplica,
    auth: super::ServerAuthDefault
}

impl EnforceAuth {
    pub fn new(db: DbReplica, auth: super::ServerAuthDefault) -> EnforceAuth {
        EnforceAuth {
            db,
            auth
        }
    }
}

impl<S, B> Transform<S> for EnforceAuth
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = EnforceAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(EnforceAuthMiddleware {
            service,
            db: self.db.clone(),
            auth: self.auth.clone()
        })
    }
}

pub struct EnforceAuthMiddleware<S> {
    service: S,
    db: DbReplica,
    auth: super::ServerAuthDefault
}

impl<S, B> Service for EnforceAuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let path: Vec<String> = req.path().split("/").map(|p| {
            p.to_string()
        }).filter(|p| {
            if p.len() == 0 {
                return false;
            }

            true
        }).collect();

        let auth = {
            let mut db = match futures::executor::block_on(self.db.get()) {
                Ok(db) => db,
                Err(_err) => {
                    return Box::pin(async move {
                        Ok(req.into_response(HttpResponse::InternalServerError().finish().into_body()))
                    })
                }
            };

            futures::executor::block_on(Auth::from_sreq(&mut req, &mut db))
        };

        let mut auth = match auth {
            Err(err) => {
                if err.invalidate {
                    let cookie = actix_http::http::Cookie::build("session", String::from(""))
                        .path("/")
                        .http_only(true)
                        .finish();

                    // Invalid cookies to the UI should remove session and
                    // redirect to login
                    if
                        (
                            path.len() == 1
                            && path[0] == String::from("admin")
                        ) || (
                            path.len() >= 2
                            && path[0] == String::from("admin")
                            && path[1] != String::from("login")
                        )
                    {
                        return Box::pin(async move {
                            Ok(req.into_response(HttpResponse::Found().cookie(cookie).header(http::header::LOCATION, "/admin/login/index.html").finish().into_body()))
                        });
                    }
                }

                return Box::pin(async move {
                    Ok(req.into_response( HttpResponse::Unauthorized().finish().into_body()))
                });
            },
            Ok(auth) => auth
        };

        // Disabled accounts should always 401
        if auth.access == AuthAccess::Disabled {
            return Box::pin(async move {
                Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
            });
        }


        // If no default auth is set - allow all api endpoints
        if self.auth == ServerAuthDefault::Public {
            auth.scope = Scope::Full;
            auth.as_headers(&mut req);
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        //Session Management is always allowed
        if req.path() == "/api/user/session" || req.path() == "/" {
            auth.as_headers(&mut req);
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            });
        }

        if
            auth.uid.is_none()
            || self.auth == ServerAuthDefault::Admin && auth.access == AuthAccess::Admin
        {
            if path.len() >= 1 && path[0] == String::from("admin") {
                // UI Results should redirect to an unauthenticated login portal
                // or allowed if they are for the login page

                if path.len() >= 2 && path[1] == String::from("login") {
                    let fut = self.service.call(req);
                    return Box::pin(async move {
                        let res = fut.await?;
                        Ok(res)
                    });
                } else {
                    return Box::pin(async move {
                        Ok(req.into_response(HttpResponse::Found().header(http::header::LOCATION, "/admin/login/index.html").finish().into_body()))
                    });
                }
            } else {
                // API Results should simply return a 401
                return Box::pin(async move {
                    Ok(req.into_response(HttpResponse::Unauthorized().finish().into_body()))
                });
            }
        }

        auth.as_headers(&mut req);
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
