use std::env;
use vaultwarden::from_db::FromDb;
use serde::{Serialize, Deserialize};
use crate::weather::weather::Weather;
use std::fs::File;
use std::io::{Error, Read};
use ownership::ownership::fail_move_ownership;
use integer_example::memory_safe::struct_is_memory_safe;
use shadowing::shadowed_example::double_shadow_example;
use mutable_references::mutable_references::mutable_references;

mod dto;
mod client;
mod vaultwarden;
mod type_inference;
mod weather;
mod ownership;
mod integer_example;
mod shadowing;
mod mutable_references;

fn find_prime_number(number: u32) {
    match number {
        1 => println!("It's one!"),

        2 | 3 | 5 => println!("It's a prime number!"),

        4..=10 => println!("It's between 4 and 10 (inclusive)!"),

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

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // 계좌 생성 메서드
    fn new(owner: String, balance: f64) -> Self {
        BankAccount {
            owner,
            balance,
        }
    }

    // 입금 메서드
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("{}님의 계좌에 {}원이 입금되었습니다.", self.owner, amount);
    }

    // 출금 메서드
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= self.balance {
            self.balance -= amount;
            println!("{}님의 계좌에서 {}원이 출금되었습니다.", self.owner, amount);
            Ok(())
        } else {
            Err("잔액이 부족합니다.".to_string())
        }
    }

    // 잔액 조회 메서드
    fn check_balance(&self) {
        println!("{}님의 현재 잔액은 {}원입니다.", self.owner, self.balance);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

impl FromDb for Person {
    type Output = Self;

    fn from_db(self) -> Self::Output {
        self
    }
}

fn read_receipt() -> Result<(), Error> {
    let file_name = "receipt.txt";

    let mut file = File::open(file_name)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;
    print!("{}", contents);

    Ok(())
}

fn variable_examples() {
    let unsigned_int = 123_u32;

    let a: u64 = 123;

    let pi = 3.14159265358979323846264338327950288;

    let small_pi : f32 = 3.14;

    let url = "https://httpbin.org/ip";

    let tenor_key = env::var("TENOR_API_KEY")
        .unwrap_or_else(|_| String::from("<default_api_key>"));

    let signed_int: i64 = 0xff_ff_ff_ff_ff; // ???
}

fn main() {
    find_prime_number(7);
    println!("{:?}", ok_or_err(true));

    let fruits = ["귤", "바나나", "사과", "수박"];

    for fruit in &fruits {
        match classify_fruit(fruit) {
            Ok(category) => println!("과일: {}, 분류: {}", fruit, category),
            Err(error) => println!("과일: {}, 오류: {}", fruit, error),
        }
    }

    println!("{:?}", get_environment_or_parameter("p@ssw0rd!"));

    let mut account = BankAccount::new("고민수".to_string(), 1000.0);
    account.check_balance(); // 현재 잔액 조회: 1000원
    account.deposit(500.0); // 500원 입금
    account.check_balance(); // 현재 잔액 조회: 1500원
    account.withdraw(200.0).unwrap(); // 200원 출금
    account.check_balance(); // 현재 잔액 조회: 1300원
    account.withdraw(2000.0)
        .unwrap_or_else(|error| println!("에러: {}", error)); // 잔액 부족으로 인한 출금 실패, 에러 출력

    // FromDb 예제
    let persons: Vec<Person> = vec![
        Person { name: "John".to_string(), age: 30 },
        Person { name: "Jane".to_string(), age: 25 },
        Person { name: "Mike".to_string(), age: 40 },
    ];

    let persons_output: Vec<Person> = FromDb::from_db(persons);
    println!("{:?}", persons_output);

    let person: Option<Person> = Some(Person { name: "Alice".to_string(), age: 35 });

    let person_output: Option<Person> = FromDb::from_db(person);
    println!("{:?}", person_output);

    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");

    println!("{:?}", Weather::weather_forecasting_stone(&Weather::Earthquake));

    fail_move_ownership();

    struct_is_memory_safe();

    double_shadow_example();

    mutable_references();
}
