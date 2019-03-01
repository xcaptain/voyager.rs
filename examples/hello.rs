use voyager::http;
use voyager::http::handler::Handler;
use voyager::http::mux::Mux;
use voyager::http::request::Request;
use voyager::http::response::ResponseWriter;

fn main() {
    let mut m = Mux::new();
    // TODO: handler must implements some traits
    let hello_handler = Handler::new(hello_handler);

    m.handle("/hello".to_string(), hello_handler);
    http::listen_and_serve(":80".to_string(), m);
}

fn hello_handler(_w: &ResponseWriter, r: &Request) {
    let path = r.url.path.clone();
    println!("in hello handler, path is: {}", path);
}
