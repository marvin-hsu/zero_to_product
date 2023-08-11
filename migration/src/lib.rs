pub use sea_orm_migration::prelude::*;

mod m20230711_142423_create_table;
mod m20230711_145754_subscribedat_should_be_not_null;
mod m20230803_125354_add_status_column_to_subscriptions_table;
mod m20230811_051343_create_user_table;
mod tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230711_142423_create_table::Migration),
            Box::new(m20230711_145754_subscribedat_should_be_not_null::Migration),
            Box::new(m20230803_125354_add_status_column_to_subscriptions_table::Migration),
            Box::new(m20230811_051343_create_user_table::Migration),
        ]
    }
}
