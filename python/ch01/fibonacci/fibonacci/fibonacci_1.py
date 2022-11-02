def fibonacci(n: int) -> int:
  return fibonacci(n - 1) + fibonacci(n - 2)


def run():
  print("***** Algorithm: Recursive Attempt *****")

  print(f"fibonacci(0) = {fibonacci(0)}")
  print(f"fibonacci(1) = {fibonacci(1)}")
  print(f"fibonacci(5) = {fibonacci(5)}")
