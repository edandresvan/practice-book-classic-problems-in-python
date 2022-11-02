use std::fs;

use assert_cmd::{self, Command};
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const BIN_NAME: &'static str = env!("CARGO_PKG_NAME");

fn run_test(
  args: &[&str],
  expected_file: &str,
) -> TestResult {
  let expected: String = fs::read_to_string(expected_file)?;

  Command::cargo_bin(BIN_NAME)?
    .args(args)
    .assert()
    .success()
    .stdout(predicate::str::contains(expected));
  Ok(())
}

#[test]
fn test_gene_empty() -> TestResult {
  run_test(&[], "tests/expected/compress_gene_0.txt")
}

#[test]
fn test_gene_a1() -> TestResult {
  run_test(&["A"], "tests/expected/compress_gene_a1.txt")
}

#[test]
fn test_gene_a2() -> TestResult {
  run_test(&["AA"], "tests/expected/compress_gene_a2.txt")
}

#[test]
fn test_gene_a3() -> TestResult {
  run_test(&["AAA"], "tests/expected/compress_gene_a3.txt")
}

#[test]
fn test_gene_a4() -> TestResult {
  run_test(&["AAAA"], "tests/expected/compress_gene_a4.txt")
}

#[test]
fn test_gene_a5() -> TestResult {
  run_test(&["AAAA", "A"], "tests/expected/compress_gene_a5.txt")
}

#[test]
fn test_gene_a6() -> TestResult {
  run_test(&["AAAA", "AA"], "tests/expected/compress_gene_a6.txt")
}

#[test]
fn test_gene_t1() -> TestResult {
  run_test(&["T"], "tests/expected/compress_gene_t1.txt")
}

#[test]
fn test_compress_gene_t4() -> TestResult {
  run_test(&["TTTT"], "tests/expected/compress_gene_t4.txt")
}

#[test]
fn test_compress_gene_2nu() -> TestResult {
  run_test(&["AT"], "tests/expected/compress_gene_2nu.txt")
}

#[test]
fn test_compress_gene_3nu() -> TestResult {
  run_test(&["TAC"], "tests/expected/compress_gene_3nu.txt")
}

#[test]
fn test_compress_gene_4nu() -> TestResult {
  run_test(&["TACG"], "tests/expected/compress_gene_4nu.txt")
}

#[test]
fn test_compress_gene_6nu() -> TestResult {
  run_test(&["TACG", "CG"], "tests/expected/compress_gene_6nu.txt")
}

#[test]
fn test_compress_gene_book() -> TestResult {
  let input: String = fs::read_to_string("tests/input/input_gene_book.txt")?;
  run_test(&[&input], "tests/expected/compress_gene_book.txt")
}
