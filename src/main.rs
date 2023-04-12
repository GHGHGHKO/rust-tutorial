mod dto;
mod client;

use client::http_bin_client::http_bin_client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    http_bin_client()
}
