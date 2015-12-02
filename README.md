# vecio

[![Unix Status](https://travis-ci.org/seanmonstar/vecio.svg?branch=master)](https://travis-ci.org/seanmonstar/vecio)
[![Windows status](https://ci.appveyor.com/api/projects/status/kpxw3a297l822sb6?svg=true)](https://ci.appveyor.com/project/seanmonstar/vecio)

Vector IO, scatter/gather, readv, writev

Works on file descriptors on Unix, and sockets on Windows.

## Example

```rust
extern crate vecio;

use vecio::Rawv;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("0.0.0.0").unwrap();
    stream.writev(&[b"foo", b"bar"]).unwrap();
}
```

## Details

There are 3 traits of import in `vecio`:

- `Rawv`
- `Writev`
- `Readv`

The `Rawv` trait implements `Writev` and `Readv` for any type that implements either `AsRawFd` or `AsRawSocket` (unix and windows).

The `Writev` and `Readv` traits exist so that any type that needs a custom implementation can have one.

In simple cases, just import `vecio::Rawv` will give the methods on the proper types.
