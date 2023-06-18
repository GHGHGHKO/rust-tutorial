use std::env;

fn awesome_type() {

    let signed_int : i64 = 0xff_ff_ff_ff_ff; // type i32

    let unsigned_int = 123_u32; // type u32

    let a: u64 = 123; // type u64

    let pi = 3.14159265358979323846264338327950288; // type f64

    let small_pi : f32 = 3.14; // type f32

    let url = "https://httpbin.org/ip"; // type &str

    let tenor_key = env::var("TENOR_API_KEY")
        .unwrap_or_else(|_| String::from("<default_api_key>")); // type String


    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
