use http::response::Builder;
use http::{Request, Response, StatusCode};
use std::collections::HashMap;

pub type HandlerFunc = Box<dyn Fn(&mut Builder, &Request<()>) -> Response<String> + Sync + Send>;
/// this trait defines how to serve_http
/// to use across multiple threads, this traits must implement Sync and Send
/// because this trait must live longer then w, so add a `static lifetime
pub trait Handler: Sync + Send + 'static {
    fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<String>;
}

impl Handler for HandlerFunc {
    fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<String> {
        (self)(w, r)
    }
}

/// default http handler struct used by the default mux implementation
pub struct DefaultHandler(HandlerFunc);

impl Handler for DefaultHandler {
    fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<String> {
        (self.0)(w, r)
    }
}

impl DefaultHandler {
    pub fn new(f: HandlerFunc) -> Self {
        DefaultHandler(f)
    }
}

/// the default mux for the framework, can be replaced as if the new object
/// implemented the Handler trait, this default implementation will follow
/// `gorilla/mux`'s api design
pub struct DefaultMux {
    m: HashMap<String, MuxEntry>,
    not_found_handler: DefaultHandler,
}

struct MuxEntry {
    h: DefaultHandler,
    pattern: String,
}

impl DefaultMux {
    pub fn new() -> Self {
        let default_not_found_handler = DefaultHandler::new(Box::new(
            |w: &mut Builder, r: &Request<()>| -> Response<String> {
                let path = r.uri().path();
                w.status(StatusCode::NOT_FOUND)
                    .body(format!("404 not found, path is: {}", path))
                    .unwrap()
            },
        ));
        DefaultMux {
            m: HashMap::new(),
            not_found_handler: default_not_found_handler,
        }
    }

    /// register router pattern
    pub fn handle(&mut self, pattern: String, handler: DefaultHandler) {
        let entry = MuxEntry {
            h: handler,
            pattern: pattern.clone(),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    /// just a piece of syntax sugar of `handle`
    pub fn handle_func(&mut self, pattern: String, handler: HandlerFunc) {
        let entry = MuxEntry {
            h: DefaultHandler::new(handler),
            pattern: pattern.clone(),
        };
        self.m.entry(pattern).or_insert(entry);
    }

    /// register custom not found handler
    pub fn handle_not_found(&mut self, handler: DefaultHandler) {
        self.not_found_handler = handler;
    }

    /// get handler from mux
    pub fn handler(&self, r: &Request<()>) -> Option<&DefaultHandler> {
        let path = r.uri().path().to_owned();
        if let Some(entry) = self.m.get(&path) {
            return Some(&entry.h);
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
    fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<String> {
        // match router
        if let Some(handler) = self.handler(&r) {
            return handler.serve_http(w, r);
        }
        self.not_found_handler.serve_http(w, r)
    }
}
