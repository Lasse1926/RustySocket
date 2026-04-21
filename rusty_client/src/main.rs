use std::thread;
use std::sync::mpsc::{self, Sender, Receiver};
use reqwest::{header::USER_AGENT, Client};
use serde::Deserialize;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native("My egui App", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Deserialize, Debug)]
struct Todo {
  userId: i32,
  id: i32,
  title: String,
  completed: bool
}

enum AppMessage {
    Todo(Todo),
    Error(String),
}

struct MyEguiApp {
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
            tx,
            rx, 
        }
    }
    fn request_data(&self) {
        let tx = self.tx.clone();
        thread::spawn(move || {
            let res = reqwest::blocking::get("https://jsonplaceholder.typicode.com/todos/1")
                .and_then(|r| r.json::<Todo>());

            match res {
                Ok(todo) => {
                    let _ = tx.send(AppMessage::Todo(todo));
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
                AppMessage::Todo(todo) => {
                    println!("{}",todo.title);
                }
                AppMessage::Error(e) => {
                }
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("send request").clicked() {
                self.request_data();
            }
        });
    }
}

