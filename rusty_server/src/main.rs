use actix_web::{App, HttpResponse, HttpServer, get,put, post, web};
use std::sync::Mutex;
mod api_types;

struct AppState {
    count: Mutex<i32>,
    chat_log: Mutex<api_types::ChatLog>, 
}

#[get("/")]
async fn dash_page()-> HttpResponse{
    HttpResponse::Ok().body("HI")
}

#[put("/chat/send")]
async fn send_msg(
    state:web::Data<AppState>,
    body: web::Json<api_types::ChatMsg>,
    )-> HttpResponse{
    let mut chat_log = state.chat_log.lock().unwrap();
    let req = body.into_inner();
    println!("{:?}",req);
    chat_log.add_msg(req);
    HttpResponse::Ok().body("increase count")
}

#[get("/chat/get_msgs")]
async fn get_chat_msgs(state:web::Data<AppState>)-> HttpResponse{
    let body:String = serde_json::to_string(&state.chat_log).unwrap();
    println!("request");
    HttpResponse::Ok().body(body)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        chat_log:Mutex::new(api_types::ChatLog::new()),
        count: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(dash_page)
            .service(get_chat_msgs)
            .service(send_msg)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
