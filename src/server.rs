use rocket_contrib::serve::StaticFiles;

pub fn start() {
  rocket::ignite()
    .mount("/", StaticFiles::from("web"))
    .launch();
}
