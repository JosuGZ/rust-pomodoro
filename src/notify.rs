static START_MSG: &str = "¡A trabajar!";
static STOP_MSG: &str = "Hora de descansar";

pub fn notify_start() {
  #[cfg(not(target_os = "linux"))]
  {
    use notify_rust::Notification;
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

  #[cfg(target_os = "linux")]
  {
    use std::process::Command;
    let mut command = Command::new("notify-send");
    command.arg("-u");
    command.arg("critical");
    command.arg("-i");
    command.arg("info");
    command.arg(START_MSG);
    let res = command.output();

    if let Err(error) = res {
      println!("Error: {error:?}");
    }
  }
}

pub fn notify_stop() {
  #[cfg(not(target_os = "linux"))]
  {
    use notify_rust::Notification;
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

  #[cfg(target_os = "linux")]
  {
    use std::process::Command;
    let mut command = Command::new("notify-send");
    command.arg("-u");
    command.arg("critical");
    command.arg("-i");
    command.arg("info");
    command.arg(STOP_MSG);
    let res = command.output();

    if let Err(error) = res {
      println!("Error: {error:?}");
    }
  }
}
