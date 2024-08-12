use eframe::*;
use egui::{ vec2, CentralPanel, Layout, Pos2, ViewportBuilder, WindowLevel};
use chrono::Local;

pub struct Myapp {
    time: String
}

impl Myapp {
    fn new() -> Self {
        Myapp {
            time: Local::now().format("%H:%M:%S").to_string()
        }
    }

    fn update_time(&mut self){
        self.time = Local::now().format("%H:%M:%S").to_string()
    }
}

impl eframe::App for Myapp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        self.update_time();
        CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::centered_and_justified(egui::Direction::TopDown), |ui|
            {
                ui.label(&self.time);
            })
        });
        ctx.request_repaint()
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    run_native(
        "My app",
        NativeOptions {
                viewport: ViewportBuilder{
                    position: Some(Pos2::new(0.0, 0.0)),
                    inner_size: Some(vec2(1536.0, 25.0)),
                    decorations: Some(false),
                    window_level: Some(WindowLevel::AlwaysOnTop),
                    ..ViewportBuilder::default()
                },
                ..NativeOptions::default()
        },
        Box::new(|_cc|{
            Ok(Box::new(Myapp::new()))
        })
    )
}
