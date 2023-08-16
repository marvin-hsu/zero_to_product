use crate::sea_orm_active_enums::SubscriptionStatus;
use crate::{subscriptions, AppError, AppState, SubscriberEmail};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use itertools::{Either, Itertools};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct NewsletterContent {
    /// Newsletter Title
    title: String,
    /// Newsletter Content
    content: Content,
}

#[derive(Deserialize, ToSchema)]
pub struct Content {
    text: String,
}

#[utoipa::path(
post,
path = "/newsletters",
tag = "newsletter",
request_body(
content = NewsletterContent,
content_type = "application/json"),
responses(
(status = 200),
(status = 400)
))]
pub async fn publish_newsletter(
    state: State<AppState>,
    Json(body): Json<NewsletterContent>,
) -> Result<StatusCode, AppError> {
    let (subscribers, error): (Vec<_>, Vec<_>) = subscriptions::Entity::find()
        .filter(subscriptions::Column::Status.eq(SubscriptionStatus::Active))
        .all(&state.database)
        .await?
        .iter()
        .map(|s| SubscriberEmail::parse(s.email.to_owned()))
        .partition_map(|r| match r {
            Ok(email) => Either::Left(email),
            Err(e) => Either::Right(e),
        });

    let mut send_fail = 0;
    for subscriber in subscribers {
        let result = state
            .email_client
            .send_email(&subscriber, &body.title, &body.content.text)
            .await;

        if result.is_err() {
            send_fail += 1;
        }
    }

    if send_fail > 0 || !error.is_empty() {
        Err(AppError::CustomInternalError(
            "Failed to send newsletter".to_string(),
        ))
    } else {
        Ok(StatusCode::OK)
    }
}
