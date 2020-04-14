use crate::{http::HttpHandler, result::CheckoutResult};
use reqwest::Method;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Payment {
    pub id: String,
    pub amount: i32,
    pub currency: String,
    pub approved: bool,
    pub status: String,
    pub source: PaymentSource,
    pub reference: String,
}

#[derive(Deserialize, Debug)]
pub struct PaymentSource {
    pub id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub last4: String,
}

#[derive(Serialize, Debug)]
pub struct PaymentSourceData {
    #[serde(rename = "type")]
    pub _type: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Customer {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct CreatePaymentData {
    pub source: PaymentSourceData,
    pub currency: String,
    pub amount: i32,
    pub reference: String,
}

pub trait PaymentHandler: HttpHandler {
    fn get_payment_details(&self, id: &str) -> CheckoutResult<Payment> {
        let url = format!("{}/payments/{}", self.base_url(), id);
        let payment =
            self.run_request(Method::GET, url.parse().expect("Invalid payments URL"), &())?;

        Ok(payment)
    }

    fn create_payment(&self, data: CreatePaymentData) -> CheckoutResult<Payment> {
        let url = format!("{}/payments", self.base_url());
        let payment = self.run_request(
            Method::POST,
            url.parse().expect("Invalid payments URL"),
            &data,
        )?;

        Ok(payment)
    }
}
