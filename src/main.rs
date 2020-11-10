#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;


mod matrix;
mod server;

fn main() {
  matrix::usb();

  server::start();
}
