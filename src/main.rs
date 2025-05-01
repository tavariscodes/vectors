use eframe::egui::{self, Color32, Pos2};

fn main() {
    env_logger::init();
    println!("Hello, world!");

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
    x: f32,
    y: f32,
    xspeed: f32,
    yspeed: f32,
}

impl Default for MyBall {
    fn default() -> Self {
        Self {
            x: 100.0,
            y: 100.0,
            xspeed: 2.5,
            yspeed: 2.0
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

            self.x += self.xspeed;
            self.y += self.yspeed;


            if self.x > rect.width() || self.x < 0.0 {
                self.xspeed *= -1.0;
            }

            if self.y > rect.height() || self.y < 0.0 {
                self.yspeed *= -1.0;
            }

            painter.circle_filled(Pos2::new(self.x, self.y), 20.0, Color32::GRAY);

            ctx.request_repaint(); // Force continuous animation

        });
    }
}
