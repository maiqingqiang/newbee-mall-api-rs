use crate::bootstrap::database::{DatabasePool, PooledConn};
use crate::bootstrap::error::ApplicationError;
use crate::models::user::User;
use crate::models::user_token::UserToken;
use crate::models::LOCKED;
use actix_web::body::BoxBody;
use actix_web::dev::Payload;
use actix_web::error::ErrorInternalServerError;
use actix_web::http::StatusCode;
use actix_web::{
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, FromRequest, HttpMessage, HttpRequest, ResponseError,
};
use chrono::Local;
use futures_util::future::LocalBoxFuture;
use log::info;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

const IGNORE_ROUTES: [&str; 5] = [
    "/api/v1/index-infos",
    "/api/v1/search",
    "/api/v1/categories",
    "/api/v1/user/login",
    "/api/v1/user/register",
];

#[derive(Debug, Clone)]
pub struct Identity {
    pub user: User,
}

impl Identity {
    pub fn logout(&self, conn: &mut PooledConn) {
        UserToken::delete(conn, self.user.user_id).unwrap();
    }
}

impl FromRequest for Identity {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match req.extensions().get::<Identity>() {
            Some(identity) => Ok(identity.clone()),
            None => Err(ErrorInternalServerError("Not Found Identity")),
        })
    }
}

pub struct Authentication;

impl<S> Transform<S, ServiceRequest> for Authentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let error_response =
            |req: ServiceRequest, status: StatusCode, message: String| -> Self::Future {
                Box::pin(async move {
                    Ok(req.into_response(ApplicationError { status, message }.error_response()))
                })
            };

        let next = |req: ServiceRequest| -> Self::Future {
            let future = self.service.call(req);
            Box::pin(async move { Ok(future.await?) })
        };

        for ignore_route in IGNORE_ROUTES {
            if req.path().starts_with(ignore_route) {
                return next(req);
            }
        }

        if let Some(token) = req.headers().get("token") {
            if !token.is_empty() {
                let pool = req.app_data::<web::Data<DatabasePool>>().unwrap();

                let conn = &mut pool.get().unwrap();

                return match UserToken::find_by_token(conn, token.to_str().unwrap().to_string()) {
                    Ok(user_token) => {
                        if user_token.expire_time >= Local::now().naive_local() {
                            info!(
                                "token:{} 过期时间 {}",
                                user_token.token, user_token.expire_time
                            );
                            return error_response(
                                req,
                                StatusCode::RANGE_NOT_SATISFIABLE,
                                "无效认证！请重新登录！".into(),
                            );
                        }

                        match User::find(conn, user_token.user_id) {
                            Ok(user) => {
                                if user.locked_flag == LOCKED {
                                    return error_response(
                                        req,
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        "用户已被禁止登录！".into(),
                                    );
                                }

                                req.extensions_mut().insert(Identity { user });

                                next(req)
                            }
                            Err(_) => error_response(
                                req,
                                StatusCode::INTERNAL_SERVER_ERROR,
                                "无效用户！请重新登录！".into(),
                            ),
                        }
                    }
                    Err(_) => error_response(
                        req,
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "无效认证！请重新登录！".into(),
                    ),
                };
            }
        }

        error_response(req, StatusCode::RANGE_NOT_SATISFIABLE, "未登录！".into())
    }
}
