pub fn cafe25() {
    let coffee = true;
    let water = true;
    let electricity = true;
    let ice = true;

    let result = match (coffee, water, electricity, ice) {
        (true, true, true, false) => "뜨거운 커피만 마실 수 있습니다..",
        (true, true, true, true) => "아이스 커피를 마실 수 있습니다!!!",
        (true, true, false, _) => "커피는 준비가 되었으나, 전기가 안들어옵니다!",
        (false, true, _, _) => "물이라도 한 잔 드세요.",
        (_, _, _, _) => "헉",
    };

    println!("{result}")
}
