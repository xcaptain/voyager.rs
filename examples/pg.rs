/// this example shows how to connect to a postgresql database
use bytes::Bytes;
use http::response::Builder;
use http::{Request, Response};
use postgres::{Connection, TlsMode};
use std::sync::{Arc, Mutex};
use voyager::http as myhttp;
use voyager::http::HandlerFunc;
use voyager::mux::DefaultServeMux;
use voyager::server::DefaultServer;

/// create database voyager_test;
/// create table person(id serial primary key, name varchar(20) not null default '');
fn main() -> Result<(), Box<std::error::Error>> {
    let mut m = DefaultServeMux::new();
    let conn =
        Connection::connect("postgres://postgres@localhost:5432/voyager_test", TlsMode::None).unwrap();

    m.handle_func("/person/".to_string(), create_person(Arc::new(Mutex::new(conn))));

    return myhttp::listen_and_serve(DefaultServer::new(
        "127.0.0.1:8080".to_string(),
        Box::new(m),
    ));
}

fn create_person(conn: Arc<Mutex<Connection>>) -> HandlerFunc {
    let handler = move |w: &mut Builder, r: Request<()>| -> Response<Bytes> {
        let conn = conn.lock().unwrap();
        conn.execute("INSERT INTO person (name) VALUES ($1)", &[&"joey"])
            .unwrap();
        let str_response = format!("success");
        w.body(Bytes::from(str_response)).unwrap()
    };
    Box::new(handler)
}
