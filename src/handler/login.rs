use crate::{users, AppError, AppState};
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use bcrypt::verify;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use tracing::instrument;
use utoipa::ToSchema;
#[derive(Deserialize, Debug, ToSchema)]
pub struct User {
    /// UserName
    pub user_name: Option<String>,
    /// Password
    pub password: Option<String>,
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
pub async fn login(state: State<AppState>, Json(user): Json<User>) -> Result<StatusCode, AppError> {
    let user_model = users::Entity::find()
        .filter(users::Column::UserName.eq(user.user_name.unwrap()))
        .one(&state.database)
        .await?
        .ok_or(AppError::CustomWarning(
            StatusCode::NOT_FOUND,
            "Can't find User.".to_string(),
        ))?;

    let valid = verify(user.password.unwrap(), &user_model.password.unwrap()).unwrap();

    if valid {
        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::BAD_REQUEST)
    }
}
