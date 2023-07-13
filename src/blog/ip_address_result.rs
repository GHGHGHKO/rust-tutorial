use std::time::Duration;
use reqwest::blocking::Client;

pub fn get_ip_address() -> Result<Option<String>, reqwest::Error> { // 1. 이 함수의 결과는
                                                                    //    성공 할 경우 `Option<String>` 반환합니다.
                                                                    //    실패 할 경우 `reqwest::Error` 반환합니다.
    let client = Client::builder()
        .timeout(Duration::from_secs(30))            // 2. timeout 시간을 1초로 설정 하였습니다.
        .build();

    let url = "https://httpbin.org/ip";

    let response = client?  // 3. `client` 변수를 호출 하였습니다.
                                         //    `?`을 사용 하여 오류가 발생할 경우 `reqwest::Error` 반환합니다.
        .get(url)
        .send()?    // 4. `send()` 함수를 호출합니다.
                    //    `?`을 사용 하여 오류가 발생할 경우 `reqwest::Error` 반환합니다.
        .text();

    match response {
        Ok(info) => Ok(Some(info)),     // 5. `client`, `send`에서 오류가 발생하지 않으면
                                               //    IP 정보를 반환합니다.
        Err(_) => Ok(None),     // 6. `client`, `send`에서 오류가 발생하면
                                //    `None`을 반환합니다. (실패했으니 reqwest::Error가 자동으로 반환됩니다.)

    }
}

pub fn print_ip_address(ip: &Result<Option<String>, reqwest::Error>) {
    println!("ip address is : {:?}", ip);
}
