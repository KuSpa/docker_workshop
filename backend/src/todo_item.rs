use tokio_postgres::{Error, Row};

use crate::db::DB_CLIENT;

#[derive(Debug, serde_derive::Serialize)]
pub struct TodoItem {
    pub id: i32,
    pub content: String,
}

impl From<Row> for TodoItem {
    fn from(row: Row) -> Self {
        Self {
            id: row.get(0),
            content: row.get(1),
        }
    }
}

impl TodoItem {
    pub async fn all() -> Result<Vec<TodoItem>, Error> {
        let rows = DB_CLIENT()
            .await
            .query("SELECT id, content FROM todos", &[])
            .await?;

        Ok(rows.into_iter().map(TodoItem::from).collect())
    }

    pub async fn insert(content: &String) -> Result<u64, Error> {
        DB_CLIENT()
            .await
            .execute(
                "INSERT INTO todos
                (id, content)
                    VALUES
                (nextval('todo_sequence'), $1)",
                &[content],
            )
            .await
    }

    pub async fn delete(id: i32) -> Result<u64, Error> {
        DB_CLIENT()
            .await
            .execute(
                // TODO maybe have a syntac error
                "DELETE FROM todos
               WHERE
                id = $1",
                &[&id],
            )
            .await
    }
}
