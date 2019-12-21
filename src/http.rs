use crate::server::Server;
use bytes::Bytes;
use http::response::Builder;
use http::uri::PathAndQuery;
use http::{Request, Response, Uri};
use std::str::FromStr;

/// pass in a server and run the server
pub fn listen_and_serve(server: impl Server) -> Result<(), Box<dyn std::error::Error>> {
    server.listen_and_serve()
}

pub type HandlerFunc = Box<dyn Fn(&mut Builder, Request<()>) -> Response<Bytes> + Sync + Send>;
/// this trait defines how to serve_http
/// to use across multiple threads, this traits must implement Sync and Send
/// because this trait must live longer then w, so add a `static lifetime
pub trait Handler: Sync + Send + 'static {
    fn serve_http(&self, w: &mut Builder, r: Request<()>) -> Response<Bytes>;
}

impl Handler for HandlerFunc {
    fn serve_http(&self, w: &mut Builder, r: Request<()>) -> Response<Bytes> {
        (self)(w, r)
    }
}

pub fn strip_prefix(prefix: String, h: Box<dyn Handler>) -> Box<dyn Handler> {
    if prefix.is_empty() {
        return h;
    }
    // restruct uri path and create a new request instance
    let handler: HandlerFunc =
        Box::new(move |w: &mut Builder, r: Request<()>| -> Response<Bytes> {
            let (mut parts, body) = r.into_parts();
            let mut uri_parts = parts.uri.into_parts();
            let old_uri_path_and_query = uri_parts.path_and_query.unwrap();
            let old_uri_str = old_uri_path_and_query.as_str();
            let new_uri_str = old_uri_str.trim_start_matches(&prefix[..]);
            uri_parts.path_and_query = Some(PathAndQuery::from_str(new_uri_str).unwrap());
            let new_uri = Uri::from_parts(uri_parts).unwrap();
            parts.uri = new_uri;
            let new_r = Request::from_parts(parts, body);
            h.serve_http(w, new_r)
        });
    Box::new(handler)
}
