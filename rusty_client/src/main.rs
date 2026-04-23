use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use reqwest::{header::USER_AGENT, Client};
use serde::Deserialize;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

// #[derive(Deserialize, Debug)]
// struct Todo {
//   userId: i32,
//   id: i32,
//   title: String,
//   completed: bool
// }

enum AppMessage {
    Msg(String),
    Error(String),
}

struct MyEguiApp {
    client:reqwest::blocking::Client,
    tx: Sender<AppMessage>,
    rx: Receiver<AppMessage>,

}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let (tx, rx) = std::sync::mpsc::channel::<AppMessage>();
        Self { 
            client: reqwest::blocking::Client::new(),
            tx,
            rx, 
        }
    }
    fn request_data(&self) {
        let tx = self.tx.clone();
        let client = self.client.clone();
        thread::spawn(move || {
            let res = client.get("http://localhost:8080/counter").send()
                        .and_then(|r| r.text());

            match res {
                Ok(msg) => {
                    let _ = tx.send(AppMessage::Msg(msg));
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::Error(e.to_string()));
                }
            }
        });
    }

    fn increase_counter(&self) {
        let tx = self.tx.clone();
        let client = self.client.clone();
        thread::spawn(move || {
            let res = client.put("http://localhost:8080/counter/increase").send()
                        .and_then(|r| r.text());

            match res {
                Ok(msg) => {
                    let _ = tx.send(AppMessage::Msg(msg));
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::Error(e.to_string()));
                }
            }
        });
    }

    fn decreas_counter(&self) {
        let tx = self.tx.clone();
        let client = self.client.clone();
        thread::spawn(move || {
            let res = client.put("http://localhost:8080/counter/decrease").send()
                        .and_then(|r| r.text());

            match res {
                Ok(msg) => {
                    let _ = tx.send(AppMessage::Msg(msg));
                }
                Err(e) => {
                    let _ = tx.send(AppMessage::Error(e.to_string()));
                }
            }
        });
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::Msg(msg) => {
                    println!("{}",msg);
                }
                AppMessage::Error(e) => {
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("send request").clicked() {
                self.request_data();
            }

            ui.vertical(|ui|{
                if ui.button("Increase").clicked() {
                    self.increase_counter();
                }

                if ui.button("Decrease").clicked() {
                    self.decreas_counter();
                }
            })
        });

    }
}

