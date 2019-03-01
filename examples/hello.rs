use voyager::http;
use voyager::http::handler::Handler;
use voyager::http::mux::Mux;

fn main() {
    let mut m = Mux::new();
    // TODO: handler must implements some traits
    let hello_handler = Handler::new();

    m.handle("/hello".to_string(), hello_handler);
    http::listen_and_serve(":80".to_string(), m);
}
