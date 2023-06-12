use std::env;

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

fn classify_fruit(fruit: &str) -> Result<&str, String> {
    match fruit {
        "용과" | "레몬" | "사과" | "오렌지" => Ok("씨앗이 있는 과일"),
        "바나나" | "귤" | "망고" | "파인애플" => Ok("씨앗이 없는 과일"),
        _ => Err("잘못된 과일입니다.".to_string()),
    }
}

fn get_environment_or_parameter(key: &str) -> String {
    let env_key = env::var("MY_PASSWORD")
        .unwrap_or_else(|_| String::from(key));

    env_key
}

fn main() {
    find_prime_number(5);
    println!("{:?}", ok_or_err(true));

    let fruits = ["귤", "바나나", "사과", "수박"];

    for fruit in &fruits {
        match classify_fruit(fruit) {
            Ok(category) => println!("과일: {}, 분류: {}", fruit, category),
            Err(error) => println!("과일: {}, 오류: {}", fruit, error),
        }
    }

    println!("{:?}", get_environment_or_parameter("p@ssw0rd!"));
}
