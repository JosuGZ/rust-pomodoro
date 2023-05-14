use notify_rust::Notification;

static START_MSG: &str = "¡A trabajar!";
static STOP_MSG: &str = "Hora de descansar";

pub fn notify_start() {
  let res = Notification::new()
    .appname("Omad") // TODO: Por alguna razón se muestra como Power Shell
    .summary("Omad")
    .body(START_MSG)
    .icon("gear") // TODO: Qué iconos puedo usar?
    .timeout(0)
    .show();

  if let Err(error) = res {
    println!("Error: {error:?}");
  }
}

pub fn notify_stop() {
  let res = Notification::new()
    .appname("Omad") // TODO: Por alguna razón se muestra como Power Shell
    .summary("Omad")
    .body(STOP_MSG)
    .icon("gear") // TODO: Qué iconos puedo usar?
    .timeout(0)
    .show();

  if let Err(error) = res {
    println!("Error: {error:?}");
  }
}
