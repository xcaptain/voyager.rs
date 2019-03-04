# voyager web framework

voyager is a web framework written in rust, inspired by Go `net/http`

## features

1. totally based on rust
2. clean api
3. minium dependency

## quick start

run the hello example:

```sh
cargo run --example hello
curl -i localhost:8080/hello
curl -i localhost:8080/hi
```

## architecture

1. `http.listen_and_serve` setup http server and bind a `mux` to the server so each incoming request can be dispatched
2. `mux` is a collection of `handler`s, this module register `handler` and resolve `handler` from each request
3. a `Handler` is in fact a closure, it accepts a `RequestBuilder` and a `Request` as arguments and returns a `Response`

Because `tokio` works on multiple threads, so every shared data must be wrapped by `Arc`.

## contribute

before commit run:

```sh
cargo fmt
cargo clippy
```

## todo

- [ ] implement `gorilla/mux` api
- [ ] middleware
- [x] json example [see](./examples/json.rs)
- [ ] a database example
- [ ] static file serving
