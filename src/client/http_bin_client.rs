use crate::dto::http_bin_response::HttpBinResponse;
use reqwest::blocking::Client;

pub fn http_bin_client() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = "https://httpbin.org/ip";
    let response = client.get(url).send()?;
    let response_text = response.text()?;
    println!("API response text: \n{}", response_text);

    let http_bin_response_body: HttpBinResponse = serde_json::from_str(&response_text).unwrap();
    println!("IP address: {}", http_bin_response_body.origin);
    Ok(())
}
