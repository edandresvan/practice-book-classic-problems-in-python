use std::mem::size_of_val;

use clap::{command, crate_authors, Arg, ArgAction, ArgMatches};

use crate::compressed_gene::CompressedGene;

mod compressed_gene;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  let args: ArgMatches = command!()
    .author(crate_authors!("\n"))
    .arg(
      Arg::new("gene")
        .value_name("GENE")
        .help("The gene secuence to compress")
        .action(ArgAction::Append)
        .required(false),
    )
    .get_matches();

   
  let original_gene = args
    .get_many::<String>("gene")
    .unwrap_or_default()
    .map(|v| v.to_string())
    .collect::<Vec<String>>()
    .join("");

  let mut compressed: CompressedGene = CompressedGene::new();
  compressed.compress(&original_gene)?;

  for byte in &compressed.bit_string {
    print!("{:08b} ", byte);
  }
  println!();
  
  let compressed_size: usize =
    compressed.bit_string.iter().map(|&g| size_of_val(&g)).sum();

  println!("Original gene has {} bytes", original_gene.len());

  println!("Compressed gene has {} bytes", compressed_size);

  println!("Compressed gene has {} nucleotides", compressed.gene_size);
  println!("Compressed gene is\n{:b}", compressed);

  //println!("Decompressed gene is \n{}", &compressed.decompress()?);
  println!("Decompressed gene is\n{}", &compressed);

  Ok(())
}
