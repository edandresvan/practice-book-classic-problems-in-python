use std::collections::HashMap;

pub fn fibonacci(n: usize) -> usize {
  let mut memo: HashMap<usize, usize> = HashMap::from([(0, 0), (1, 1)]);
  get_fibonacci(n, &mut memo)
}

fn get_fibonacci(
  n: usize,
  memo: &mut HashMap<usize, usize>,
) -> usize {
  if memo.get(&n).is_none() {
    let value: usize = get_fibonacci(n - 1, memo) + get_fibonacci(n - 2, memo);
    memo.insert(n, value);
  }
  memo.get(&n).unwrap().clone()
}

pub fn run() {
  println!("***** Algorithm: HashMap Memoization *****");

  println!("fibonacci(0) = {}", fibonacci(0));
  println!("fibonacci(1) = {}", fibonacci(1));
  println!("fibonacci(5) = {}", fibonacci(5));
  println!("fibonacci(20) = {}", fibonacci(20));
  println!("fibonacci(50) = {}", fibonacci(50));
}
