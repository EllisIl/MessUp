use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use actix_files as fs;

#[derive(Serialize, Deserialize, Clone)]
struct Message {
    sender: String,
    content: String,
}

#[derive(Clone)]
struct AppState {
    messages: Arc<Mutex<Vec<Message>>>,
}

async fn send_message(
    data: web::Data<AppState>,
    msg: web::Json<Message>,
) -> impl Responder {
    let mut messages = data.messages.lock().unwrap();
    messages.push(msg.into_inner());
    HttpResponse::Created().finish()
}

async fn get_messages(data: web::Data<AppState>) -> impl Responder {
    let messages = data.messages.lock().unwrap();
    HttpResponse::Ok().json(&*messages)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = AppState {
        messages: Arc::new(Mutex::new(vec![])),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/send", web::post().to(send_message))
            .route("/messages", web::get().to(get_messages))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
