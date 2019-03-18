use bytes::Bytes;
use chrono::prelude::Local;
use http::response::Builder;
use http::{Request, Response, StatusCode};
use std::{thread, time};
use voyager::fs::FileServer;
use voyager::http as myhttp;
use voyager::http::{strip_prefix, Handler, HandlerFunc};
use voyager::mux::DefaultServeMux;
use voyager::server::DefaultServer;

fn main() -> Result<(), Box<std::error::Error>> {
    let mut m = DefaultServeMux::new();

    let not_found_handler: HandlerFunc =
        Box::new(|w: &mut Builder, r: Request<()>| -> Response<Bytes> {
            let path = r.uri().path();
            let str_response = format!("page: {} has gone, please go to index page", path);
            w.status(StatusCode::NOT_FOUND)
                .body(Bytes::from(str_response))
                .unwrap()
        });
    m.handle("/hello".to_string(), Box::new(HelloHandler::new()));
    m.handle("/world".to_string(), Box::new(WorldHandler::new()));
    m.handle_func(
        "/foo".to_string(),
        logging_middleware(foo("dbconnection".to_string())),
    ); // inject dependence to handler
    let file_server = FileServer::new("./examples/static".to_string());
    m.handle(
        "/static/test.png".to_string(),
        strip_prefix("/static/".to_string(), Box::new(file_server)),
    );
    m.handle_not_found(not_found_handler);

    return myhttp::listen_and_serve(DefaultServer::new(
        "127.0.0.1:8080".to_string(),
        Box::new(m),
    ));
}

fn foo(db: String) -> HandlerFunc {
    let foo_handler = move |w: &mut Builder, r: Request<()>| -> Response<Bytes> {
        let path = r.uri().path();
        thread::sleep(time::Duration::from_secs(2));
        let str_response = format!("in foo handler, path is: {}, db is {}", path, db);
        w.body(Bytes::from(str_response)).unwrap()
    };
    Box::new(foo_handler)
}

fn logging_middleware(f: HandlerFunc) -> HandlerFunc {
    let result = Box::new(move |w: &mut Builder, r: Request<()>| -> Response<Bytes> {
        println!("request start: {}", Local::now());
        let resp = f(w, r);
        println!("request ends: {}", Local::now());
        return resp;
    });

    return result;
}

struct HelloHandler();
impl HelloHandler {
    pub fn new() -> Self {
        HelloHandler()
    }
}
impl Handler for HelloHandler {
    fn serve_http(&self, w: &mut Builder, r: Request<()>) -> Response<Bytes> {
        let path = r.uri().path();
        let str_response = format!("in hello handler, path is: {}", path);
        w.body(Bytes::from(str_response)).unwrap()
    }
}

struct WorldHandler {}
impl WorldHandler {
    pub fn new() -> Self {
        WorldHandler {}
    }
}
impl Handler for WorldHandler {
    fn serve_http(&self, w: &mut Builder, r: Request<()>) -> Response<Bytes> {
        let path = r.uri().path();
        let str_response = format!("in world handler, path is: {}", path);
        w.body(Bytes::from(str_response)).unwrap()
    }
}
