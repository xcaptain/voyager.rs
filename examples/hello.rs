use voyager::http;
use voyager::http::handler::Handler;
use voyager::http::mux::Mux;
use voyager::http::request::Request;
use voyager::http::response::ResponseWriter;

fn main() {
    let mut m = Mux::new();

    let hello_handler = |w: &ResponseWriter, r: &Request| {
        let path = r.url.path.clone();
        w.write(format!("in hello handler, path is: {}", path));
    };
    let world_handler = |w: &ResponseWriter, r: &Request| {
        let path = r.url.path.clone();
        w.write(format!("in world handler, path is: {}", path));
    };

    m.handle("/hello".to_string(), Handler::new(Box::new(hello_handler)));
    m.handle("/world".to_string(), Handler::new(Box::new(world_handler)));
    http::listen_and_serve(":8080".to_string(), m);
}
