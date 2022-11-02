from sys import getsizeof
from .compressed_gene import CompressedGene


def main():
  original: str = "TAGGGATTAACCGTTATATATATATAGCCATGGATCGATTATATAGGGATTAACCGTTATATATATATAGCCATGGATCGATTATA" * 100
  print(f"original: {original}")
  print(f"original is {getsizeof(original)} bytes")
  compressed: CompressedGene = CompressedGene(original)
  print()
  print(f"compressed is: {compressed.bit_string:b}")
  print(f"compressed is {getsizeof(compressed.bit_string)} bytes")


if __name__ == "__main__":
  main()
