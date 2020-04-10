use crate::{http::HttpHandler, result::CheckoutResult};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Instrument {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub expiry_month: i32,
    pub expiry_year: i32,
    pub scheme: String,
    pub last4: String,
}

#[derive(Serialize)]
pub struct CreateInstrumentData {
    pub token: String,
    #[serde(rename = "type")]
    pub _type: String,
}

pub trait InstrumentHandler: HttpHandler {
    fn create_instrument(&self, data: CreateInstrumentData) -> CheckoutResult<Instrument> {
        let url = format!("{}/instruments", self.base_url());
        let token = self.run_request(
            Method::POST,
            url.parse().expect("Invalid instruments URL"),
            &data,
        )?;

        Ok(token)
    }
}
