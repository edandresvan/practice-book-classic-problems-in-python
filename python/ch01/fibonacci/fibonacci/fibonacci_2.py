def fibonacci(n: int) -> int:
  if n < 2:
    return n
  return fibonacci(n - 2) + fibonacci(n - 1)


def run():
  print("***** Algorithm: Recursive with Base Cases *****")

  print(f"fibonacci(0) = {fibonacci(0)}")
  print(f"fibonacci(1) = {fibonacci(1)}")
  print(f"fibonacci(5) = {fibonacci(5)}")
  print(f"fibonacci(20) = {fibonacci(20)}")
  # print(f"fibonacci(50) = {fibonacci(50)}")
