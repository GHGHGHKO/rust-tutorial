pub fn ownership_is_difficult() {
    let mut fib_numbers: Vec<i32> = vec![0, 1, 1, 2, 3, 5, 8]; // 1. 여기서부터 `fib_numbers`가 유효합니다.

    let mut fib_numbers = add_vector(fib_numbers);              // 2. `add_vector` 함수를 호출합니다.
    // 6. `fib_numbers`변수를 `shadowing`하여 `add_vector`값을 입력합니다.
    //    `fib_numbers`변수가 여기서부터 다시 유효합니다.

    print_vector(fib_numbers);                                  // 7. `print_vector`함수를 호출합니다.

    // println!("{:?}", fib_numbers); 오류 발생!
}

fn print_vector(vector: Vec<i32>) {
    println!("{:?}", vector)                                    // 8. `vector`를 출력합니다
}                                                               // 9. 여기서부터 `vector`의 `ownership`이 사라집니다.

fn add_vector(mut vector: Vec<i32>) -> Vec<i32> {               // 3. Vec<i32>를 반환하는 `add_vector`함수를 정의합니다.
    vector.push(13);
    vector.push(21);
    // 4. 가변 `parameter`인 `vector`에 13, 21을 넣습니다.
    vector                                                      // 5. `vector`를 반환합니다.
}
