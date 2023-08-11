use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::EnumIter;

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Subscriptions {
    Table,
    Id,
    Email,
    Name,
    SubscribedAt,
    Status,
}

#[derive(Iden, EnumIter)]
pub enum SubscriptionStatus {
    Table,
    #[iden = "active"]
    Active,
    #[iden = "inactive"]
    Inactive,
}

#[derive(Iden)]
pub enum SubscriptionTokens {
    Table,
    SubscriptionToken,
    SubscriberId,
}

#[derive(Iden)]
pub enum Users {
    Table,
    Id,
    UserName,
    Password,
    CreatedAt,
}
