extern crate readline;
use readline::*;

#[allow(while_true)]

fn main() {
  println!("Ruspy Version 0.0.0.0.3");
  println!("Press Ctrl+c to Exit\n");

  /* In a never ending loop */
  loop {
      match readline("luspy> ") {
          Some(line) => {
            add_history(line.as_ref());
            println!("No you're a {}", line)
        },
          None => println!("")
    }
  }
}