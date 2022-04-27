#![no_std]
#![deny(clippy::all)]
#![feature(never_type)]

extern crate alloc;

mod gameloop;

#[craydate::main]
async fn main(api: craydate::Api) -> ! {
  gameloop::run(api).await
}
