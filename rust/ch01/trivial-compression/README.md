# Trivial Compression of Genes

In this Rust project, a long gene sequence of nucleotides A, C, G, and T is transformed from a string of characters into vector of 8-bit unsigned integers.

This is a departure from the book because Python can increase the capacity of an integer automatically, but in Rust we have restricted integers (u8, u16, u32, u64, u128).

So, in order to practice Rust programming, an of 8-bit unsigned integer is utilized to store up to 4 nucleotides every two bits.

The compression algorithm is similar to filling boxes with four elements, each until finally obtaining a final box probably with less than 4 elements. This allows cases where the number of nucleotides is either odd or not multiple of 4. However, the number of nucleotides must be counted in order to avoid the last 8-bit integer represents additional nucleotides.

In the decompression function the gene is rebuilt in sequential order up to reaching the size of the original gene. In each 8-bit integer every pair of bits is masked and shifted to the right to allow a proper matching. For example, a masked nucleotide 'T' `11000000` should be changed to `00000011` to produce a 'T' character.

To run the program use these commands:

```shell
$ cargo run -- AAGT TGCC TGAT
$ cargo run -- AAGTTGCCTGAT
$ cargo run -- "$(< ./tests/input/input_gene_book.txt)"
```
  
