use crate::base::check_token;
use actix_web::body::BoxBody;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};

pub struct Auth;

impl<S> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware { service }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let ignored_routes = vec!["/login/", "/users/"];
        for route in ignored_routes.iter() {
            if &req.path() == route {
                let fut = self.service.call(req);
                return Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                });
            }
        }

        let reject = |req: ServiceRequest| {
            Box::pin(async move {
                let mut response_builder = HttpResponse::Unauthorized();
                response_builder.insert_header((header::CONTENT_TYPE, "text/plain"));
                let res = response_builder.body("Unauthorized");
                let response: Self::Response = req.into_response(res);
                Ok(response)
            })
        };

        match req.headers().get("Authorization") {
            None => return reject(req),
            Some(header) => {
                let token = header.clone().to_str().unwrap().to_string();
                match check_token(token.replace("Bearer ", "")) {
                    Err(_) => return reject(req),
                    _ => {
                        let fut = self.service.call(req);
                        return Box::pin(async move {
                            let res = fut.await?;
                            Ok(res)
                        });
                    }
                }
            }
        };
    }
}
