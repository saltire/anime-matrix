use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;


#[derive(Deserialize)]
struct Data {
  values: Vec<u8>,
}

#[post("/post", format = "json", data = "<data>")]
fn post(data: Json<Data>) -> () {
  println!("{:?}", data.values);
}

pub fn start() {
  rocket::ignite()
    .mount("/", StaticFiles::from("web"))
    .mount("/", routes![post])
    .launch();
}
