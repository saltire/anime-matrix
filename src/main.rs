#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


mod matrix;
mod server;

fn main() {
  let anime = matrix::Matrix::new().unwrap();

  server::start(anime);
}
