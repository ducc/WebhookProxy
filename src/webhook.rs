use model::{Webhook, WebhookClient};
use uuid::Uuid;
use hyper::{Client, Request, Method, Body, Uri};
use hyper::header::ContentType;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;
use futures::Future;

impl<'a> Webhook<'a> {
    pub fn new(core: &'a Core, client: &'a WebhookClient, uri: Uri, formatter: &'a str) -> Webhook<'a> {
        Webhook{
            core: core,
            client: client,
            id: Uuid::new_v4(),
            uri: uri,
            token: Uuid::new_v4().simple().to_string(),
            formatter: formatter,
        }
    }

    pub fn format(&self, input: &str) -> &str {
        unimplemented!()
    }

    pub fn request(&self, input: &str) {
        unimplemented!()
    }
}