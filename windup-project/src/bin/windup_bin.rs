#![deny(clippy::all)]
#![no_std]
#![no_main]
#![feature(never_type)]

extern crate alloc;
extern crate game;

#[cfg(not(doc))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  // The #[panic_handler] must be in the top level crate, but we foward to the
  // craydate implementation.
  craydate::panic_handler(info)
}

// TODO: for SOME reason if you call the proc macro inside of windup/lib.rs, then
// .capi_handler is dropped from the resulting link (however bss start/end are fine???).
// If you call it here, then it is included properly.
#[craydate::main]
async fn main(api: craydate::Api) -> ! {
  game::main(api).await
}