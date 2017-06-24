#![allow(unused_variables)]

extern crate rocket;

use rocket_contrib::JSON;
use uuid::{Uuid, ParseError};
use std::str::FromStr;

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
pub fn webhooks_view(id: &str) -> Result<JSON<WebhookView>, ParseError> {
    // header = Authorization: <token>
    // response = {uri, formatter}

    let uuid = match Uuid::from_str(id) {
        Ok(uuid) => uuid,
        Err(e) => return Err(e)
    };

    Ok(JSON(WebhookView {
        uri: "memes".to_string(),
        formatter: "some formatter lol".to_string(),
    }))
}

#[put("/webhooks/<id>", format = "application/json", data = "<data>")]
pub fn webhooks_edit(id: &str, data: JSON<WebhookEdit>) -> Result<(), ParseError> {
    // header = Authorization: <token>
    // body = {uri, formatter}

    let uuid = match Uuid::from_str(id) {
        Ok(uuid) => uuid,
        Err(e) => return Err(e)
    };

    Ok(())
}

#[delete("/webhooks/<id>")]
pub fn webhooks_delete(id: &str) -> Result<(), ParseError> {
    // header = Authorization: <token>

    let uuid = match Uuid::from_str(id) {
        Ok(uuid) => uuid,
        Err(e) => return Err(e)
    };

    Ok(())
}

#[post("/webhooks/<id>", format = "application/json", data = "<data>")]
pub fn webhooks_send(id: &str, data: WebhookSend) -> Result<&str, ParseError> {
    // response = proxied response

    let uuid = match Uuid::from_str(id) {
        Ok(uuid) => uuid,
        Err(e) => return Err(e)
    };

    println!("data: {}", data);
    Ok("the response body from proxied req")
}