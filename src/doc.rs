use crate::handler::*;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(health_check, subscribe, confirm, login, publish_newsletter),
    components(schemas(NewSubscriber, User, NewsletterContent, Content))
)]
pub struct ApiDoc;
