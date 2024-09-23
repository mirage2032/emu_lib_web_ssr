use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;
pub mod models;
pub mod schema;
pub mod password;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub async fn establish_connection() -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new("postgres://user:pass@localhost/emu_web");
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

#[derive(Debug, Clone)]
pub struct AppContext {
    pub pool: DbPool,
}