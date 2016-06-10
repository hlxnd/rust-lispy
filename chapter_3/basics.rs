fn main() {
  println!("Sum: {}", add_together(5,6));
  println!("Length: {}", length(Point {x: 5.,y: 6.}));
  println!("x=56 and ...");
  check_val(56);
  iterate();
}

// Functions
fn add_together(x: i32, y: i32) -> i32 {
  x + y
}

// Structs
struct Point {
    x: f64,
    y: f64,
}

fn length(p: Point) -> f64 {
  (p.x.powf(2.)+p.y.powf(2.)).sqrt()
}

// Condtionals
fn check_val(x: i32) -> () {
  if x > 10 && x < 100 {
    println!("x is greater than 10 and less than 100!");
  } else {
    println!("x is less than 11 or greater than 99!");
  }
}

// Loops
fn iterate() -> () {
  // while
  let mut i=10;
  while i > 0 {
    println!("Loop Iteration with while ({} to go)",i);
    i = i - 1;
  }
 
  // for
  for i in 0..10 {
    println!("Loop Iteration with for (Loop {})",i);
  }
}