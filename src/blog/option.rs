#[derive(Debug)]
struct GsPostbox {
    name: String,
    price: u32,
    discount_coupon_price: Option<u32>
}

pub fn coupon_option() {
    let gs_postbox_info = vec![
        GsPostbox { name: "국내택배".to_string(), price: 3200, discount_coupon_price: Some(200) },
        GsPostbox { name: "반값택배".to_string(), price: 1800, discount_coupon_price: Some(100) },
        GsPostbox { name: "반값택배".to_string(), price: 1800, discount_coupon_price: None },
    ];

    for gs_postbox in &gs_postbox_info {
        println!("{} 기본 가격 : {}", gs_postbox.name, gs_postbox.price);
        match gs_postbox.discount_coupon_price {
            Some(discount_price) =>
                println!("{}원 할인 쿠폰, 쿠폰 할인가 : {}",
                         discount_price,
                         gs_postbox.price - discount_price),

            None => println!("쿠폰 할인가 : 쿠폰이 적용되지 않음")
        }
    }
}
