#![allow(unused_variables)]

extern crate rocket;

use rocket_contrib::JSON;
use uuid::Uuid;

use model::{WebhookCreate, WebhookCreated, WebhookView, WebhookEdit, WebhookSend};

#[post("/webhooks", format = "application/json", data = "<data>")]
pub fn webhooks_create(data: JSON<WebhookCreate>) -> JSON<WebhookCreated> {
    // body = {uri, formatter}
    // response = {proxied webhook uri, token}

    JSON(WebhookCreated {
        id: Uuid::new_v4().hyphenated().to_string(),
        token: "meme".to_string(),
    })
}

#[get("/webhooks/<id>")]
pub fn webhooks_view(id: &str) -> JSON<WebhookView> {
    // header = Authorization: <token>
    // response = {uri, formatter}

    JSON(WebhookView {
        uri: "memes".to_string(),
        formatter: "some formatter lol".to_string(),
    })
}

#[put("/webhooks/<id>", format = "application/json", data = "<data>")]
pub fn webhooks_edit(id: &str, data: JSON<WebhookEdit>) {
    // header = Authorization: <token>
    // body = {uri, formatter}
}

#[delete("/webhooks/<id>")]
pub fn webhooks_delete(id: &str) {
    // header = Authorization: <token>
}

#[post("/webhooks/<id>", format = "application/json", data = "<data>")]
pub fn webhooks_send(id: &str, data: WebhookSend) {
    // response = proxied response
    println!("data: {}", data);
}