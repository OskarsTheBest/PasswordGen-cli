
use rand::Rng;


fn main() {
    let password = generate_password(12);
    println!("Generated Password: {}", password);
}


fn generate_password(length: usize) -> String {
    let chars: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                        abcdefghijklmnopqrstuvwxyz\
                        0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| char::from(chars[rng.gen_range(0..chars.len())]))
        .collect()
}