use crate::handler;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(handler::handler, handler::health_check, handler::subscription::subscribe))]
pub struct ApiDoc;
