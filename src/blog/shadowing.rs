#[derive(Debug)]
struct GsPostbox {
    name: String,
    price: u32
}

pub fn vector_shadowing() {
    let mut products_vector = Vec::new();

    products_vector.push(GsPostbox { name: "국내택배".to_string(), price: 3200});
    products_vector.push(GsPostbox { name: "반값택배".to_string(), price: 1800});

    let products_vector = products_vector;

    println!("{:?}", products_vector);
}
