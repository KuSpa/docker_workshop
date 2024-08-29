use actix_cors::Cors;
use actix_web::{
    delete, get, put,
    web::{self, Json},
    App, HttpResponse, HttpServer,
};
use serde_derive::{Deserialize, Serialize};
use todo_item::TodoItem;

mod db;
mod db_migration;
mod todo_item;

#[derive(Serialize)]
struct AllTodos {
    todos: Vec<TodoItem>,
}

#[get("/todos")]
async fn get_todo() -> HttpResponse {
    log::debug!("Get all todos");
    match TodoItem::all().await {
        Ok(todos) => {
            let all_todos = AllTodos { todos };
            let return_string = serde_json::to_string(&all_todos).unwrap();
            HttpResponse::Ok()
                .content_type("application/json")
                .body(return_string)
        }
        Err(err) => {
            log::warn!("unable to fetch users: {:?}", err);
            HttpResponse::InternalServerError().json("unable to fetch users")
        }
    }
}

#[derive(Deserialize)]
struct CreateTodo {
    content: String,
}

#[put("/todos")]
async fn insert_todo(request: Json<CreateTodo>) -> HttpResponse {
    log::debug!("Insert: {}", &request.content);
    match TodoItem::insert(&request.content).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(err) => {
            log::warn!("unable to insert item: {:?}", err);
            // TODO don't have db active and see this error
            HttpResponse::InternalServerError().json("unable to insert item")
        }
    }
}

#[delete("/todos/{id}")]
async fn delete_todo(id: web::Path<i32>) -> HttpResponse {
    log::debug!("Delete: {}", *id);
    match TodoItem::delete(*id).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::warn!("unable to delete item: {:?}", err);
            HttpResponse::InternalServerError().json("unable to delete item")
        }
    }
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    db_migration::migrate_up().await;

    let address = address();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .service(get_todo)
            .service(insert_todo)
            .service(delete_todo)
    })
    .bind(&address)?
    .run()
    .await
}
