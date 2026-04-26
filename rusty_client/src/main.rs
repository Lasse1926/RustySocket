use std::collections::HashMap;
use std::sync::{Arc,Mutex};

use reqwest::{Error, Response};
mod api_types;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Hash, Eq, PartialEq, Clone, Copy,Debug)]
enum RequestId {
    Increase,
    Decrease,
    RequestData,
}

#[derive(Debug)]
enum AppMessage {
    ChatLog(api_types::ChatLog),
    Msg(api_types::ChatMsg),
    Error(String),
    CallResponse(Response),
}

struct RequestState {
    loading: bool,
    result: Arc<Mutex<Option<AppMessage>>>,
}

impl RequestState {
    fn new() -> Self {
        Self {
            loading: false,
            result: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(Clone)]
struct ApiClient{
    client:reqwest::Client,
}

impl ApiClient {
    async fn request_chat_log(&self) -> Result<AppMessage, reqwest::Error> {
        let res = self.client.get("http://localhost:8080/chat/get_msgs").send().await?;
        let text = res.text().await?;
        Ok(AppMessage::ChatLog(serde_json::from_str(&text).unwrap_or(api_types::ChatLog::new())))
    }

    async fn send_msg(&self,msg:String,sender:String) -> Result<AppMessage, reqwest::Error> {
        let msg = api_types::ChatMsg::new(msg,sender);
        let res = self.client.put("http://localhost:8080/chat/send").json(&msg).send().await?;
        Ok(AppMessage::CallResponse(res))
    }
}

struct MyEguiApp {
    api_client: ApiClient,
    runtime: tokio::runtime::Runtime,
    requests: HashMap<RequestId, RequestState>,

}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self { 
            api_client: ApiClient { client: reqwest::Client::new() }, 
            runtime: tokio::runtime::Runtime::new().unwrap(),
            requests: HashMap::new(),
        }
    }

    fn state(&mut self, id: RequestId) -> &mut RequestState {
        self.requests.entry(id).or_insert_with(RequestState::new)
    }

    fn spawn_request<F>(&mut self, id: RequestId, fut: F)
    where
        F: std::future::Future<Output = AppMessage> + Send + 'static,
    {
        let state = self.state(id);
        state.loading = true;
        let result = state.result.clone();

        self.runtime.spawn(async move {
            let msg = fut.await;
            *result.lock().unwrap() = Some(msg);
        });
    }

}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        for (id, state) in &mut self.requests {
            if let Some(msg) = state.result.lock().unwrap().take() {
                state.loading = false;
                println!("Request {:?} finished: {:?}", id, msg);
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            let state = self.state(RequestId::RequestData);

            if state.loading {
                ui.add_enabled(false, egui::Button::new("Request Data..."));
            } else if ui.button("Request Chat Log").clicked() {
                let api = self.api_client.clone();
                self.spawn_request(RequestId::RequestData, async move {
                    api.request_chat_log().await.unwrap_or(AppMessage::Error("failed".into()))
                });
            }

            ui.vertical(|ui|{
                let state = self.state(RequestId::Increase);

                if state.loading {
                    ui.add_enabled(false, egui::Button::new("Increasing..."));
                } else if ui.button("Increase").clicked() {
                    let api = self.api_client.clone();
                    self.spawn_request(RequestId::Increase, async move {
                        api.send_msg("Testing".to_string(),"Client".to_string()).await.unwrap_or(AppMessage::Error("failed".into()))
                    });
                }
            })
        });

    }
}

