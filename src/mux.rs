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

/// the default mux for the framework, can be replaced as if the new object
/// implemented the Handler trait, this default implementation will follow
/// `go-chi/chi`'s api design
pub struct DefaultMux {
    m: HashMap<String, MuxEntry>,
    not_found_handler: HandlerFunc,
}

struct MuxEntry {
    h: HandlerFunc,
}

impl DefaultMux {
    pub fn new() -> Self {
        let notfound = |w: &mut Builder, r: &Request<()>| -> Response<Bytes> {
            let path = r.uri().path();
            let str_response = format!("404 not found, path is: {}", path.clone());
            w.status(StatusCode::NOT_FOUND)
                .body(Bytes::from(str_response))
                .unwrap()
        };
        DefaultMux {
            m: HashMap::new(),
            not_found_handler: Box::new(notfound),
        }
    }

    /// register http handler
    pub fn handle_func(&mut self, pattern: String, handler: HandlerFunc) {
        let entry = MuxEntry {
            h: handler,
        };
        self.m.entry(pattern).or_insert(entry);
    }

    /// what the fuck, why rust doesn't support `handler.serve_http` as a closure
    // pub fn handle(&mut self, pattern: String, handler: impl Handler) {
    //     let new_handler: HandlerFunc = Box::new(handler.serve_http);
    //     self.handle_func(pattern, new_handler);
    // }

    /// register custom not found handler
    pub fn handle_not_found(&mut self, handler: HandlerFunc) {
        self.not_found_handler = handler;
    }

    /// get handler from mux
    pub fn handler(&self, r: &Request<()>) -> Option<&HandlerFunc> {
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
    fn serve_http(&self, w: &mut Builder, r: &Request<()>) -> Response<Bytes> {
        // match router
        if let Some(handler) = self.handler(&r) {
            return handler.serve_http(w, r);
        }
        self.not_found_handler.serve_http(w, r)
    }
}
