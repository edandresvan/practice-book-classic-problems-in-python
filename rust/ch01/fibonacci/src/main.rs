use clap::{arg, ArgMatches, Command};

mod fibonacci_1;
mod fibonacci_2;
mod fibonacci_3;
mod fibonacci_4;
mod fibonacci_5;

fn cli() -> Command {
  let long_help: &str = "\
1: Recursive infinite.
2: Recursive with base cases.
3: HashMap memoization.
4: Cached memoization
5: Iterative approach";
  Command::new("fibonacci")
    .about("Calculate the fibonacci using several algorithms")
    .arg(
      arg!(-a --algorithm <NUMBER> "Algorithm to use")
        .value_parser(["1", "2", "3", "4", "5", "6"])
        .required(true)
        .long_help(long_help),
    )
}

fn main() {
  let args: ArgMatches = cli().get_matches();

  let algorithm: &String = args.get_one::<String>("algorithm").unwrap();

  match algorithm.as_str() {
    "1" => fibonacci_1::run(),
    "2" => fibonacci_2::run(),
    "3" => fibonacci_3::run(),
    "4" => fibonacci_4::run(),
    "5" => fibonacci_5::run(),
    "6" => println!("Sorry. Generators in Rust is an experimental feature."),
    _ => println!("invalid option."),
  }
}
