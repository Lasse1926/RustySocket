use actix_web::{App, HttpServer, web};
use tokio::sync::broadcast::{self, Receiver, Sender};
use std::sync::Mutex;
mod api_types;
mod chat_endpoints;

struct AppState {
    chat_log: Mutex<api_types::ChatLog>, 
    new_msg_tx: Sender<String>, 
    new_msg_rx: Receiver<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (tx, rx) = broadcast::channel::<String>(100);
    let app_state = web::Data::new(AppState {
        chat_log:Mutex::new(api_types::ChatLog::new()),
        new_msg_tx: tx, 
        new_msg_rx: rx,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(chat_endpoints::get_chat_msgs)
            .service(chat_endpoints::send_msg)
            .route("/send_new_msgs", web::get().to(chat_endpoints::send_new_msgs))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
