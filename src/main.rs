use clap::{App, Arg};
use rand::Rng;


fn main() {
    let matches = App::new("Pasword Generator")
    .version("1.0")
    .author("Oskars")
    .about("Generates an specified random password")
    .arg(
        Arg::with_name("length")
        .short('l')
        .long("length")
        .value_name("LENGTH")
        .help("Sets the length of the password")
        .takes_value(true),
    )
    .get_matches();

    let length: usize = matches.value_of("length").unwrap_or("12").parse().unwrap_or(12);

    let password = generate_password(length);
    match password {
        Ok(p) => println!("Generated Password: {}",p),
        Err(e) => println!("Error: {}", e),
    }
}


fn generate_password(length: usize) -> Result<String, &'static str> {
    if length == 0 {
        return Err("Password length must be greater than zero");
    }
    let chars: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    Ok((0..length)
        .map(|_| char::from(chars[rng.gen_range(0..chars.len())]))
        .collect())
}