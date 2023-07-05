use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(crate::handler, crate::health_check))]
pub struct ApiDoc;
