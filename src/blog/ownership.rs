pub fn ownership_is_difficult() {
    let fib_numbers: Vec<i32> = vec![0, 1, 1, 2, 3, 5, 8]; // 1. 여기서부터 `fib_numbers`가 유효합니다.

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

pub fn ownership_is_fun() {
    let mut fib_numbers: Vec<i32> = vec![0, 1, 1, 2, 3, 5, 8]; // 1. 여기서부터 `fib_numbers`가 유효합니다.

    add_vector_by_reference(&mut fib_numbers);           // 2. 가변 참조자를 통해 `add_vector_by_reference` 함수를 호출합니다.
    print_vector_by_reference(&fib_numbers);             // 5. 참조자를 통해 `print_vector_by_reference` 함수를 호출합니다.

    fib_numbers.push(34);                                 // 9. `fib_numbers` Ownership 이 유효하기 때문에
                                                                //    쓰기, 읽기가 정상적으로 됩니다.
    println!("{:?}", fib_numbers);
}                                           // 10. `fib_numbers`변수는 스코프 밖으로 벗어났기 때문에
                                            //     더 이상 유효하지 않습니다. 메모리가 반환됩니다.

fn add_vector_by_reference(vector: &mut Vec<i32>) {             // 3. 가변 참조자를 받아 내용을 수정합니다.
    vector.push(13);
    vector.push(21);
}                                           // 4. `vector`는 가변 참조자로 받기 때문에
                                            //    스코프 밖으로 넘어가도 아무런 일이 발생하지 않습니다.
                                            //    즉, `fib_numbers` Ownership 은 유효합니다.

fn print_vector_by_reference(vector: &Vec<i32>) {               // 6. 참조자를 받아 `vector`를 출력합니다.
    println!("{:?}", vector);
}                                           // 7. `vector`는 참조자로 받기 때문에
                                            //    이 또한 `fib_numbers` Ownership 이 유효합니다.

#[derive(Debug)]
struct GooglePlayApplication {
    name: String,
    download_count: i32
}

pub fn data_race() {
    let mut applications = vec![
        GooglePlayApplication { name: "우리동네GS".to_string(), download_count: 5_000_000 },
        GooglePlayApplication { name: "GSSHOP".to_string(), download_count: 10_000_000 }
    ];

    application_information(&applications);
    download_increase(&mut applications);
    application_information(&applications);

}

fn download_increase(applications: &mut Vec<GooglePlayApplication>) {
    for application in applications {
        if application.name == "우리동네GS" {
            application.download_count *= 10;
        }
    }
}

fn application_information(applications: &Vec<GooglePlayApplication>) {
    for application in applications {
        println!("{:?}", application);
    }
}
