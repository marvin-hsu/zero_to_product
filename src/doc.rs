use crate::handler;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(handler::health_check, handler::subscribe),
    components(schemas(handler::FormData))
)]
pub struct ApiDoc;
