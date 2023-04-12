mod dto;
mod client;

use client::http_bin_client::http_bin_client;
use client::tenor_client::tenor_client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = http_bin_client();
    tenor_client()
}
