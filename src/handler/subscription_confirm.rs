use crate::prelude::Subscriptions;
use crate::sea_orm_active_enums::SubscriptionStatus;
use crate::AppError::{CustomInternalError, CustomWarning};
use crate::{subscription_tokens, AppError, AppState};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use tracing::instrument;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/subscriptions/confirm/{token}",
    tag = "subscription",
    responses(
        (status = 200),
        (status = 400)
    ),
    params(
        ("token" = uuid, Path, description = "Subscription Token")
    ))]
#[instrument]
pub async fn confirm(
    state: State<AppState>,
    Path(token): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let mut subscription = subscription_tokens::Entity::find()
        .find_also_related(Subscriptions)
        .filter(subscription_tokens::Column::SubscriptionToken.eq(token.to_string()))
        .one(&state.database)
        .await?
        .ok_or(CustomWarning(
            StatusCode::NOT_FOUND,
            "Can't find Subscription.".to_string(),
        ))?
        .1
        .ok_or(CustomInternalError(
            "incomplete subscription record.".to_string(),
        ))?
        .into_active_model();

    subscription.status = Set(SubscriptionStatus::Active);

    subscription.update(&state.database).await?;

    Ok(StatusCode::OK)
}
