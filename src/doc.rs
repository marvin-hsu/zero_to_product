use crate::handler;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(handler::health_check, handler::subscribe),
    components(schemas(handler::NewSubscriber))
)]
pub struct ApiDoc;
