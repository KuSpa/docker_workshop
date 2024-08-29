use std::time::{SystemTime, UNIX_EPOCH};

use actix_cors::Cors;
use actix_web::{
    delete, get, put,
    web::{self, Json},
    App, HttpResponse, HttpServer,
};
use serde_derive::{Deserialize, Serialize};
use todo_item::TodoItem;
use todo_item::TODOS;

mod todo_item;

#[derive(Serialize)]
struct AllTodos {
    todos: Vec<TodoItem>,
}

#[get("/todos")]
async fn get_todo() -> HttpResponse {
    log::debug!("Get all todos");
    let todos: Vec<TodoItem> = TODOS.read().unwrap().iter().map(TodoItem::from).collect();
    let all_todos = AllTodos { todos };
    let return_string = serde_json::to_string(&all_todos).unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(return_string)
}

#[derive(Deserialize)]
struct CreateTodo {
    content: String,
}

#[put("/todos")]
async fn insert_todo(request: Json<CreateTodo>) -> HttpResponse {
    log::debug!("Insert: {}", &request.content);
    let very_serious_id: u128 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    TODOS
        .write()
        .unwrap()
        .insert(very_serious_id as i32, request.content.clone());
    HttpResponse::Ok().into()
}

#[delete("/todos/{id}")]
async fn delete_todo(id: web::Path<i32>) -> HttpResponse {
    log::debug!("Delete: {}", *id);
    let mut map = TODOS.write().unwrap();
    map.remove(&*id);
    HttpResponse::Ok().into()
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:8000".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let log_level =  std::env::var("RUST_LOG").unwrap_or("info".to_owned());
    if log_level != "debug" {
        log::info!(
            "Log level is {}. To see more verbose logging, set environment variable to 'debug'.",
            log_level
        );
    }
    
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
