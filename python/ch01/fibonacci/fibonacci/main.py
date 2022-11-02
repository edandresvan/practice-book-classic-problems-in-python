import argparse
from fibonacci import fibonacci_1, fibonacci_2, fibonacci_3, fibonacci_4, fibonacci_5, fibonacci_6

def main():
  long_help: str = """1: Recursive infinite.
2: Recursive with base cases.
3: HashMap memoization.
4: Cached memoization.
5: Iterative approach.
6: Using a generator.
"""
  parser: argparse.ArgumentParser = argparse.ArgumentParser()

  parser.add_argument("--algorithm", type = int, choices=[1, 2, 3, 4, 5, 6], required=True, help=long_help)

  args: argparse.Namespace = parser.parse_args()

  if args.algorithm is not None:
    match args.algorithm:
      case 1:
        fibonacci_1.run()
      case 2:
        fibonacci_2.run()
      case 3:
        fibonacci_3.run()
      case 4:
        fibonacci_4.run()
      case 5:
        fibonacci_5.run()
      case 6:
        fibonacci_6.run()
      case _:
        print(f"{args.algorithm} is not a valid algorithm option.")


if __name__ == "__main__":
  main()
  
