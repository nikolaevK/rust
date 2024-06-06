use std::{collections::HashMap, sync::Arc};
use axum::{extract::{ws::{Message, WebSocket}, WebSocketUpgrade}, response::IntoResponse, routing::get,  Extension, Router};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{ mpsc::{self, UnboundedReceiver, UnboundedSender}, RwLock};
use tower_http::services::ServeDir;

static NEXT_USER_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
type Users = Arc<RwLock<HashMap<usize, UnboundedSender<Message>>>>;



#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = router();
    Ok(router.into())
}

fn router() -> Router {
    let users = Users::default();
    let router = Router::new()
        .route("/websocket", get(handle_websocket_upgrade))
        .layer(Extension(users))
        .fallback_service(ServeDir::new("assets"));
    router
}

async fn handle_websocket_upgrade(
    ws: WebSocketUpgrade,
    Extension(users): Extension<Users>
) -> impl IntoResponse {
    ws.on_upgrade(|ws | handle_socket(ws, users))
}

// convert js object and add to it
#[derive(Serialize, Deserialize)]
struct ChatMessage {
    message: String,
    username: String,
    uid: Option<usize>,
}

async fn handle_socket(ws: WebSocket, users: Users) {
    // Get a user id
    let user_id = NEXT_USER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let (tx, mut rx): (UnboundedSender<Message>, UnboundedReceiver<Message>) = 
        mpsc::unbounded_channel();
    
    users.write().await.insert(user_id, tx);


    let (mut sender, mut receiver) = ws.split();

    // Listen for messages
    tokio::task::spawn(async move {
        while let Some(msg )= rx.recv().await {
            if sender.send(msg).await.is_err() {
                // Disconnect the user

            }
        }
    });

    // Broadcast Messages
    while let Some(Ok(result)) = receiver.next().await {
        if let Ok(result) = enrich_result(result, user_id) {
            broadcast_message(result, &users).await;
        }
    }

    disconnect(user_id, &users).await;
   
}

fn enrich_result(message: Message, user_id: usize) -> Result<Message, serde_json::Error> {
    match message {
        Message::Text(message) => {
            let mut chat_message: ChatMessage = serde_json::from_str(&message)?;
            chat_message.uid = Some(user_id);
            let message = serde_json::to_string(&chat_message)?;
            Ok(Message::Text(message))
        },
        _ => Ok(message),
    }

}

async fn disconnect(user_id:usize, users: &Users) {
    users.write().await.remove(&user_id);
}

async fn broadcast_message(msg: Message, users: &Users) {
    if let Message::Text(msg) = msg {
        for (_user_id, tx) in users.read().await.iter() {
            let _ = tx.send(Message::Text(msg.clone()));
        }
    }
}
