mod app;
mod app_thread;
mod notify;

const APP_NAME: &str = "Omad";

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let options = eframe::NativeOptions {
    resizable: false,
    vsync: false,
    initial_window_size: Some([400.0, 400.0].into()),
    ..Default::default()
  };

  eframe::run_native(APP_NAME, options, Box::new(|_creation_context| {
    Box::new(app::Omad::new())
  }));
}
