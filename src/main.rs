// Import the `App` and `Arg` types from the `clap` crate
use clap::{App, Arg};
// Import the `Rng` trait from the `rand` crate
use rand::Rng;

use wasm_bindgen::prelude::*;

// Define the entry point for the program
fn main() {
    // Create a new `App` with the program name, version, author, and description
    let matches = App::new("Pasword Generator")
        .version("1.0")
        .author("Oskars")
        .about("Generates an specified random password")
        // Define the `length` argument
        .arg(
            Arg::with_name("length")
                .short('l')
                .long("length")
                .value_name("LENGTH")
                .help("Sets the length of the password")
                .takes_value(true),
        )
        // Define the `complexity` argument
        .arg(
            Arg::with_name("complexity")
                .short('c')
                .long("complexity")
                .value_name("COMPLEXITY")
                .help("Sets the complexity of the password (low, medium, high)")
                .takes_value(true),
        )
        // Parse the command-line arguments and get a `Matches` struct
        .get_matches();

    // Get the value of the `length` argument or use a default of 12
    let length: usize = matches.value_of("length").unwrap_or("12").parse().unwrap_or(12);

    // Get the value of the `complexity` argument or use a default of `Medium`
    let complexity: Complexity = match matches.value_of("complexity") {
        Some("low") => Complexity::Low,
        Some("medium") => Complexity::Medium,
        Some("high") => Complexity::High,
        _ => Complexity::Medium,
    };

    // Generate a password with the given length and complexity
    let password = generate_password(length, complexity);
    // Print the password or an error message
    match password {
        Ok(p) => println!("Generated Password: {}", p),
        Err(e) => println!("Error: {}", e),
    }
}

// Define an enum to represent the different password complexities
#[derive(Debug)]
enum Complexity {
    Low,
    Medium,
    High,
}

// Define a function to generate a random password with the given length and complexity
fn generate_password(length: usize, complexity: Complexity) -> Result<String, &'static str> {
    // If the length is zero, return an error
    if length == 0 {
        return Err("Password length must be greater than zero");
    }
    // Define the characters that can be used in the password based on the complexity
    let chars: &[u8] = match complexity {
        Complexity::Low => b"abcdefghijklmnopqrstuvwxyz",
        Complexity::Medium => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        Complexity::High => b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~",
    };
    // Create a random number generator
    let mut rng = rand::thread_rng();
    // Generate a password by selecting random characters from the character set
    Ok((0..length)
        .map(|_| char::from(chars[rng.gen_range(0..chars.len())]))
        .collect())
}
