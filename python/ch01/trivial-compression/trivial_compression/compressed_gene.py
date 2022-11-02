class CompressedGene:
  def __init__(self, gene: str):
    self._compress(gene)

  def _compress(self, gene: str):
    self.bit_string: int = 1
    
    for nucleotide in gene.upper():
      self.bit_string <<= 2
      match nucleotide:
        case "A":
          self.bit_string |= 0b00
        case "C":
          self.bit_string |= 0b01
        case "G":
          self.bit_string |= 0b10
        case "T":
          self.bit_string |= 0b11
        case _: 
          raise ValueError("Invalid Nucleotide:{nucleotide}")

  def decompress(self) -> str:
    gene: str = ""
    for i in range(0, self.bit_string.bit_length() - 1, 2):
      bits: int = self.bit_string >> i & 0b11

      match bits:
        case 0b00:
          gene += "A"
        case 0b01:
          gene += "C"
        case 0b10:
          gene += "G"
        case 0b11:
          gene += "T"
        case _:
          raise ValueError(f"Invalid bits:{bits}")
    
    return gene[::-1]

  def __str__(self) -> str:
    return self.decompress()