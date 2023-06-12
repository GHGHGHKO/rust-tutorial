mod dto;
mod client;

fn find_prime_number(number: u32) {
    match number {
        1 => println!("It's one!"),

        2 | 3 | 5 => println!("It's a prime number!"),

        4..=10 => println!("It's between 4 and 10 (inclusive)!"),

        n if n % 2 == 0 => println!("It's an even number!"),

        _ => println!("It doesn't match any condition."),
    }
}

fn ok_or_err(is_success: bool) -> Result<String, String> {
    match is_success {
        true => {
            Ok(String::from("success!"))
        }
        false => {
            Err(String::from("error!"))
        }
    }
}

fn main() {
    find_prime_number(5);
    println!("{:?}", ok_or_err(true));
}
