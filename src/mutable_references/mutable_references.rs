pub fn mutable_references() {
    let mut vector: Vec<i32> = Vec::new();

    let vector1 = &mut vector;
    // let vector2 = &vector; -> error

    vector1.push(500);
    // println!("{:?}", vector2); -> error
    println!("{:?}", vector1);
}
