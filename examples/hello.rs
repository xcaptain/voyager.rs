use http::response::Builder;
use http::{Request, Response};
use std::sync::Arc;
use voyager::http as myhttp;
use voyager::http::handler::Handler;
use voyager::http::mux::Mux;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut m = Mux::new();

    let hello_handler = |w: &mut Builder, r: &Request<()>| -> Response<String> {
        let path = r.uri().path();
        w.body(format!("in hello handler, path is: {}", path))
            .unwrap()
    };
    let world_handler = |w: &mut Builder, r: &Request<()>| -> Response<String> {
        let path = r.uri().path();
        w.body(format!("in world handler, path is: {}", path))
            .unwrap()
    };

    m.handle("/hello".to_string(), Handler::new(Arc::new(hello_handler)));
    m.handle("/world".to_string(), Handler::new(Arc::new(world_handler)));
    return myhttp::listen_and_serve("127.0.0.1:8080".to_string(), m);
}
