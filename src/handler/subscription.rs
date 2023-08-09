use crate::{domain::*, subscription_tokens, subscriptions, AppError, AppState, BadRequestError};

use crate::subscription_tokens::Model;
use axum::{
    extract::{Form, State},
    http::StatusCode,
};
use sea_orm::{ActiveModelTrait, Set, TransactionTrait};
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
pub async fn subscribe(
    state: State<AppState>,
    Form(data): Form<NewSubscriber>,
) -> Result<StatusCode, AppError> {
    let new_subscriber = Subscriber::try_from(data)?;

    let txn = state.database.begin().await?;

    let subscriber = subscriptions::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(new_subscriber.email.as_ref().to_owned()),
        name: Set(new_subscriber.name.as_ref().to_owned()),
        subscribed_at: Set(chrono::Utc::now().into()),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    let subscription_token = subscription_tokens::ActiveModel {
        subscriber_id: Set(subscriber.id),
        subscription_token: Set(Uuid::new_v4().to_string()),
    }
    .insert(&txn)
    .await?;

    let html_body = get_email_content(&state, subscription_token);
    _ = &state
        .email_client
        .send_email(&new_subscriber.email, "Welcome!", &html_body)
        .await?;

    txn.commit().await?;

    Ok(StatusCode::OK)
}

fn get_email_content(state: &State<AppState>, subscription_token: Model) -> String {
    let confirmation_link_relative = format!(
        "/subscriptions/confirm?subscription_token={}",
        subscription_token.subscription_token
    );
    let confirmation_link = &state
        .base_url
        .clone()
        .join(&confirmation_link_relative)
        .unwrap();

    let html_body = format!(
        "Welcome to our newsletter!<br />\
                    Click <a href=\"{}\">here</a> to confirm your subscription.",
        confirmation_link
    );
    html_body
}

impl TryFrom<NewSubscriber> for Subscriber {
    type Error = BadRequestError;

    fn try_from(value: NewSubscriber) -> Result<Self, BadRequestError> {
        let name = SubscriberName::parse(value.name.unwrap_or_default())?;
        let email = SubscriberEmail::parse(value.email.unwrap_or_default())?;
        Ok(Subscriber { email, name })
    }
}
