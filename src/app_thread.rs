use std::sync::Once;

static START: Once = Once::new();

pub fn init_once(ctx: egui::Context) {
  START.call_once(move || {
    println!("Starting thread.");
    let _handle = std::thread::spawn(move || { // TODO: Por ahora nunca terminamos
      loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
        ctx.request_repaint();
      }
    });
  });
}
