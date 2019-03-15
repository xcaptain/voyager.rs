use crate::mux::Handler;
use bytes::Bytes;
use http::{Request, Response};
use Response::builder;

pub struct FileServer {
    root: String,
}

impl FileServer {
    pub fn new(root: String) -> Self {
        FileServer { root }
    }
}

impl Handler for FileServer {
    fn serve_http(&self, w: &mut Builder, r: Request<()>) -> Response<Bytes> {
        //
    }
}
