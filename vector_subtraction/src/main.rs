use eframe::{
    egui::{self, Color32, Pos2},
    epaint::PathStroke,
};

fn main() {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Vector Subtraction",
        options,
        Box::new(|cc| Ok(Box::<MyLine>::default())),
    );
}

struct MyLine {}

impl Default for MyLine {
    fn default() -> Self {
        Self {}
    }
}

impl eframe::App for MyLine {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();
            painter.rect_filled(rect, 0.0, Color32::WHITE);

            let origin = Pos2::new(0.0, 0.0);
            let center = rect.center();

            if let Some(mouse_pos) = ctx.pointer_hover_pos() {
                
                let vector_sub_result = mouse_pos - center;

                painter.line(
                    vec![origin, mouse_pos],
                    PathStroke::new(2.0, Color32::LIGHT_GRAY),
                );

                painter.line(
                    vec![origin, center],
                    PathStroke::new(2.0, Color32::LIGHT_GRAY),
                );

                painter.line(
                    vec![center, center + vector_sub_result],
                    PathStroke::new(3.0, Color32::BLACK),
                );
            }

            ctx.request_repaint(); // Force continuous animation
        });
    }
}
