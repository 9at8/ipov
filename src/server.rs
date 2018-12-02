extern crate iron;

use iron::prelude::*;
use iron::modifiers::Header;
use iron::{
    headers::ContentType,
    mime::{Mime, SubLevel, TopLevel},
};

pub struct Server {}

impl Server {
    pub fn start(&self) {
        fn play(_: &mut Request) -> IronResult<Response> {
            Ok(Response::with((
                iron::status::Ok,
                String::from(Server::create_response_string()),
                Header(ContentType(Mime(
                    TopLevel::Text,
                    SubLevel::Xml,
                    vec![],
                ))),
            )))
        }

        Iron::new(play).http("localhost:3000");
    }

    fn create_response_string() -> String {
        String::from(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<Response>
    <Play loop=\"1\">https://2e3afb91.ngrok.io/encoded.wav</Play>
</Response>
"
        )
    }
}
