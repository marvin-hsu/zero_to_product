use crate::tables::{SubscriptionStatus, SubscriptionTokens, Subscriptions};
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Iterable;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(SubscriptionStatus::Table)
                    .values(SubscriptionStatus::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Subscriptions::Table)
                    .add_column(
                        ColumnDef::new(Subscriptions::Status)
                            .enumeration(
                                SubscriptionStatus::Table,
                                SubscriptionStatus::iter().skip(1),
                            )
                            .not_null()
                            .default("inactive"),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SubscriptionTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SubscriptionTokens::SubscriptionToken)
                            .text()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SubscriptionTokens::SubscriberId)
                            .uuid()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .from(SubscriptionTokens::Table, SubscriptionTokens::SubscriberId)
                    .to(Subscriptions::Table, Subscriptions::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SubscriptionTokens::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Subscriptions::Table)
                    .drop_column(Subscriptions::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(SubscriptionStatus::Table).to_owned())
            .await?;

        Ok(())
    }
}
