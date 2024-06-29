use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Deserialize, Serialize)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Todo name required"))]
    pub todo: String,
}
#[derive(Validate, Deserialize, Serialize)]
pub struct UpdateTodo {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
   pub id: u64, 
   pub todo: String,
}

pub struct UpdateItem {
   pub id: String, 
   pub todo: String,
}