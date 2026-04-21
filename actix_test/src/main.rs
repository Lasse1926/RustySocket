use actix_web::{App, HttpResponse, HttpServer, get,put, post, web};
use std::{fmt::format, sync::Mutex};

struct AppState {
    count: Mutex<i32>,
}

#[get("/")]
async fn dash_page()-> HttpResponse{
    HttpResponse::Ok().body("HI")
}

#[put("/counter/increase")]
async fn increase(state:web::Data<AppState>)-> HttpResponse{
    let mut count = state.count.lock().unwrap();
    *count += 1;
    HttpResponse::Ok().body("increase count")
}
#[put("/counter/decrease")]
async fn decrease(state:web::Data<AppState>)-> HttpResponse{
    let mut count = state.count.lock().unwrap();
    *count -= 1;
    HttpResponse::Ok().body("decrease count")
}
#[get("/counter")]
async fn get_counter(state:web::Data<AppState>)-> HttpResponse{
    let count = state.count.lock().unwrap();
    HttpResponse::Ok().body(format!("counter: {}",count))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        count: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(dash_page)
            .service(increase)
            .service(decrease)
            .service(get_counter)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
