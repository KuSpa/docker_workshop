use actix_cors::Cors;
use actix_web::{
    delete, get, put,
    web::{self, Json},
    App, HttpResponse, HttpServer,
};
use deadpool_postgres::Pool;
use serde_derive::{Deserialize, Serialize};
use todo_item::TodoItem;

mod postgres;
mod todo_item;

#[derive(Serialize)]
struct AllTodos {
    todos: Vec<TodoItem>,
}

#[get("/todos")]
async fn get_todo(pool: web::Data<Pool>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    let client = client;
    match TodoItem::all(&**client).await {
        Ok(todos) => {
            let all_todos = AllTodos { todos };
            let return_string = serde_json::to_string(&all_todos).unwrap();
            log::debug!("{}", &return_string);
            HttpResponse::Ok()
                .content_type("application/json")
                .body(return_string)
        }
        Err(err) => {
            log::debug!("unable to fetch users: {:?}", err);
            HttpResponse::InternalServerError().json("unable to fetch users")
        }
    }
}

#[derive(Deserialize)]
struct CreateTodo {
    content: String,
}

// TODO read json
#[put("/todos")]
async fn insert_todo(pool: web::Data<Pool>, request: Json<CreateTodo>) -> HttpResponse {
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    log::debug!("{}", &request.content);
    let client = client;
    match TodoItem::insert(&**client, &request.content).await {
        Ok(_) => HttpResponse::Ok().into(),
        Err(err) => {
            log::debug!("unable to insert item: {:?}", err);
            // ERROR don't have db active and see this error
            HttpResponse::InternalServerError().json("unable to insert item")
        }
    }
}

#[delete("/todos/{id}")]
async fn delete_todo(pool: web::Data<Pool>, id: web::Path<i32>) -> HttpResponse {
    log::warn!("{}", *id);
    let client = match pool.get().await {
        Ok(client) => client,
        Err(err) => {
            log::debug!("unable to get postgres client: {:?}", err);
            return HttpResponse::InternalServerError().json("unable to get postgres client");
        }
    };
    let client = client;
    match TodoItem::delete(&**client, *id).await {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(err) => {
            log::debug!("unable to delete item: {:?}", err);
            HttpResponse::InternalServerError().json("unable to delete item")
        }
    }
}

fn address() -> String {
    std::env::var("ADDRESS").unwrap_or_else(|_| "127.0.0.1:8000".into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let pg_pool = postgres::create_pool();
    postgres::migrate_up(&pg_pool).await;

    let address = address();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pg_pool.clone()))
            .service(get_todo)
            .service(insert_todo)
            .service(delete_todo)
    })
    .bind(&address)?
    .run()
    .await
}
