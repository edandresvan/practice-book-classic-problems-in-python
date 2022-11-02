from typing import Generator


def fibonacci(n: int) -> Generator[int, None, None]:
  yield 0

  if n > 0: yield 1

  last: int = 0
  next: int = 1

  for _ in range(1, n):
    last, next = next, last + next
    yield next


def run():
  print("***** Algorithm: Generator Function *****")

  print("--- fibonacci(0) ---")
  for value in fibonacci(0):
    print( value)
  
  print()

  print("--- fibonacci(1) ---")
  for value in fibonacci(1):
    print( value)
  
  print()

  print("--- fibonacci(5) ---")
  for value in fibonacci(5):
    print( value)
  
  print()

  print("--- fibonacci(20) ---")
  for value in fibonacci(20):
    print( value)
  
  print()

  print("--- fibonacci(50) ---")
  for value in fibonacci(50):
    print( value)
  
  print()
