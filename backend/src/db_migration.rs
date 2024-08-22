use tokio_postgres_migration::Migration;

use crate::db::DB_CLIENT;

const SCRIPTS_UP: [(&str, &str); 2] = [
    (
        "0001_create-todos-sequcence",
        include_str!("../migrations/0001_create-todos-sequence_up.sql"),
    ),
    (
        "0002_create-todos-table",
        include_str!("../migrations/0002_create-todos-table_up.sql"),
    ),
];

pub async fn migrate_up() {
    let mut client = DB_CLIENT().await;
    let migration = Migration::new("migrations".to_string());
    migration
        .up(&mut **client, &SCRIPTS_UP)
        .await
        .expect("couldn't run migrations");
}
