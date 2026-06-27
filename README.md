# http-in-rust

An HTTP/1.1 server written from scratch in Rust, using only the standard library. No `tokio`, no `hyper`, no external crates. It implements the request/response lifecycle directly on top of raw TCP sockets: connection handling, an incremental request parser, header validation, response serialization, and a worker thread pool.

The project exists as a study of what web frameworks and async runtimes abstract away. The crate is named `httpfromtcp`.

## Features

- **Zero runtime dependencies.** Built entirely on `std::net`, `std::io`, `std::thread`, and `std::sync`.
- **Incremental, streaming request parser.** Requests are parsed by a state machine (`Initialized -> ParsingHeader -> ParsingBody -> Done`) that consumes bytes as they arrive, correctly handling requests delivered across multiple TCP reads rather than assuming a single complete buffer.
- **Spec-aware header parsing.** Case-insensitive header names, header-name validation against the RFC 7230 token set, folding of repeated headers into a comma-separated value, and CRLF / blank-line boundary detection.
- **Content-Length body handling**, with validation against the declared length.
- **Response builder.** Serializes the status line, headers, and body with correct framing, and sets `Content-Length` automatically.
- **Fixed-size thread pool.** Connections are dispatched to a pool of worker threads over an `mpsc` channel guarded by `Arc<Mutex<Receiver>>`, with graceful shutdown implemented through `Drop`.
- **Test harness for fragmented input.** A custom `ChunkReader` implements `Read` to feed the parser a configurable number of bytes per call, verifying the parser behaves correctly under partial reads.

## Request lifecycle

```
TCP connection accepted
        |
        v
ThreadPool dispatches the stream to a worker
        |
        v
request_from_reader: read bytes into a buffer
        |
        v
Request::parse (state machine)
   request line  ->  headers  ->  body
        |
        v
Response::to_bytes -> stream.write_all
```

## Project structure

```
src/
  main.rs                     HTTP server entry point (binds, accepts, responds)
  lib.rs
  utils.rs                    Token validation and helpers
  internal/
    requests/
      request.rs              Request type, parse state machine, request_from_reader
      parse_stream.rs         ChunkReader (Read impl for fragmented-input testing)
      request_test.rs
    headers/
      headers.rs              Header parsing and validation
      headers_test.rs
    body/
      body.rs
    response/
      response.rs             Response type and serialization
    thread_pool.rs            Worker thread pool with graceful shutdown
  tcpsender/  udpsender/  udpreceiver/   Standalone socket experiments
```

The repository also defines auxiliary binaries (`tcpsender`, `udpsender`, `udpreceiver`) used to explore raw TCP and UDP socket behavior. The HTTP server is the `main` binary.

## Requirements

- Rust (stable toolchain, edition 2024)
- Cargo

## Build and run

```bash
# Build
cargo build --release

# Run the HTTP server (binds to 127.0.0.1:8080)
cargo run --bin main
```

In a second terminal:

```bash
curl -v http://127.0.0.1:8080/
```

Expected response:

```
HTTP/1.1 200 OK
Content-Type: text/plain
Content-Length: 38

Hello from a from-scratch Rust server!
```

## Testing

```bash
cargo test
```

The test suite covers header parsing, including byte-offset accounting and rejection of malformed input (for example, headers with invalid surrounding whitespace).

## Design notes

- **Why a state machine.** TCP delivers a stream of bytes, not framed messages. A single `read` may return part of a header or several requests at once. Parsing incrementally and reporting the number of bytes consumed lets the server make progress with whatever data has arrived so far.
- **Why a thread pool.** Each connection is handled on a worker thread, so a slow or blocked request does not stall others. Bounding the pool size caps resource usage. This is a deliberate, blocking design; it illustrates the model that asynchronous runtimes such as `tokio` replace to scale to large connection counts.

## Limitations and roadmap

This is an educational implementation and is not production-ready. Known limitations, in rough priority order:

- No routing: every request returns a single fixed response.
- HTTP/1.1 keep-alive is not implemented; connections are handled once.
- Request bodies are parsed via `Content-Length` only; chunked `Transfer-Encoding` is not supported.
- No TLS/HTTPS.
- The listen address and port are fixed in source.

Planned improvements: a routing layer, keep-alive support, chunked transfer decoding, and configurable bind settings.

## License

Released under the MIT License. See `LICENSE` for details.
