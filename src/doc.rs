use crate::handler;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(handler::handler, handler::health_check))]
pub struct ApiDoc;
