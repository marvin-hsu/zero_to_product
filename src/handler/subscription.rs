use crate::{domain::*, subscription_tokens, subscriptions, AppState};

use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, Set};
use serde::Deserialize;
use tracing::instrument;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Debug, ToSchema)]
pub struct NewSubscriber {
    /// Subscriber Email
    pub email: Option<String>,
    /// Subscriber Name
    pub name: Option<String>,
}

#[utoipa::path(
    post,
    path = "/subscriptions",
    tag = "subscription",
    request_body(
        content = NewSubscriber,
        content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200),
        (status = 400)
    ))]
#[instrument]
pub async fn subscribe(state: State<AppState>, Form(data): Form<NewSubscriber>) -> StatusCode {
    if let Ok(subscriber) = Subscriber::try_from(data) {
        let result = subscriptions::ActiveModel {
            id: Set(Uuid::new_v4()),
            email: Set(subscriber.email.as_ref().to_owned()),
            name: Set(subscriber.name.as_ref().to_owned()),
            subscribed_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        }
        .insert(&state.database)
        .await;

        if let Ok(r) = result {
            let subscription_token_result = subscription_tokens::ActiveModel {
                subscriber_id: Set(r.id),
                subscription_token: Set(Uuid::new_v4().to_string()),
                ..Default::default()
            }
            .insert(&state.database)
            .await;

            if let Ok(subscription_token) = subscription_token_result {
                let confirmation_link = format!(
                    "{}/subscriptions/confirm?subscription_token={}",
                    &state.base_url, subscription_token.subscription_token
                );

                let html_body = format!(
                    "Welcome to our newsletter!<br />\
                    Click <a href=\"{}\">here</a> to confirm your subscription.",
                    confirmation_link
                );
                let send_result = &state
                    .email_client
                    .send_email(&subscriber.email, "Welcome!", "text/html", &html_body)
                    .await;

                if send_result.is_ok() {
                    return StatusCode::OK;
                }

                return StatusCode::INTERNAL_SERVER_ERROR;
            }
        };

        StatusCode::BAD_REQUEST
    } else {
        StatusCode::BAD_REQUEST
    }
}

impl TryFrom<NewSubscriber> for Subscriber {
    type Error = String;

    fn try_from(value: NewSubscriber) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name.unwrap_or_default())?;
        let email = SubscriberEmail::parse(value.email.unwrap_or_default())?;
        Ok(Subscriber { email, name })
    }
}
