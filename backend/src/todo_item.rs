use tokio_postgres::{Error, GenericClient, Row};

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
    pub async fn all<C: GenericClient>(client: &C) -> Result<Vec<TodoItem>, Error> {
        let rows = client.query("SELECT id, content FROM todos", &[]).await?;

        Ok(rows.into_iter().map(TodoItem::from).collect())
    }

    pub async fn insert<C: GenericClient>(client: &C, content: &String) -> Result<u64, Error> {
        client
            .execute(
                "INSERT INTO todos
                (id, content)
                    VALUES
                (nextval('todo_sequence'), $1)",
                &[content],
            )
            .await
    }

    pub async fn delete<C: GenericClient>(client: &C, id: i32) -> Result<u64, Error> {
        client
            .execute(
                // TODO maybe have a syntac error here?
                "DELETE FROM todos
               WHERE
                id = $1",
                &[&id],
            )
            .await
    }
}
