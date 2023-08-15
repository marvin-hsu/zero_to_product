use crate::AppError::CustomWarning;
use crate::{users, AppError, AppState};
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum_extra::extract::cookie::Cookie;
use bcrypt::verify;
use reqwest::header::SET_COOKIE;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use tracing::instrument;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct User {
    /// UserName
    pub user_name: String,
    /// Password
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "authentication",
    request_body(
        content = User,
        content_type = "application/json"),
    responses(
        (status = 200),
        (status = 400)
    ))]
#[instrument]
pub async fn login(
    state: State<AppState>,
    Json(user): Json<User>,
) -> Result<(StatusCode, HeaderMap), AppError> {
    let user_model = users::Entity::find()
        .filter(users::Column::UserName.eq(user.user_name))
        .one(&state.database)
        .await?
        .ok_or(CustomWarning(
            StatusCode::NOT_FOUND,
            "Can't find User.".to_string(),
        ))?;

    let valid = verify(user.password, &user_model.password.unwrap()).unwrap();

    if valid {
        let token = state
            .jwt_handler
            .clone()
            .create_token(&user_model.user_name);

        let cookie = Cookie::build("token", token.to_owned())
            .http_only(true)
            // .secure(true)
            .finish();

        let mut headers = HeaderMap::new();
        headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());

        Ok((StatusCode::OK, headers))
    } else {
        Err(CustomWarning(
            StatusCode::BAD_REQUEST,
            "Error password".to_string(),
        ))
    }
}
