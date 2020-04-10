// Entry point for repl.it

use std::process::Command;

fn main() {
  std::process::exit(match Command::new("cargo").arg("test")
    .spawn()
    .expect("Can't run test")
    .wait()
    .expect("Test didn't finish")
    .code() {
      Some(code) => code,
      None => 127
    }
  );
}
