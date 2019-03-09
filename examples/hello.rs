use bytes::Bytes;
use chrono::prelude::Local;
use http::response::Builder;
use http::{Request, Response};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
use voyager::http as myhttp;
use voyager::mux::{DefaultHandler, DefaultMux, HandlerFunc};

fn main() -> Result<(), Box<std::error::Error>> {
    let mut m = DefaultMux::new();

    let hello_handler = |w: &mut Builder, r: &Request<()>| -> Response<Bytes> {
        let path = r.uri().path();
        let str_response = format!("in hello handler, path is: {}", path);
        w.body(Bytes::from(str_response)).unwrap()
    };
    let not_found_handler = |w: &mut Builder, r: &Request<()>| -> Response<Bytes> {
        let path = r.uri().path();
        let str_response = format!("page: {} has gone, please go to index page", path);
        w.body(Bytes::from(str_response)).unwrap()
    };
    m.handle(
        "/hello".to_string(),
        DefaultHandler::new(Box::new(hello_handler)),
    );
    m.handle_func("/world".to_string(), Box::new(world_handler));
    m.handle_func(
        "/foo".to_string(),
        logging_middleware(foo("dbconnection".to_string())),
    ); // inject dependence to handler
    m.handle_func("/test.png".to_string(), file_handler());
    m.handle_not_found(DefaultHandler::new(Box::new(not_found_handler)));

    return myhttp::listen_and_serve("127.0.0.1:8080".to_string(), m);
}

fn world_handler(w: &mut Builder, r: &Request<()>) -> Response<Bytes> {
    let path = r.uri().path();
    let str_response = format!("in world handler, path is: {}", path);
    w.body(Bytes::from(str_response)).unwrap()
}

fn foo(db: String) -> HandlerFunc {
    let foo_handler = move |w: &mut Builder, r: &Request<()>| -> Response<Bytes> {
        let path = r.uri().path();
        thread::sleep(time::Duration::from_secs(3));
        let str_response = format!("in foo handler, path is: {}, db is {}", path, db);
        w.body(Bytes::from(str_response)).unwrap()
    };
    Box::new(foo_handler)
}

fn file_handler() -> HandlerFunc {
    let handler = move |w: &mut Builder, _r: &Request<()>| -> Response<Bytes> {
        let mut file = match File::open("./examples/test.png") {
            Err(why) => {
                let s = format!("couldn't open test.png, {}", why.description());
                return w.body(Bytes::from(s)).unwrap();
            }
            Ok(file) => file,
        };

        let mut buf = Vec::new();
        match file.read_to_end(&mut buf) {
            Err(why) => {
                let ss = format!("couldn't read test.png, {}", why.description());
                return w.status(400).body(Bytes::from(ss)).unwrap();
            }
            Ok(_nbytes) => {
                return w.body(Bytes::from(buf)).unwrap();
            }
        }
    };
    Box::new(handler)
}

fn logging_middleware(f: HandlerFunc) -> HandlerFunc {
    let result = Box::new(move |w: &mut Builder, r: &Request<()>| -> Response<Bytes> {
        println!("request start: {}", Local::now());
        let resp = f(w, r);
        println!("request ends: {}", Local::now());
        return resp;
    });

    return result;
}
