mod models;
mod errors;
mod db;
use crate::db::Db;

use std::env;
use dotenvy::dotenv;
use errors::TodoError;
use libsql::{Builder, Database};
use actix_cors::Cors;
use actix_web::{get, http::header, middleware::Logger, patch, post, web::{self, Data, Json, Path, ServiceConfig}, Responder, Result,};
use models::{CreateTodo, UpdateTodo, Item};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use validator::Validate;




#[get("/")]
async fn retrieve(state: Data<Db>) -> Result<Json<Vec<Item>>, TodoError> {
    // let conn = state.connection.connect().unwrap();
    // let results = conn
    // .query("SELECT * FROM todos", ())
    // .await;

    // let mut items = Vec::new();
    // match results {
    //     Ok(mut rows) => {
    //         while let Some(row) = rows.next().await.unwrap() {
    //             let item: Item = Item {
    //                 id: row.get(0).unwrap(),
    //                 todo: row.get(1).unwrap(),
    //             };
    //             items.push(item);
    //         }
    //     },
    //     Err(_) => {
    //         println!("Error retrieving");
    //         ()
    //     }
    // }   

    let results = state.get_all_todos().await;

    match results {
        Some(results) => Ok(Json(results)),
        //TODO: Frontend Needs empty array not error
        None => Err(TodoError::NoTodosFound)
    }
    
}

#[post("/add")]
async fn add_todo(body: Json<CreateTodo>) -> Result<Json<String>, TodoError> {
    let db = connection().await;
    let conn = db.connect().unwrap();

    let is_valid = body.validate();

    match is_valid {
        Ok(_) => {
            let todo = conn
                .execute("INSERT into todos (todo) VALUES (?)", [body.todo.clone()])
                .await;

            match todo {
                Ok(_) => Ok(Json(body.todo.clone())),
                Err(_) => Err(TodoError::TodoCreationFailure),
            }
        },
        Err(_) => Err(TodoError::TodoCreationFailure)
    }
    
}

#[patch("/update/{id}")]
async fn update_todo(update_todo_url: Path<UpdateTodo>, body: Json<CreateTodo>) -> impl Responder {
    let db = connection().await;
    let conn = db.connect().unwrap();

    let id = update_todo_url.into_inner().id;
    let is_valid = body.validate();

    match is_valid {
        Ok(_) => println!("Valid todo"),
        Err(_) => panic!("Invalid todo name"),
    }

    let todo = conn
                .execute("UPDATE todos SET todo = ? WHERE id = ?", [body.todo.clone(), id.clone()])
                .await;
    match todo {
        Ok(_) => Ok(Json("Todo {id} updated successfully")),
        Err(_) => Err(TodoError::NoSuchTodoFound)
    }
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    let db = Db::init().await.expect("error with db initialization");
    // let db = connection().await;
    let conn = db.connection.connect().unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos(id INTEGER PRIMARY KEY AUTOINCREMENT, todo VARCHAR)",
        (),
    )
    .await
    .unwrap();

    let db_connection = web::Data::new(db);
   
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
            .app_data(db_connection)
            .wrap(cors)
            .wrap(Logger::default())
        );

    };

    Ok(config.into())
}

async fn connection() -> Database {
    dotenv().expect(".env file not found");

    let url = env::var("TURSO_DATABASE_URL").expect("URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").unwrap_or_default();

    let db = Builder::new_remote(url, token)
        .build()
        .await
        .unwrap();

    db
}


