use http::response::Builder;
use http::{Request, Response};
use voyager::http as myhttp;
use voyager::mux::{DefaultHandler, DefaultMux, HandlerFunc};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut m = DefaultMux::new();

    let hello_handler = |w: &mut Builder, r: &Request<()>| -> Response<String> {
        let path = r.uri().path();
        w.body(format!("in hello handler, path is: {}", path))
            .unwrap()
    };
    let not_found_handler = |w: &mut Builder, r: &Request<()>| -> Response<String> {
        let path = r.uri().path();
        w.body(format!("page: {} has gone, please go to index page", path))
            .unwrap()
    };
    m.handle(
        "/hello".to_string(),
        DefaultHandler::new(Box::new(hello_handler)),
    );
    m.handle_func("/world".to_string(), Box::new(world_handler));
    m.handle_func("/foo".to_string(), foo("dbconnection".to_string())); // inject dependence to handler
    m.handle_not_found(DefaultHandler::new(Box::new(not_found_handler)));

    return myhttp::listen_and_serve("127.0.0.1:8080".to_string(), m);
}

fn world_handler(w: &mut Builder, r: &Request<()>) -> Response<String> {
    let path = r.uri().path();
    w.body(format!("in world handler, path is: {}", path))
        .unwrap()
}

fn foo(db: String) -> HandlerFunc {
    let foo_handler = move |w: &mut Builder, r: &Request<()>| -> Response<String> {
        let path = r.uri().path();
        w.body(format!("in foo handler, path is: {}, db is {}", path, db))
            .unwrap()
    };
    Box::new(foo_handler)
}
