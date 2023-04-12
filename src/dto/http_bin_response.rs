use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HttpBinResponse {
    pub origin: String,
}
