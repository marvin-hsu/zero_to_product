use crate::handler::*;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(health_check, subscribe, confirm, login),
    components(schemas(NewSubscriber, User))
)]
pub struct ApiDoc;
