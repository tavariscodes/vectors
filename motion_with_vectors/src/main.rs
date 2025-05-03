use eframe::egui::{self, Color32, Pos2, Rect, Vec2};
use rand::rng;
use rand_distr::{Distribution, UnitCircle};

fn random_unit_vec2() -> Vec2 {
    let mut rng = rng();
    let [x, y]: [f32; 2] = UnitCircle.sample(&mut rng);
    Vec2::new(x, y)
}

fn main() {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Accelerate Towards Mouse",
        options,
        Box::new(|cc| Ok(Box::<Mover>::default())),
    );
}

pub struct Mover {
    position: Pos2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Default for Mover {
    fn default() -> Self {
        Self {
            position: Pos2::new(0.0, 0.0),
            velocity: Vec2::new(-2.0, -2.0),
            acceleration: Vec2::new(-0.001, 0.01),
        }
    }
}

impl Mover {
    pub fn update(&mut self) {
        let max_speed = 5.0;

        self.acceleration = random_unit_vec2();

        self.acceleration = self.acceleration.normalized() * 2.0; 
        
        self.velocity += self.acceleration;

        if self.velocity.length() > max_speed {
            self.velocity = self.velocity.normalized() * max_speed;
        }

        self.position += self.velocity;
    }

    pub fn show(&self, painter: &egui::Painter) {
        painter.circle_filled(self.position, 20.0, Color32::GRAY);
    }

    pub fn check_edges(&mut self, display: &Rect) {
        if self.position.x > display.width() {
            self.position.x = 0.0;
        } else if self.position.x < 0.0 {
            self.position.x = display.width();
        }

        if self.position.y > display.height() {
            self.position.y = 0.0;
        } else if self.position.y < 0.0 {
            self.position.y = display.height();
        }
    }
}

// Acceleration = the rate of change of velocity.

impl eframe::App for Mover {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();
            painter.rect_filled(rect, 0.0, Color32::WHITE);

            self.update();
            self.check_edges(&rect);
            self.show(painter);

            ctx.request_repaint(); // Force continuous animation
        });
    }
}
