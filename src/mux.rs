use bytes::Bytes;
use http::response::Builder;
use http::{Request, Response, StatusCode};
use std::collections::HashMap;

pub type HandlerFunc = Box<dyn Fn(&mut Builder, &Request<()>) -> Response<Bytes> + Sync + Send>;
/// this trait defines how to serve_http
/// to use across multiple threads, this traits must implement Sync and Send
/// because this trait must live longer then w, so add a `static lifetime
pub trait Handler: Sync + Send + 'static {
    fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<Bytes>;
}

impl Handler for HandlerFunc {
    fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<Bytes> {
        (self)(w, r)
    }
}

// TODO: figure out use Box<Handler> or Box<dyn Handler>?
/// the default mux for the framework, can be replaced as if the new object
/// implemented the Handler trait, this default implementation will follow
/// `go-chi/chi`'s api design
pub struct DefaultMux {
    m: HashMap<String, MuxEntry>,
    not_found_handler: Box<Handler>,
}

struct MuxEntry {
    h: Box<Handler>,
}

impl DefaultMux {
    pub fn new() -> Self {
        let notfound: HandlerFunc =
            Box::new(|w: &mut Builder, r: &Request<()>| -> Response<Bytes> {
                let path = r.uri().path();
                let str_response = format!("404 not found, path is: {}", path);
                w.status(StatusCode::NOT_FOUND)
                    .body(Bytes::from(str_response))
                    .unwrap()
            });
        DefaultMux {
            m: HashMap::new(),
            not_found_handler: Box::new(notfound),
        }
    }

    /// register http handler
    pub fn handle_func(&mut self, pattern: String, handler: HandlerFunc) {
        let entry = MuxEntry {
            h: Box::new(handler),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    /// what the fuck, why rust doesn't support `handler.serve_http` as a closure
    pub fn handle(&mut self, pattern: String, handler: Box<Handler>) {
        let entry = MuxEntry { h: handler };
        self.m.entry(pattern).or_insert(entry);
    }

    /// register custom not found handler
    pub fn handle_not_found(&mut self, handler: HandlerFunc) {
        self.not_found_handler = Box::new(handler);
    }

    /// get handler from mux
    pub fn handler(&self, r: &Request<()>) -> Option<&Handler> {
        let path = r.uri().path().to_owned();
        if let Some(entry) = self.m.get(&path) {
            return Some(&*entry.h);
        }
        None
    }
}

impl Default for DefaultMux {
    fn default() -> Self {
        Self::new()
    }
}

impl Handler for DefaultMux {
    fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<Bytes> {
        // match router
        if let Some(handler) = self.handler(&r) {
            return handler.serve_http(w, r);
        }
        self.not_found_handler.serve_http(w, r)
    }
}
