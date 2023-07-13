use sysinfo::{System, SystemExt};

#[derive(Debug)]
pub struct ErrorMessage {
    message: String,
}

pub fn get_os_info() -> Result<String, ErrorMessage> {
    let mut sys = System::new_all();

    sys.refresh_all();

    match sys.os_version() {
        Some(version) => Ok(version),
        None => Err(ErrorMessage { message: "OS 정보를 불러오지 못했습니다.".to_string() })
    }
}

pub fn print_os_info(os: Result<String, ErrorMessage>) {
    println!("os info : {:?}", os);
}
