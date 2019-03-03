# voyager web framework

voyager is a web framework written in rust, inspired by Go `net/http`

## features

1. totally based on rust
2. clean api
3. minium dependency

## quick start

run the sample application:

```sh
cargo run --example hello
curl -i localhost:8080/hello
curl -i localhost:8080/hi
```

## contribute

before commit run:

```sh
cargo fmt
cargo clippy
```

## todo

- [ ] follow `gorilla/mux` api
- [ ] middleware
- [ ] json example
