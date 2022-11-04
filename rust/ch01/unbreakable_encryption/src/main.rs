use clap::{command, crate_authors, Arg, ArgMatches};

mod encryption;
use encryption::{decrypt, encrypt};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
  let args: ArgMatches = command!()
    .author(crate_authors!("\n"))
    .arg(
      Arg::new("data")
        .value_name("DATA")
        .help("Data to encrypt and decrypt")
        .required(true),
    )
    .get_matches();

  let original_data: &String = args.get_one::<String>("data").unwrap();

  let (key1, key2) = encrypt(&original_data);

  let result: String = decrypt(&key1, &key2);

  println!("key1 = {:?}", &key1);
  println!("key2 = {:?}", &key2);

  println!("original = {}", &original_data);
  println!("result   = {}", &result);

  Ok(())
}
