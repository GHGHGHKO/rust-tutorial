use std::env;
use crate::dto::tenor_response::TenorResults;
use reqwest::blocking::Client;

pub fn tenor_client() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let tenor_key = env::var("TENOR_API_KEY")
        .unwrap_or_else(|_| String::from("<default_api_key>"));
    let url =
        format!("https://tenor.googleapis.com/v2/search?q={}&key={}&limit={}&random=true",
                "excited", tenor_key, 3);

    let response = client.get(url).send()?;
    let response_text = response.text()?;
    println!("Tenor API response text: \n{}", response_text);

    let tenor_response_body: TenorResults = serde_json::from_str(&response_text).unwrap();
    for result in &tenor_response_body.results {
        println!("API url: {}", result.media_formats.tiny_gif.url);
    }
    Ok(())
}
