use sea_orm::DatabaseConnection;

#[derive(Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
}
