
def fibonacci(n: int) -> int:
  if  n == 0: return n

  last: int = 0
  next: int = 1

  for _ in range(1, n):
    last, next = next, last + next 
  
  return next

def run():
  print("***** Algorithm: Iterative Calculation *****")

  print(f"fibonacci(0) = {fibonacci(0)}")
  print(f"fibonacci(1) = {fibonacci(1)}")
  print(f"fibonacci(5) = {fibonacci(5)}")
  print(f"fibonacci(20) = {fibonacci(20)}")
  print(f"fibonacci(50) = {fibonacci(50)}")
