use eframe::egui;
use eframe::egui::{CentralPanel, Label};

/// A simple GUI application for LongYield.
struct MyApp {
    message: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            message: "Welcome to LongYield GUI!".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("LongYield Node GUI");
            ui.label(&self.message);

            if ui.button("Start Mining").clicked() {
                self.message = "Mining started!".to_owned();
                // TODO: Integrate the mining functionality here.
            }

            if ui.button("Stop Mining").clicked() {
                self.message = "Mining stopped!".to_owned();
                // TODO: Integrate logic to stop mining.
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "LongYield GUI",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
