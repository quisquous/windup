#![deny(clippy::all)]
#![no_std]
#![no_main]

extern crate game;

#[cfg(not(doc))]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
  // The #[panic_handler] must be in the top level crate, but we foward to the
  // craydate implementation.
  craydate::panic_handler(info)
}

// TODO ^ what of all of this can get moved here or elsewhere