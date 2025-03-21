mod app;
use app::MyApp;
use eframe::{self, egui};

fn main() -> Result<(), eframe::Error> {
    // Graphic application launch
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Password Generator",
        options,
        Box::new(|cc| {
            let visuals = egui::Visuals::dark();
            cc.egui_ctx.set_visuals(visuals);

            Ok(Box::new(MyApp::default()))
        }),
    )
}
