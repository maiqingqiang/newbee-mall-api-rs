use crate::bootstrap::database::{DatabasePool, PooledConn};
use crate::bootstrap::error::ApplicationError;
use crate::models::admin_user::AdminUser;
use crate::models::admin_user_token::AdminUserToken;
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
use serde::Serialize;
use std::{
    future::{ready, Ready},
    rc::Rc,
};
use tracing::info;

const MALL_IGNORE_ROUTES: [&str; 5] = [
    "/api/v1/index-infos",
    "/api/v1/search",
    "/api/v1/categories",
    "/api/v1/user/login",
    "/api/v1/user/register",
];

const ADMIN_IGNORE_ROUTES: [&str; 1] = ["/manage-api/v1/adminUser/login"];

#[derive(Debug, Clone, Serialize)]
pub struct MallIdentity {
    pub user: User,
}

impl MallIdentity {
    pub fn logout(&self, conn: &mut PooledConn) {
        UserToken::delete(conn, self.user.user_id).unwrap();
    }
}

impl FromRequest for MallIdentity {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match req.extensions().get::<MallIdentity>() {
            Some(identity) => Ok(identity.clone()),
            None => Err(ErrorInternalServerError("Not Found Identity")),
        })
    }
}

pub struct MallAuthentication;

impl<S> Transform<S, ServiceRequest> for MallAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = MallAuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(MallAuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct MallAuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for MallAuthenticationMiddleware<S>
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
            Box::pin(async move { future.await })
        };

        for ignore_route in MALL_IGNORE_ROUTES {
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
                        if user_token.expire_time <= Local::now().naive_local() {
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

                                req.extensions_mut().insert(MallIdentity { user });

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

#[derive(Debug, Clone, Serialize)]
pub struct AdminIdentity {
    pub admin_user: AdminUser,
}

impl AdminIdentity {
    pub fn logout(&self, conn: &mut PooledConn) {
        AdminUserToken::delete(conn, self.admin_user.admin_user_id).unwrap();
    }
}

impl FromRequest for AdminIdentity {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match req.extensions().get::<AdminIdentity>() {
            Some(identity) => Ok(identity.clone()),
            None => Err(ErrorInternalServerError("Not Found Identity")),
        })
    }
}

pub struct AdminAuthentication;

impl<S> Transform<S, ServiceRequest> for AdminAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AdminAuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminAuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AdminAuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AdminAuthenticationMiddleware<S>
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
            Box::pin(async move { future.await })
        };

        for ignore_route in ADMIN_IGNORE_ROUTES {
            if req.path().starts_with(ignore_route) {
                return next(req);
            }
        }

        if let Some(token) = req.headers().get("token") {
            if !token.is_empty() {
                let pool = req.app_data::<web::Data<DatabasePool>>().unwrap();

                let conn = &mut pool.get().unwrap();

                return match AdminUserToken::find_by_token(
                    conn,
                    token.to_str().unwrap().to_string(),
                ) {
                    Ok(admin_user_token) => {
                        if admin_user_token.expire_time <= Local::now().naive_local() {
                            info!(
                                "token:{} 过期时间 {}",
                                admin_user_token.token, admin_user_token.expire_time
                            );
                            return error_response(
                                req,
                                StatusCode::RANGE_NOT_SATISFIABLE,
                                "无效认证！请重新登录！".into(),
                            );
                        }

                        match AdminUser::find(conn, admin_user_token.admin_user_id) {
                            Ok(admin_user) => {
                                req.extensions_mut().insert(AdminIdentity { admin_user });

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
