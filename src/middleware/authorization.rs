use axum::body::Bytes;
use axum::http::{header, Request};
use http_body::combinators::UnsyncBoxBody;
use tower_http::validate_request::ValidateRequest;
use tracing::{info, warn};
use crate::JwtHandler;

#[derive(Clone)]
pub struct Authorization {
    pub jwt_handler: JwtHandler,
}

impl<B>  ValidateRequest<B> for Authorization {
    type ResponseBody = UnsyncBoxBody<Bytes, axum::Error>;

    fn validate(&mut self, request: &mut Request<B>) -> Result<(), axum::http::Response<Self::ResponseBody>> {
        request
            .headers().get(header::COOKIE)
            .and_then(|cookie| cookie.to_str().ok())
            .and_then(|cookie| cookie.split(';').find(|cookie| cookie.contains("token")))
            .and_then(|cookie| cookie.split('=').nth(1))
            .map(|token| self.jwt_handler.clone().decode_token(token.to_string()))
            .map(|result| match result {
                Ok(_) => {
                    info!("Token is valid");
                    Ok(())
                },
                Err(_) => {
                    warn!("Failed to decode token: {:?}", result);
                    Ok(())
                },
            })
            .unwrap_or_else(|| {
                warn!("Missing authorization header");
                Ok(())
            })
    }
}
