use libsql::{Builder, Database, Error};
use crate::models::Item;
use std::env;
use dotenvy::dotenv;


pub struct Db {
    pub connection: Database
}

impl Db {
    pub async fn init() -> Result<Self, Error> {
        dotenv().expect(".env file not found");

        let url = env::var("TURSO_DATABASE_URL").expect("URL must be set");
        let token = env::var("TURSO_AUTH_TOKEN").unwrap_or_default();

        let db = Builder::new_remote(url, token)
        .build()  
        .await
        .unwrap();

        Ok(Db {connection:db})
    }

    pub async fn get_all_todos(&self) -> Option<Vec<Item>> {
        let conn = self.connection.connect().unwrap();
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
        Some(items)
    }
}

