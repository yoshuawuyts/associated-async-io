# associate-async-io
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Async IO traits that use futures instead of poll.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
__Basic usage__
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
    fn new(kittens: Vec<String>) -> Self {
        Self { cursor: 0, kittens }
    }
}

impl AsyncIterator for KittenIterator {
    type Item = String;
    type Fut = Pin<Box<Future<Output = Option<Self::Item>>>>;
    fn next(&mut self) -> Self::Fut {
        let cursor = self.cursor;
        self.cursor += 1;
        let kitten = self.kittens.get(cursor).map(|k| k.clone());
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
