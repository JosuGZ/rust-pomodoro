use std::time::Instant;
use eframe::App;
use egui::Slider;

enum Status {
  Stopped, Working, Resting
}

pub struct Omad {
  total_pomodoros: u32,
  total_pomodoros_since_long_rest: u32,
  working_time_min: u32,
  resting_time_min: u32,
  pomodoros_before_long_rest: u32,
  long_resting_time_min: u32,
  updates: u32,
  status: Status,
  pomodoro_start_time: Option<Instant>
}

impl Omad {
  pub fn new() -> Self {
    Omad {
      total_pomodoros: 0,
      total_pomodoros_since_long_rest: 0,
      working_time_min: 25,
      resting_time_min: 5,
      pomodoros_before_long_rest: 3,
      long_resting_time_min: 30,
      updates: 0,
      status: Status::Stopped,
      pomodoro_start_time: None
    }
  }

  fn on_start(&mut self) {
    self.start_working();
  }

  fn on_stop(&mut self) {
    crate::notify::notify_stop(); // TODO: No siempre stop
    self.status = Status::Stopped;
    self.pomodoro_start_time = None;
  }

  fn on_update(&mut self) {
    self.updates += 1;

    match self.status {
      Status::Working => self.on_update_working(),
      Status::Resting => self.on_update_resting(),
      Status::Stopped => {}
    }
  }

  fn on_update_working(&mut self) {
    let elapsed = self.pomodoro_start_time.unwrap().elapsed(); // Unwrap: it shoudn't be None here
    if elapsed.as_secs() >= 60 * self.working_time_min as u64 {
      self.start_resting();
    }
  }

  fn on_update_resting(&mut self) {
    let elapsed = self.pomodoro_start_time.unwrap().elapsed(); // Unwrap: it shoudn't be None here

    if self.total_pomodoros_since_long_rest >= self.pomodoros_before_long_rest {
      if elapsed.as_secs() >= 60 * self.long_resting_time_min as u64 {
        self.total_pomodoros_since_long_rest = 0;
        self.start_working();
      }
    } else if elapsed.as_secs() >= 60 * self.resting_time_min as u64 {
      self.start_working();
    }
  }

  fn start_working(&mut self) {
    crate::notify::notify_start();
    self.status = Status::Working;
    self.pomodoro_start_time = Some(Instant::now());
  }

  fn start_resting(&mut self) { // ¿Puedo forzar a que sólo sea llamada por update?
    crate::notify::notify_stop();
    self.total_pomodoros += 1;
    self.total_pomodoros_since_long_rest += 1;
    self.status = Status::Resting;
    self.pomodoro_start_time = Some(Instant::now());
  }
}

impl App for Omad {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    crate::app_thread::init_once(ctx.clone());

    self.on_update();

    egui::CentralPanel::default().show(ctx, |ui| {

      ui.vertical(|ui| {
        ui.heading("Estadísticas");
        ui.label(format!("Pomodoros hechos: {}", self.total_pomodoros));
        ui.label(format!("Updates: {}", self.updates));
        if let Some(start_time) = self.pomodoro_start_time {
          let time = start_time.elapsed().as_secs(); // TODO: Improve
          ui.label(format!("Tiempo transcurrido en el último intervalo: {}", time));
        }
        ui.heading("Configuración");
        ui.label(format!("Working time (minutes): {}", self.working_time_min));
        let working_slider = Slider::new(&mut self.working_time_min, 1..=240).step_by(1.0).smart_aim(false);
        ui.add(working_slider);
        ui.label(format!("Resting time (minutes): {}", self.resting_time_min));
        let resting_slider = Slider::new(&mut self.resting_time_min, 1..=90).step_by(1.0).smart_aim(false);
        ui.add(resting_slider);
        ui.label(format!("Pomodoros before long rest: {}", self.pomodoros_before_long_rest));
        let pomodoros_before_long_rest = Slider::new(&mut self.pomodoros_before_long_rest, 1..=10).step_by(1.0).smart_aim(false);
        ui.add(pomodoros_before_long_rest);
        ui.label(format!("Long resting time (minutes): {}", self.long_resting_time_min));
        let long_resting_slider = Slider::new(&mut self.long_resting_time_min, 1..=240).step_by(1.0).smart_aim(false);
        ui.add(long_resting_slider);
      });
    });

    egui::TopBottomPanel::bottom("Buttons").show(ctx, |ui| {
      ui.horizontal(|ui| {

        match self.status {
          Status::Working | Status::Resting => {
            if ui.button("Stop").clicked() {
              self.on_stop();
            }
          },
          Status::Stopped => {
            if ui.button("Start").clicked() {
              self.on_start();
            }
          }
        }
      });
    });
  }
}
