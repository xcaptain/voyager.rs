# voyager web framework

voyager is a web framework written in rust, inspired by Go `net/http`

## features

1. totally based on rust
2. clean api learned from Go
3. middleware based
4. interface first, components are replacable

## quick start

run the hello example:

```sh
cargo run --example hello
curl -i localhost:8080/hello
curl -i localhost:8080/hi
curl -i localhost:8080/static/test.png
```

## architecture

1. `http::listen_and_serve` accept a `Server` instance and run the server
2. `server::Server` is a trait that defines how to implement a server that can serve requests, with this abstraction
   users can implement their own flavor of server, the default server is implemented by `tokio` and `http` crate.
3. `mux::DefaultMux` contains a default mux implementation to route requests to handler.

Because `tokio` works on multiple threads, so every shared data must be wrapped by `Arc`.

## contribute

before commit run:

```sh
cargo fmt
cargo clippy
```

## todo

- [ ] implement `go-chi/chi` api as the default mux
- [x] middleware
- [x] json example [see](./examples/json.rs)
- [ ] a database example
- [x] static file serving(./examples/hello.rs)

## benchmark

### go with net/http

demo code

```go
package main

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type Person struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
	Age  int    `json:"age"`
}

func main() {
	var persons []Person
	persons = append(persons, Person{
		ID:   1,
		Name: "person1",
		Age:  10,
	})
	persons = append(persons, Person{
		ID:   2,
		Name: "person2",
		Age:  20,
	})
	http.HandleFunc("/hello", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello, you've requested: %s\n", r.URL.Path)
	})

	http.HandleFunc("/person", func(w http.ResponseWriter, r *http.Request) {
		person := persons[0]
		w.Header().Add("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(person)
	})

	http.ListenAndServe(":8081", nil)
}
```

command:

```sh
go run main.go
wrk2 -t2 -c100 -d30s -R2000 http://127.0.0.1:8081/person
```

result:

```log
Running 30s test @ http://127.0.0.1:8081/person
  2 threads and 100 connections
  Thread calibration: mean lat.: 1.130ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.394ms, rate sampling interval: 10ms
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.24ms  542.80us   6.44ms   66.07%
    Req/Sec     1.05k     1.07k    4.44k    89.87%
  58902 requests in 30.00s, 8.03MB read
Requests/sec:   1963.31
Transfer/sec:    274.17KB
```

### voyager framework

demo code is from the json example, command:

```sh
cargo run --example json
wrk2 -t2 -c100 -d30s -R2000 http://127.0.0.1:8080/person
```

result:

```log
Running 30s test @ http://127.0.0.1:8080/person
  2 threads and 100 connections
  Thread calibration: mean lat.: 1.974ms, rate sampling interval: 10ms
  Thread calibration: mean lat.: 1.569ms, rate sampling interval: 10ms
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.72ms  562.29us   5.89ms   72.95%
    Req/Sec     1.06k     1.18k    5.55k    88.95%
  58902 requests in 30.00s, 9.04MB read
Requests/sec:   1963.27
Transfer/sec:    308.68KB
```

from this benchmark we can see that voyager framework is almost as fast as go, but tokio get errors

```log
failed to process connection; error = Os { code: 104, kind: ConnectionReset, message: "Connection reset by peer" }
```
