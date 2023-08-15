use crate::JwtHandler;
use axum::body::BoxBody;
use axum::http::{header, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use jsonwebtoken::errors::ErrorKind;
use tower_http::validate_request::ValidateRequest;
use tracing::warn;

#[derive(Clone)]
pub struct Authorization {
    pub jwt_handler: JwtHandler,
}

impl<B> ValidateRequest<B> for Authorization {
    type ResponseBody = BoxBody;

    fn validate(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        request
            .headers()
            .get(header::COOKIE)
            .and_then(|cookie| cookie.to_str().ok())
            .and_then(|cookie| cookie.split(';').find(|cookie| cookie.contains("token")))
            .and_then(|cookie| cookie.split('=').nth(1))
            .map(|token| self.jwt_handler.clone().decode_token(token.to_string()))
            .map(|result| match result {
                Ok(_) => Ok(()),
                Err(e) => {
                    if e.kind().eq(&ErrorKind::ExpiredSignature) {
                        Err((
                            StatusCode::UNAUTHORIZED,
                            "Token expired, please login again",
                        )
                            .into_response())
                    } else {
                        Err(StatusCode::UNAUTHORIZED.into_response())
                    }
                }
            })
            .unwrap_or_else(|| {
                warn!("Missing authorization header");
                Err(StatusCode::UNAUTHORIZED.into_response())
            })
    }
}
