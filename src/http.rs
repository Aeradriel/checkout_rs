use json_errors::json_errors::{JsonError, JsonErrors};
use reqwest::{Client as HttpClient, Method, Url};
use serde::{de::DeserializeOwned, Serialize};

pub trait HttpHandler {
    fn http_client(&self) -> &HttpClient;

    fn secret_key(&self) -> &str;

    fn base_url(&self) -> &str;

    fn run_request<T, U>(&self, method: Method, url: Url, body: &U) -> Result<T, JsonErrors>
    where
        U: Serialize,
        T: DeserializeOwned,
    {
        let client = self.http_client();
        let mut request = match method {
            Method::GET => client.get(url),
            Method::POST => client.post(url),
            Method::PUT => client.put(url),
            Method::PATCH => client.patch(url),
            Method::DELETE => client.delete(url),
            _ => {
                return Err(JsonError::new(501, "Unsupported method").into());
            }
        };

        request = request.json(body);
        request = request
            .header("authorization", self.secret_key())
            .header("content-type", "application/json");

        let mut resp = request.send()?;

        match resp.error_for_status_ref() {
            Ok(_) => resp.json().map_err(|err| err.into()),
            Err(_) => Err(resp.into()),
        }
    }
}
