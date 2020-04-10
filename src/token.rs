use crate::{http::HttpHandler, result::CheckoutResult};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Token {
    #[serde(rename = "type")]
    pub _type: String,
    pub token: String,
    pub expiry_month: i32,
    pub expiry_year: i32,
    pub scheme: String,
    pub last4: String,
}

#[derive(Serialize)]
pub struct CreateTokenData {
    #[serde(rename = "type")]
    pub _type: String,
    pub number: String,
    pub expiry_month: i32,
    pub expiry_year: i32,
    pub name: String,
    pub cvv: String,
}

pub trait TokenHandler: HttpHandler {
    fn create_token(&self, data: CreateTokenData) -> CheckoutResult<Token> {
        let url = format!("{}/tokens", self.base_url());
        let token =
            self.run_request(Method::POST, url.parse().expect("Invalid token URL"), &data)?;

        Ok(token)
    }
}
