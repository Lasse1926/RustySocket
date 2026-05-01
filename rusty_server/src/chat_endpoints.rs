use actix_web::{rt, web, Error, HttpRequest, HttpResponse,get,put,};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use super::{AppState,api_types};


#[put("/chat/send")]
pub async fn send_msg(
    state:web::Data<AppState>,
    body: web::Json<api_types::ChatMsg>,
    )-> HttpResponse{
    let mut chat_log = state.chat_log.lock().unwrap();

    let req = body.into_inner();
    let text = serde_json::to_string(&req).unwrap();
    
    let _ = state.new_msg_tx.send(text);

    chat_log.add_msg(req);

    HttpResponse::Ok().body("MsgSend")
}

#[get("/chat/get_msgs")]
pub async fn get_chat_msgs(state:web::Data<AppState>)-> HttpResponse{
    let body:String = serde_json::to_string(&state.chat_log).unwrap();
    HttpResponse::Ok().body(body)
}

pub async fn send_new_msgs(
    req: HttpRequest, 
    stream: web::Payload,
    state:web::Data<AppState>
    ) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut rx = state.new_msg_tx.subscribe();
    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        let mut stream = stream;

        loop {
            tokio::select! {
                Ok(msg) = rx.recv() => {
                    session.text(msg).await.unwrap();
                }

                Some(msg) = stream.next() => {
                    match msg {
                        Ok(AggregatedMessage::Text(_)) => {}
                        Ok(AggregatedMessage::Ping(p)) => {
                            session.pong(&p).await.unwrap();
                        }
                        _ => {}
                    }
                }

                else => break,
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
}
