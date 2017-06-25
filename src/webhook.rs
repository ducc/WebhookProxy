use model::{Webhook, WebhookClient};
use uuid::Uuid;
use hyper::{Request, Method, Uri};
use tokio_core::reactor::Core;
use futures::Future;

impl<'a> Webhook<'a> {
    pub fn new(core: &'a mut Core, client: &'a WebhookClient, uri: Uri, formatter: &'a str) -> Webhook<'a> {
        Webhook{
            core: core,
            client: client,
            id: Uuid::new_v4(),
            uri: uri,
            token: Uuid::new_v4().simple().to_string(),
            formatter: formatter,
        }
    }

    #[allow(dead_code, unused_variables)]
    pub fn format(&self, input: &str) -> &str {
        unimplemented!()
    }

    // todo callback
    pub fn request(&mut self, input: String) {
        let mut req = Request::new(Method::Post, self.uri.to_owned());
        req.set_body(input);
        let task = self.client.request(req).map(|res| {
            println!("status: {}", res.status());
        });
        let mut core = &mut self.core;
        core.run(task).expect("error occurred sending request");
    }
}