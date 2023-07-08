use crate::domain::*;

use axum::{extract::Form, http::StatusCode};
use serde::Deserialize;
use tracing::{info, instrument};
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct FormData {
    /// Subscriber Name
    pub email: String,
    /// Subscriber Email
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/subscriptions",
    tag = "subscription",
    request_body(
        content = FormData,
        content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200),
        (status = 400)
    ))]
#[instrument]
pub async fn subscribe(Form(data): Form<FormData>) -> StatusCode {
    match Subscriber::try_from(data) {
        Ok(subcriber) => StatusCode::OK,
        Err(e) => StatusCode::BAD_REQUEST,
    }
}

impl TryFrom<FormData> for Subscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Subscriber {
            email: email,
            name: name,
        })
    }
}
