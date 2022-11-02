pub fn fibonacci(n: usize) -> usize {
  if n == 0 {
    return n;
  }

  let mut last: usize = 0;
  let mut next: usize = 1;

  for _ in 1..n {
    let temp: usize = last;
    last = next;
    next = temp + next;
  }
  next
}

pub fn run() {
  println!("***** Algorithm: Iterative Calculation *****");

  println!("fibonacci(0) = {}", fibonacci(0));
  println!("fibonacci(1) = {}", fibonacci(1));
  println!("fibonacci(5) = {}", fibonacci(5));
  println!("fibonacci(20) = {}", fibonacci(20));
  println!("fibonacci(50) = {}", fibonacci(50));
}
