pub fn double_shadow_example() {
    let mut x = 10;
    let y = 0;

    if x > 5 {
        x = 50; // mutation
        let y = y + 5; // shadow

        println!("{y}")
    }

    println!("{x}, {y}")
}
