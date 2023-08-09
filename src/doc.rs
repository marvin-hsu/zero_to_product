use crate::handler::*;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(health_check, subscribe, confirm),
    components(schemas(NewSubscriber))
)]
pub struct ApiDoc;
