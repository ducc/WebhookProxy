#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate serde_json;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

mod route;
mod model;

fn main() {
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


