use crate::error_handling::handle_error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SymbolsResponse {
    pub supported_codes: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Ok(SymbolsResponse),
    Err(crate::error_handling::ErrorResponse),
}

pub async fn fetch_symbols(api_key: &str) -> Result<SymbolsResponse, Box<dyn std::error::Error>> {
    let url = format!("https://v6.exchangerate-api.com/v6/{}/codes", api_key);
    let response: Response = reqwest::get(&url).await?.json().await?;

    match response {
        Response::Ok(data) => Ok(data),
        Response::Err(error) => Err(handle_error(error)),
    }
}
