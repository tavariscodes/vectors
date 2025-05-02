use std::ops::Add;

use eframe::egui::{self, Color32, Pos2, Vec2};

fn main() {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Vectors",
        options,
        Box::new(|cc| Ok(Box::<MyBall>::default())),
    );
}

pub struct MyBall {
    position: Pos2,
    velocity: Vec2,
}

impl Default for MyBall {
    fn default() -> Self {
        Self {
            position: Pos2::new(100.0, 100.0),
            velocity: Vec2::new(2.5, 2.0),
        }
    }
}

// A Euclidean Vector also known as the geometric vector is defined as
// an entity that has both magnitude and direction. Typically drawn as an arrow

// Magnitude is = length of Vector

impl eframe::App for MyBall {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();
            painter.rect_filled(rect, 0.0, Color32::WHITE);

            self.position = self.position.add(self.velocity);

            if self.position.x > rect.right() || self.position.x < rect.left() {
                self.velocity.x *= -1.0;
            }

            if self.position.y > rect.bottom() || self.position.y < rect.top() {
                self.velocity.y *= -1.0;
            }

            painter.circle_filled(self.position, 20.0, Color32::GRAY);

            ctx.request_repaint(); // Force continuous animation
        });
    }
}
