use crate::{
    http::HttpHandler, instrument::InstrumentHandler, payment::PaymentHandler, token::TokenHandler,
};
use reqwest::Client as HttpClient;

pub struct Client {
    pub base_url: String,
    pub http_client: HttpClient,
    pub secret_key: String,
}

impl Client {
    pub fn new(base_url: &str, secret_key: &str) -> Self {
        Client {
            base_url: base_url.to_owned(),
            http_client: HttpClient::new(),
            secret_key: secret_key.to_owned(),
        }
    }
}

impl HttpHandler for Client {
    fn http_client(&self) -> &HttpClient {
        &self.http_client
    }

    fn secret_key(&self) -> &str {
        &self.secret_key
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }
}

impl InstrumentHandler for Client {}

impl TokenHandler for Client {}

impl PaymentHandler for Client {}
