pub fn fibonacci(n: usize) -> usize {
  fibonacci(n - 1) + fibonacci(n - 2)
}

pub fn run() {
  println!("***** Algorithm: Recursive Attempt *****");

  println!("fibonacci(0) = {}", fibonacci(0));
  println!("fibonacci(1) = {}", fibonacci(1));
  // println!("fibonacci(5) = {}", fibonacci(5));
}
