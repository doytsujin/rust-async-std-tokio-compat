# async-std to tokio streams

An extension trait to convert [`async-std`](https://async.rs) streams to `tokio` network streams.

Useful if you are using `async-std`, but need to send streams to 3rd party crates such as `hyper`, that require `tokio` streams.

## Usage:

```rust
use async_std_tokio_compat::*;

let std_stream = TcpStream::connect(addr).await?;
let tokio_stream = std_stream.compat();
```
