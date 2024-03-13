use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "error-type")]
    error_type: String,
}

pub fn handle_error(error: ErrorResponse) -> Box<dyn Error> {
    match error.error_type.as_str() {
        "unsupported-code" => "The supplied currency code is not supported.".into(),
        "malformed-request" => "The request was malformed.".into(),
        "invalid-key" => "Invalid API key. Please check your API_KEY.".into(),
        "inactive-account" => "The account is inactive. Please confirm your email address.".into(),
        "quota-reached" => "The request quota has been reached. Please upgrade your plan or wait until the quota resets.".into(),
        _ => format!("An unknown error occurred: {}", error.error_type).into(),
    }
}
