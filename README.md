# associate-async-io
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Async IO traits that use futures instead of poll. This is an experiment to see
if using `async fn` for the `futures::io` traits is possible.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Why does this exist?
This is useful because currently implementing `AsyncRead`, `AsyncWrite`, and
`Stream` require knowledge of `Poll`, `Pin`, and arbitrary self types. We
think it would be an ergonomics improvement if knowledge of these concepts was
not required to implement these traits. Instead once `async fn` in traits comes
around we think that would make a great fit:

```rust
pub trait AsyncRead {
    async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

pub trait AsyncWrite {
    async fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
}

pub trait AsyncIterator {
    type Item;
    async fn next(&mut self) -> Option<Self::Item>;
}
```

These would be direct async counterparts to `Read`, `Write` and `Iterator`:

```rust
pub trait AsyncRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}

pub trait AsyncWrite {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
}

pub trait AsyncIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

However currently `async fn` in traits doesn't work. So we're defining these
traits with an associated type instead.

```rust
pub trait AsyncRead {
    type Fut: Future<Output = io::Result<usize>>;
    fn read(&mut self, buf: &mut [u8]) -> Self::Fut;
}

pub trait AsyncWrite {
    type Fut: Future<Output = io::Result<usize>>;
    fn write(&mut self, buf: &[u8]) -> Self::Fut;
}

pub trait AsyncIterator {
    type Item;
    type Fut: Future<Output = Option<Self::Item>>;
    fn next(&mut self) -> Self::Fut;
}
```

Because of compiler reasons this means there currently is the overhead of an
extra box. But we think that's fine, as it's unlikely to become a bottleneck,
and this would be temporary anyway.

However a limitation is that this can't return borrowed values, as it relies on
GATs. Which seems like the most convincing counterpoint to using these traits
today.

## Examples
```rust
#![feature(async_await)]

use futures::executor::block_on;
use associated_async_io::AsyncIterator;
use futures::future::{self, Future};
use std::pin::Pin;

#[derive(Debug)]
struct KittenIterator {
    cursor: usize,
    kittens: Vec<String>,
}

impl KittenIterator {
    fn new(mut kittens: Vec<String>) -> Self {
        kittens.reverse();
        Self { cursor: 0, kittens }
    }
}

impl AsyncIterator for KittenIterator {
    type Item = String;
    type Fut = Pin<Box<Future<Output = Option<Self::Item>>>>;
    fn next(&mut self) -> Self::Fut {
        self.cursor += 1;
        let kitten = self.kittens.pop();
        Box::pin(future::ready(kitten))
    }
}

fn main () {
    block_on(async {
        let kittens = vec!["chashu".to_owned(), "nori".to_owned()];
        let mut kittens = KittenIterator::new(kittens);
        AsyncIterator::next(&mut kittens);
    })
}
```

## Installation
```sh
$ cargo add associate-async-io
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/associate-async-io.svg?style=flat-square
[2]: https://crates.io/crates/associate-async-io
[3]: https://img.shields.io/travis/yoshuawuyts/associate-async-io/master.svg?style=flat-square
[4]: https://travis-ci.org/yoshuawuyts/associate-async-io
[5]: https://img.shields.io/crates/d/associate-async-io.svg?style=flat-square
[6]: https://crates.io/crates/associate-async-io
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/associate-async-io

[releases]: https://github.com/yoshuawuyts/associate-async-io/releases
[contributing]: https://github.com/yoshuawuyts/associate-async-io/blob/master.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/yoshuawuyts/associate-async-io/labels/good%20first%20issue
[help-wanted]: https://github.com/yoshuawuyts/associate-async-io/labels/help%20wanted
