use std::io;
use std::io::Write;

#[allow(while_true)]

fn main() {
  println!("Ruspy Version 0.0.0.0.3");
  println!("Press Ctrl+c to Exit\n");

  /* In a never ending loop */
  while true {

    /* Output our prompt */
    print!("ruspy> ");
    io::stdout().flush().unwrap();

    /* Read a line of user input of maximum size 2048 */
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    /* Echo input back to user */
    print!("No you're a {}", input);
  }
}