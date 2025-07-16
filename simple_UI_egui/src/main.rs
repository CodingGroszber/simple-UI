mod app;
mod text_viewer;

use app::MyApp;
use eframe::NativeOptions;

fn main() {
    let options = NativeOptions::default();
    eframe::run_native(
        "Text Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    )
    .unwrap();
}
