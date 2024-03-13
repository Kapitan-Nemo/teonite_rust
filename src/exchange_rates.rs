use crate::error_handling::handle_error;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use ttl_cache::TtlCache;

#[derive(Debug, Deserialize)]
struct ExchangeResponse {
    conversion_rates: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Response {
    Ok(ExchangeResponse),
    Err(crate::error_handling::ErrorResponse),
}

lazy_static! {
    static ref CACHE: Mutex<TtlCache<String, f64>> = Mutex::new(TtlCache::new(100));
}

pub async fn fetch_exchange_rate(
    base: &str,
    target: &str,
    api_key: &str,
) -> Result<f64, Box<dyn std::error::Error>> {
    if base.parse::<f64>().is_ok() || target.parse::<f64>().is_ok() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Base and target currency must be non-numeric strings.",
        )));
    }
    let key = format!("{}-{}", base, target);
    let mut cache = CACHE.lock().unwrap();

    if let Some(rate) = cache.get(&key) {
        return Ok(*rate);
    }

    let url = format!(
        "https://v6.exchangerate-api.com/v6/{}/latest/{}",
        api_key, base
    );
    let response: Response = reqwest::get(&url).await?.json().await?;

    match response {
        Response::Ok(data) => {
            let rate = data
                .conversion_rates
                .get(target)
                .ok_or("Invalid currency code.")?;
            cache.insert(key, *rate, Duration::from_secs(3600));
            Ok(*rate)
        }
        Response::Err(error) => Err(handle_error(error)),
    }
}
