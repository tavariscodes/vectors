use std::default;

use eframe::egui::{self};

fn main() {
    env_logger::init();
    println!("Hello, world!");

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };


    eframe::run_native("Vectors", options, Box::new(|cc| Ok(Box::<MyApp>::default()))
);
}

struct MyApp {

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
    }
}