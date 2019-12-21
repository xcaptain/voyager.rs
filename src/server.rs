use crate::http::Handler;
use bytes::{Bytes, BytesMut};
use chrono::prelude::{DateTime, Local};
use http::header::HeaderValue;
use http::{Request, Response};
use std::net::SocketAddr;
use std::sync::Arc;
use std::{fmt, io};
use tokio::codec::{Decoder, Encoder};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

/// http server trait, may be different backend implementation
/// e.g tokio, raw tcp socket, quic, http2 and so on
pub trait Server {
    fn listen_and_serve(self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DefaultServer {
    addr: String,
    handler: Box<dyn Handler>,
}

impl DefaultServer {
    pub fn new(addr: String, handler: Box<dyn Handler>) -> Self {
        DefaultServer { addr, handler }
    }
}

impl Server for DefaultServer {
    fn listen_and_serve(self) -> Result<(), Box<dyn std::error::Error>> {
        let addr = self.addr.parse::<SocketAddr>()?;
        let listener = TcpListener::bind(&addr)?;
        let ah = Arc::new(self.handler);
        tokio::run({
            listener
                .incoming()
                .map_err(|e| println!("failed to accept socket; error = {:?}", e))
                .for_each(move |socket| {
                    process(socket, ah.clone());
                    Ok(())
                })
        });
        Ok(())
    }
}

fn process(socket: TcpStream, ah: Arc<Box<dyn Handler>>) {
    let (tx, rx) =
        // Frame the socket using the `Http` protocol. This maps the TCP socket
        // to a Stream + Sink of HTTP frames.
        Http.framed(socket)
        // This splits a single `Stream + Sink` value into two separate handles
        // that can be used independently (even on different tasks or threads).
        .split();

    // Map all requests into responses and send them back to the client.
    let task = tx
        .send_all(rx.and_then(
            move |req| -> Box<dyn Future<Item = Response<Bytes>, Error = io::Error> + Send> {
                let ah = ah.clone();
                let f = future::lazy(move || {
                    let mut response_builder = Response::builder();
                    let response = ah.serve_http(&mut response_builder, req);
                    Ok(response)
                });

                Box::new(f)
            },
        ))
        .then(|res| {
            if let Err(e) = res {
                println!("failed to process connection; error = {:?}", e);
            }

            Ok(())
        });

    // Spawn the task that handles the connection.
    tokio::spawn(task);
}

// code below is copied from `tokio tinyhttp example`
struct Http;

/// Implementation of encoding an HTTP response into a `BytesMut`, basically
/// just writing out an HTTP/1.1 response.
impl Encoder for Http {
    type Item = Response<Bytes>;
    type Error = io::Error;

    fn encode(&mut self, item: Response<Bytes>, dst: &mut BytesMut) -> io::Result<()> {
        use std::fmt::Write;

        let local: DateTime<Local> = Local::now();
        write!(
            BytesWrite(dst),
            "\
             HTTP/1.1 {}\r\n\
             Server: Example\r\n\
             Content-Length: {}\r\n\
             Date: {}\r\n\
             ",
            item.status(),
            item.body().len(),
            local.to_rfc2822(),
        )
        .unwrap();

        for (k, v) in item.headers() {
            dst.extend_from_slice(k.as_str().as_bytes());
            dst.extend_from_slice(b": ");
            dst.extend_from_slice(v.as_bytes());
            dst.extend_from_slice(b"\r\n");
        }

        dst.extend_from_slice(b"\r\n");
        dst.extend_from_slice(item.body());

        return Ok(());

        // Right now `write!` on `Vec<u8>` goes through io::Write and is not
        // super speedy, so inline a less-crufty implementation here which
        // doesn't go through io::Error.
        struct BytesWrite<'a>(&'a mut BytesMut);

        impl<'a> fmt::Write for BytesWrite<'a> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.extend_from_slice(s.as_bytes());
                Ok(())
            }

            fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
                fmt::write(self, args)
            }
        }
    }
}

/// Implementation of decoding an HTTP request from the bytes we've read so far.
/// This leverages the `httparse` crate to do the actual parsing and then we use
/// that information to construct an instance of a `http::Request` object,
/// trying to avoid allocations where possible.
impl Decoder for Http {
    type Item = Request<()>;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> io::Result<Option<Request<()>>> {
        // TODO: we should grow this headers array if parsing fails and asks
        //       for more headers
        let mut headers = [None; 16];
        let (method, path, version, amt) = {
            let mut parsed_headers = [httparse::EMPTY_HEADER; 16];
            let mut r = httparse::Request::new(&mut parsed_headers);
            let status = r.parse(src).map_err(|e| {
                let msg = format!("failed to parse http request: {:?}", e);
                io::Error::new(io::ErrorKind::Other, msg)
            })?;

            let amt = match status {
                httparse::Status::Complete(amt) => amt,
                httparse::Status::Partial => return Ok(None),
            };

            let toslice = |a: &[u8]| {
                let start = a.as_ptr() as usize - src.as_ptr() as usize;
                assert!(start < src.len());
                (start, start + a.len())
            };

            for (i, header) in r.headers.iter().enumerate() {
                let k = toslice(header.name.as_bytes());
                let v = toslice(header.value);
                headers[i] = Some((k, v));
            }

            (
                toslice(r.method.unwrap().as_bytes()),
                toslice(r.path.unwrap().as_bytes()),
                r.version.unwrap(),
                amt,
            )
        };
        if version != 1 {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "only HTTP/1.1 accepted",
            ));
        }
        let data = src.split_to(amt).freeze();
        let mut ret = Request::builder();
        ret.method(&data[method.0..method.1]);
        ret.uri(data.slice(path.0, path.1));
        ret.version(http::Version::HTTP_11);
        for header in headers.iter() {
            let (k, v) = match *header {
                Some((ref k, ref v)) => (k, v),
                None => break,
            };
            let value = unsafe { HeaderValue::from_shared_unchecked(data.slice(v.0, v.1)) };
            ret.header(&data[k.0..k.1], value);
        }

        let req = ret
            .body(())
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(Some(req))
    }
}
