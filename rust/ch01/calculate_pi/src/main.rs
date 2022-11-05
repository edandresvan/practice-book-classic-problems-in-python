use clap::{command, crate_authors, value_parser, Arg, ArgMatches};

mod calculating_pi;

use calculating_pi::calculate_pi;

fn main() {
  let args: ArgMatches = command!()
    .author(crate_authors!("\n"))
    .arg(
      Arg::new("nterms")
        .short('n')
        .long("nterms")
        .help("Number of terms of the series of the Leibniz formula")
        .default_value("1000000")
        .value_parser(value_parser!(u32))
        .required(false),
    )
    .get_matches();

  let n_terms: &u32 = args.get_one::<u32>("nterms").unwrap();

  println!("pi = {}", calculate_pi(*n_terms));
}
