from typing import Dict

memo: Dict[int, int] = {0: 0, 1: 1}


def fibonacci(n: int) -> int:
  if n not in memo:
    memo[n] = fibonacci(n - 1) + fibonacci(n - 2)

  return memo[n]


def run():
  print("***** Algorithm: HashMap Memoization *****")

  print(f"fibonacci(0) = {fibonacci(0)}")
  print(f"fibonacci(1) = {fibonacci(1)}")
  print(f"fibonacci(5) = {fibonacci(5)}")
  print(f"fibonacci(20) = {fibonacci(20)}")
  print(f"fibonacci(50) = {fibonacci(50)}")
