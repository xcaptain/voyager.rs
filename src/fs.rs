use crate::http::Handler;
use bytes::Bytes;
use http::response::Builder;
use http::{Request, Response, StatusCode};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
        let filename = r.uri().path();
        let full_path = Path::new(&self.root).join(filename);
        let mut file = match File::open(&full_path) {
            Err(why) => {
                let s = format!("couldn't open {:?}, {}", full_path, why.description());
                return w
                    .status(StatusCode::BAD_REQUEST)
                    .body(Bytes::from(s))
                    .unwrap();
            }
            Ok(file) => file,
        };

        let mut buf = Vec::new();
        match file.read_to_end(&mut buf) {
            Err(why) => {
                let ss = format!("couldn't read {:?}, {}", full_path, why.description());
                return w
                    .status(StatusCode::BAD_REQUEST)
                    .body(Bytes::from(ss))
                    .unwrap();
            }
            Ok(_nbytes) => {
                return w.body(Bytes::from(buf)).unwrap();
            }
        }
    }
}
