use bytes::Bytes;
use http::response::Builder;
use http::{Request, Response};

pub use crate::server::listen_and_serve;

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

pub fn strip_prefix(prefix: String, h: Box<dyn Handler>) -> Box<dyn Handler> {
    if prefix.is_empty() {
        return h;
    }
    // restruct uri path and create a new request instance
    // TODO: must be carefully revised to ensure just prefix has been trimed
    let handler: HandlerFunc =
        Box::new(move |w: &mut Builder, r: &Request<()>| -> Response<Bytes> {
            // let (mut parts, body) = r.into_parts();
            // parts.uri = Uri::builder().build().unwrap();
            // let new_r = Request::from_parts(parts, body);
            // `http` crate is so difficult to use, fuck
            h.serve_http(w, r)
        });
    Box::new(handler)
}
