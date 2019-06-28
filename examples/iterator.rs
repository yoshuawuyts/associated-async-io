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
