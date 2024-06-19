use std::env;
use dotenvy::dotenv;
use libsql::{Builder, Connection, };
use actix_web::{get, web::{self, Json, ServiceConfig}, Result};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;


pub struct AppState {
    db: Connection
}

#[get("/")]
async fn retrieve(state: web::Data<AppState>) -> Result<Json<Vec<Item>>> {    
    let mut results = state
    .db
    .query("SELECT * FROM todo", ())
    .await
    .unwrap();

    let mut items = Vec::new();

    while let Some(row) = results.next().await.unwrap() {
        let item: Item = Item {
            id: row.get(0).unwrap(),
            content: row.get(1).unwrap(),
        };
        items.push(item);
    }

    Ok(Json(items))
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    dotenv().expect(".env file not found");
    let url = env::var("TURSO_DATABASE_URL").expect("URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").unwrap_or_default();

    let db = Builder::new_remote(url, token).build().await.expect("Something with database");
    let conn = db.connect().unwrap();

   let state = web::Data::new(AppState {
    db:conn
   });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
            .service(retrieve)
            .app_data(state)
        );

    };

    Ok(config.into())
}
#[derive(Serialize, Deserialize)]
struct Item {
    id: i64, 
    content: String,
}

