use crate::domain::*;

use axum::{extract::RawForm, http::StatusCode};
use serde::Deserialize;
use tracing::instrument;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct FormData {
    pub email: Option<String>,
    pub name: Option<String>,
}

#[utoipa::path(
    post,
     path = "/subscribe",
      tag = "subscription",
      request_body(
        content = FormData,
        content_type = "application/x-www-form-urlencoded"))]
#[instrument]
pub async fn subscribe(RawForm(form): RawForm) -> StatusCode {
    let form_data: FormData = serde_urlencoded::from_bytes(&form).unwrap();

    if form_data.email.is_some() && form_data.name.is_some() {
        let subscriber = Subscriber::try_from(form_data).unwrap();
        StatusCode::OK
    } else {
        StatusCode::BAD_REQUEST
    }
}

impl TryFrom<FormData> for Subscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name.unwrap())?;
        let email = SubscriberEmail::parse(value.email.unwrap())?;
        Ok(Subscriber {
            email: email,
            name: name,
        })
    }
}
