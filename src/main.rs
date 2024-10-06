use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use warp::ws::{Message, WebSocket};
use warp::Filter;

// Define a structure for the chat message that will be serialized/deserialized
#[derive(Serialize, Deserialize)]
struct ChatMessage {
    user: String,     // The username of the person sending the message
    message: String,  // The content of the chat message
}

// Type alias for a thread-safe, shared set of users
type Users = Arc<Mutex<HashSet<String>>>;

#[tokio::main]
async fn main() {
    // Create a broadcast channel to send messages to all clients
    let (tx, _rx) = broadcast::channel(100);

    // Shared state for keeping track of connected users
    let users: Users = Arc::new(Mutex::new(HashSet::new()));

    // Define a WebSocket route for "/chat" that handles upgrades and passes broadcast and users
    let chat = warp::path("chat")
        .and(warp::ws())                     // WebSocket upgrade filter
        .and(with_broadcast(tx.clone()))      // Inject broadcast sender
        .and(with_users(users.clone()))       // Inject shared users state
        .map(|ws: warp::ws::Ws, tx, users| {
            // On WebSocket connection upgrade, handle the connection
            ws.on_upgrade(move |socket| handle_connection(socket, tx, users))
        });

    // Serve static files from the "static" directory (e.g., index.html)
    let static_files = warp::fs::dir("static");

    // Combine WebSocket and static file routes into one set of routes
    let routes = warp::get().and(
        chat.or(static_files),  // Either WebSocket or static file routes
    );

    // Start the Warp server on localhost at port 3030
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Function to inject the broadcast sender into the WebSocket handler
fn with_broadcast(
    tx: broadcast::Sender<String>,
) -> impl Filter<Extract = (broadcast::Sender<String>,), Error = std::convert::Infallible> + Clone {
    // Clone the broadcast sender for each WebSocket connection
    warp::any().map(move || tx.clone())
}

// Function to inject the shared users state into the WebSocket handler
fn with_users(
    users: Users,
) -> impl Filter<Extract = (Users,), Error = std::convert::Infallible> + Clone {
    // Clone the users list for each WebSocket connection
    warp::any().map(move || users.clone())
}

// Handle the WebSocket connection for a client
async fn handle_connection(ws: WebSocket, tx: broadcast::Sender<String>, _users: Users) {
    // Split the WebSocket into a sender and receiver
    let (mut ws_tx, mut ws_rx) = ws.split();
    let mut rx = tx.subscribe();  // Subscribe to the broadcast channel for receiving messages

    // Spawn a task to send broadcasted messages to the WebSocket client
    let tx_clone = tx.clone();
    let send_task = tokio::spawn(async move {
        // Receive messages from the broadcast channel and send them over WebSocket
        while let Ok(msg) = rx.recv().await {
            ws_tx.send(Message::text(msg)).await.ok();  // Send received message to the client
        }
    });

    // Receive messages from the WebSocket client
    while let Some(Ok(message)) = ws_rx.next().await {
        if let Ok(text) = message.to_str() {
            // Parse the incoming message into a ChatMessage struct
            let chat_message: ChatMessage = serde_json::from_str(text).unwrap();
            // Format the message to broadcast it to other users
            let broadcast_message = format!("{}: {}", chat_message.user, chat_message.message);
            // Send the message to all clients via the broadcast channel
            tx_clone.send(broadcast_message).unwrap();
        }
    }

    // Wait for the task that sends broadcasted messages to complete
    send_task.await.unwrap();
}
