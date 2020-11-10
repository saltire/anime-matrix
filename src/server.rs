use serde::Deserialize;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;


#[derive(Deserialize)]
struct Data {
  rows: Vec<Vec<u8>>,
}

#[post("/post", format = "json", data = "<data>")]
fn post(data: Json<Data>) -> () {
  println!("{:?}", data.rows);
}

pub fn start() {
  rocket::ignite()
    .mount("/", StaticFiles::from("web"))
    .mount("/", routes![post])
    .launch();
}
