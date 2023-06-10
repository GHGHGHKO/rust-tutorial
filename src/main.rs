mod dto;
mod client;

use client::http_bin_client::http_bin_client;
use client::tenor_client::tenor_client;

fn find_prime_number() {
    let number = 7;

    match number {
        1 => println!("It's one!"),

        2 | 3 | 5 => println!("It's a prime number!"),

        4..=10 => println!("It's between 4 and 10 (inclusive)!"),

        n if n % 2 == 0 => println!("It's an even number!"),

        _ => println!("It doesn't match any condition."),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    find_prime_number();
    let _ = http_bin_client();
    tenor_client()
}
