#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate uuid;
#[macro_use] extern crate lazy_static;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;

mod route;
mod model;
mod webhook;

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    // testing a post request
    let mut wh = model::Webhook::new(&mut core, &client,
                                 "https://google.co.uk".parse().unwrap(), "formatter lol");
    wh.request("meme".to_string());

    rocket::ignite()
        .mount("/", routes![
            route::webhooks_create,
            route::webhooks_view,
            route::webhooks_edit,
            route::webhooks_delete,
            route::webhooks_send
        ])
        .launch();
}


