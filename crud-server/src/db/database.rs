use crate::models::{todo::UpdateItem, Item};
use libsql::{Builder, Database, Error};
use dotenvy::dotenv;
use std::env;


pub struct Db {
    pub db: Database
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

        Ok(Db { db })
    }

    pub async fn get_all_todos(&self) -> Option<Vec<Item>> {
        let conn = self.db.connect().unwrap();
        let results: Result<libsql::Rows, Error> = conn
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
               Some(items)
            },
            Err(_) => None   
        }
    }

    pub async fn add_item(&self, todo: String) -> Option<String> {
        let conn = self.db.connect().unwrap();
        let result = conn
            .execute("INSERT into todos (todo) VALUES (?)", [todo.clone()])
            .await;

        match result {
            Ok(_) => Some(todo),
            Err(_) => None,
        }
    }

    pub async fn update_item(&self, todo: UpdateItem) -> Option<UpdateItem> {
        let conn = self.db.connect().unwrap();

        let result = conn
            .execute("UPDATE todos SET todo = ? WHERE id = ?", [todo.todo.clone(), todo.id.to_string()])
            .await;

        match result {
            Ok(_) => Some(todo),
            Err(_) => None,
        }

    }
}

