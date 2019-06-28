//! Async IO traits that use futures instead of poll.
//!
//! ## Example
//!
//! ```rust
//! #![feature(async_await)]
//!
//! use futures::executor::block_on;
//! use associated_async_io::AsyncIterator;
//! use futures::future::{self, Future};
//! use std::pin::Pin;
//!
//! #[derive(Debug)]
//! struct KittenIterator {
//!     cursor: usize,
//!     kittens: Vec<String>,
//! }
//!
//! impl KittenIterator {
//!     fn new(kittens: Vec<String>) -> Self {
//!         Self { cursor: 0, kittens }
//!     }
//! }
//!
//! impl AsyncIterator for KittenIterator {
//!     type Item = String;
//!     type Fut = Pin<Box<Future<Output = Option<Self::Item>>>>;
//!     fn next(&mut self) -> Self::Fut {
//!         let cursor = self.cursor;
//!         self.cursor += 1;
//!         let kitten = self.kittens.get(cursor).map(|k| k.clone());
//!         Box::pin(future::ready(kitten))
//!     }
//! }
//!
//! fn main () {
//!     block_on(async {
//!         let kittens = vec!["chashu".to_owned(), "nori".to_owned()];
//!         let mut kittens = KittenIterator::new(kittens);
//!         AsyncIterator::next(&mut kittens);
//!     })
//! }
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]
#![cfg_attr(test, deny(warnings))]

use std::future::Future;
use std::io;

/// Async version of `std::io::Read`.
pub trait AsyncRead {
    /// The future returned by `read`.
    type Fut: Future<Output = io::Result<usize>>;
    /// Read data.
    fn read(&mut self, buf: &mut [u8]) -> Self::Fut;
}

/// Async version of `std::io::Write`.
pub trait AsyncWrite {
    /// The future returned by `write`.
    type Fut: Future<Output = io::Result<usize>>;
    /// Write data.
    fn write(&mut self, buf: &[u8]) -> Self::Fut;
}

/// Async version of `std::iter::Iterator`.
pub trait AsyncIterator {
    /// The item returned by `next`.
    type Item;
    /// The future returned by `next`.
    type Fut: Future<Output = Option<Self::Item>>;
    /// Yield the next item.
    fn next(&mut self) -> Self::Fut;
}
