use std::env;
use dotenvy::dotenv;
use libsql::{params, Builder, Database, Value};
use actix_cors::Cors;
use actix_web::{get,post, http::header, middleware::Logger, web::{self, Json, ServiceConfig}, Result};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;


#[derive(Serialize, Deserialize)]
struct Item {
    id: i64, 
    todo: String,
}

#[get("/")]
async fn retrieve() -> Result<Json<Vec<Item>>> {
    let db = connection().await;
    let conn = db.connect().unwrap();
    let results = conn
    .query("SELECT * FROM todos", ())
    .await;

    let mut items = Vec::new();
    match results {
        Ok(mut rows) => {
            while let Some(row) = rows.next().await.unwrap() {
                let item: Item = Item {
                    id: row.get(0).unwrap(),
                    todo: row.get(1).unwrap(),
                };
                items.push(item);
            }
        },
        Err(_) => {
            println!("Error retrieving");
            ()
        }
    }   
    Ok(Json(items))
}

#[post("/add")]
async fn add_todo(body: Json<Item>) -> Result<Json<bool>> {
    let db = connection().await;
    let conn = db.connect().unwrap();

    let todo = conn
        .execute("INSERT into todos (todo) VALUES (?1)", [body.todo.clone()])
        .await;

    // TODO: create separate modules and crates
    // write good response
    match todo {
        Ok(_) => Ok(Json(true)),
        Err(_) => Ok(Json(false)),
    }
    
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {

    let db = connection().await;
    let conn = db.connect().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos(id INTEGER PRIMARY KEY AUTOINCREMENT, todo VARCHAR)",
        (),
    )
    .await
    .unwrap();
   
    let config = move |cfg: &mut ServiceConfig| {
        let cors = Cors::default()
                 .allowed_origin("http://localhost:5173")
                 .allowed_origin("http://localhost:5173/")
                 .allowed_methods(vec!["GET", "POST"])
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


