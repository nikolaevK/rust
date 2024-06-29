mod models;
mod errors;
mod db;
use crate::db::Db;
use errors::TodoError;
use actix_cors::Cors;
use actix_web::{get, http::header, middleware::Logger, patch, post, web::{self, Data, Json, Path, ServiceConfig}, Result,};
use models::{todo::UpdateItem, CreateTodo, Item, UpdateTodo};
use shuttle_actix_web::ShuttleActixWeb;
use validator::Validate;



#[get("/")]
async fn retrieve(state: Data<Db>) -> Result<Json<Vec<Item>>, TodoError> {
    let results = state.get_all_todos().await;
    match results {
        Some(results) => Ok(Json(results)),
        None => Err(TodoError::NoTodosFound),
    }
    
}

#[post("/add")]
async fn add_todo(state: Data<Db>, body: Json<CreateTodo>) -> Result<Json<String>, TodoError> {
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let todo = state.add_item(body.todo.clone()).await;
            match todo {
                Some(todo) => Ok(Json(todo)),
                None => Err(TodoError::TodoCreationFailure),
            }
        },
        Err(_) => Err(TodoError::NoSuchTodoFound)
    }
}

#[patch("/update/{id}")]
async fn update_todo(state:Data<Db>, update_todo_url: Path<UpdateTodo>, body: Json<CreateTodo>) -> Result<Json<String>, TodoError> {
    let id = update_todo_url.into_inner().id;
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let todo = state.update_item(UpdateItem {
                id,
                todo: body.todo.clone(),
            }).await;

            match todo {
                Some(_) => Ok(Json(body.todo.clone())),
                None => Err(TodoError::TodoCreationFailure),
            }
        }
        Err(_) => Err(TodoError::TodoCreationFailure)
    }

    
    
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let database = Db::init().await.expect("error with db initialization");
    let conn = database.db.connect().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos(id INTEGER PRIMARY KEY AUTOINCREMENT, todo VARCHAR)",
        (),
    )
    .await
    .unwrap();

    let state = web::Data::new(database);
   
    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::default()
                .allowed_origin("http://localhost:5173")
                .allowed_origin("http://localhost:5173/")
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                .allowed_headers(vec![
                     header::CONTENT_TYPE,
                     header::AUTHORIZATION,
                     header::ACCEPT,
                ])
                .supports_credentials();
        cfg.service(
            web::scope("")
            .service(retrieve)
            .service(add_todo)
            .service(update_todo)
            .app_data(state)
            .wrap(cors)
            .wrap(Logger::default())
        );

    };

    Ok(config.into())
}


