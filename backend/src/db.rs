use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Config, ManagerConfig, Object, Pool, RecyclingMethod, Runtime};
use once_cell::sync::Lazy;
use std::sync::Arc;

fn create_config() -> Config {
    let mut cfg = Config::new();
    if let Ok(host) = std::env::var("PG_HOST") {
        cfg.host = Some(host);
    }
    if let Ok(dbname) = std::env::var("PG_DBNAME") {
        cfg.dbname = Some(dbname);
    }
    if let Ok(user) = std::env::var("PG_USER") {
        cfg.user = Some(user);
    }
    if let Ok(password) = std::env::var("PG_PASSWORD") {
        cfg.password = Some(password);
    }
    cfg
}

static DB_POOL: Lazy<Arc<Pool>> = Lazy::new(|| {
    let mut cfg = create_config();
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    Arc::new(pool)
});

#[allow(non_snake_case)]
pub async fn DB_CLIENT() -> Object {
    DB_POOL.get().await.expect("cannot connect to db")
}
