#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

mod webhook;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            webhooks_create,
            webhooks_view,
            webhooks_edit,
            webhooks_delete,
            webhooks_send
        ])
        .launch();
}

#[post("/webhooks", format = "application/json")]
fn webhooks_create() {
    // body = {uri, formatter}
    // response = {proxied webhook uri, token}
}

#[get("/webhooks/<id>")]
fn webhooks_view(id: &str) {
    // header = Authorization: <token>
    // response = {uri, formatter}
}

#[put("/webhooks/<id>", format = "application/json")]
fn webhooks_edit(id: &str) {
    // header = Authorization: <token>
    // body = {uri, formatter}
}

#[delete("/webhooks/<id>", format = "application/json")]
fn webhooks_delete(id: &str) {
    // header = Authorization: <token>
}

#[post("/webhooks/<id>", format = "application/json")]
fn webhooks_send(id: &str) {
    // response = proxied response
}
