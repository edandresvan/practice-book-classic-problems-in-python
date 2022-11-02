pub struct CompressedGene {
  pub bit_string: Vec<u8>,
  pub gene_size: usize,
}

impl CompressedGene {
  pub fn new() -> Self {
    Self {
      bit_string: vec![],
      gene_size: 0,
    }
  }

  pub fn compress(
    &mut self,
    gene: &str,
  ) -> Result<(), String> {
    // if gene.is_empty() {
    //   return Ok(())
    // }

    self.bit_string = vec![];
    self.gene_size = 0;

    let mut nucleotide_group: u8 = 0;
    let mut nucleotide_index: i8 = 6;
    let mut nucleotides_iterator = gene.chars().peekable();

    'fill_byte: while nucleotide_index >= 0 {
      if nucleotides_iterator.peek().is_none() {
        break 'fill_byte;
      }

      let nucleotide_char: char = nucleotides_iterator.next().unwrap();

      let mut nucleotide_bits: u8 = match nucleotide_char {
        'A' => 0b00,
        'C' => 0b01,
        'G' => 0b10,
        'T' => 0b11,
        _ => return Err(format!("Invalid nucleotide: {}", nucleotide_char)),
      };

      nucleotide_bits <<= nucleotide_index;
      nucleotide_group |= nucleotide_bits;

      self.gene_size += 1;

      if nucleotide_index == 0 || nucleotides_iterator.peek().is_none() {
        self.bit_string.push(nucleotide_group);
        nucleotide_group = 0u8;
        nucleotide_index = 6;
      } else {
        nucleotide_index -= 2;
      }
    }

    Ok(())
  }

  pub fn decompress(&self) -> Result<String, String> {
    let mut gene: String = String::with_capacity(self.gene_size);
    let mut gene_count: usize = 0;

    'fill_string: for nucleotide_group in &self.bit_string {
      for nucleotide_index in (0..8u8).step_by(2).rev() {
        if gene_count >= self.gene_size {
          break 'fill_string;
        }

        let nucleotide_bits =
          (nucleotide_group & (0b11 << nucleotide_index)) >> nucleotide_index;

        match nucleotide_bits {
          0b00 => gene.push('A'),
          0b01 => gene.push('C'),
          0b10 => gene.push('G'),
          0b11 => gene.push('T'),
          _ => return Err(format!("Invalid bits: {}", nucleotide_bits)),
        }

        gene_count += 1;
      }
    }
    Ok(gene)
  }
}

impl std::fmt::Display for CompressedGene {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let displayed = self.decompress().unwrap();
    write!(f, "{}", displayed)
  }
}

impl std::fmt::Binary for CompressedGene {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    let mut displayed: String = String::new();

    for gene_group in &self.bit_string {
      displayed += &format!("{:0>8b}", gene_group);
    }

    write!(f, "{}", displayed)
  }
}
