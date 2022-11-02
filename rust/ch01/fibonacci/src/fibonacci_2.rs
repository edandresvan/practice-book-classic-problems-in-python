pub fn fibonacci(n: usize) -> usize {
  if n < 2 {
    return n;
  }

  fibonacci(n - 2) + fibonacci(n - 1)
}

pub fn run() {
  println!("***** Algorithm: Recursive with Base Cases *****");

  println!("fibonacci(0) = {}", fibonacci(0));
  println!("fibonacci(1) = {}", fibonacci(1));
  println!("fibonacci(5) = {}", fibonacci(5));
  println!("fibonacci(20) = {}", fibonacci(20));
  // println!("fibonacci(50) = {}", fibonacci(50));
}
