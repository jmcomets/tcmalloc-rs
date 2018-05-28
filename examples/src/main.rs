#![feature(global_allocator)]
extern crate tcmalloc;

use tcmalloc::TCMalloc;

#[global_allocator]
static GLOBAL: TCMalloc = TCMalloc;

fn main() {
    let _v: Vec<_> = (0..1000).collect();
}
