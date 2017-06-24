extern crate rocket;

use std::io::Read;
use std::fmt;
use rocket::{Data, Request};
use rocket::data::{self, FromData};
use rocket::http::Status;
use uuid::Uuid;
use hyper::{Client, Body, Uri};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

pub type WebhookClient = Client<HttpsConnector<HttpConnector>, Body>;

#[allow(dead_code)]
pub struct Webhook<'a> {
    pub core: &'a Core,
    pub client: &'a WebhookClient,
    pub id: Uuid,
    pub uri: Uri,
    pub token: String,
    pub formatter: &'a str,
}

#[derive(Deserialize)]
pub struct WebhookCreate {
    pub uri: String,
    pub formatter: String,
}

#[derive(Serialize)]
pub struct WebhookCreated {
    pub id: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct WebhookView {
    pub uri: String,
    pub formatter: String,
}

#[derive(Deserialize)]
pub struct WebhookEdit {
    pub uri: String,
    pub formatter: String
}

pub struct WebhookSend {
    value: String
}

impl FromData for WebhookSend {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        let mut string = String::new();
        if let Err(e) = data.open().read_to_string(&mut string) {
            return rocket::Outcome::Failure((Status::InternalServerError, format!("{:?}", e)));
        }
        rocket::Outcome::Success(WebhookSend{
            value: string,
        })
    }
}

impl fmt::Display for WebhookSend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}