use serde::Deserialize;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

use super::matrix::Matrix;


#[derive(Deserialize)]
struct Data {
  rows: Vec<Vec<u8>>,
}

#[post("/post", format = "json", data = "<data>")]
fn post(data: Json<Data>, matrix: State<Matrix>) -> () {
  matrix.send_pixels(&data.rows);
}

pub fn start(matrix: Matrix) {
  rocket::ignite()
    .manage(matrix)
    .mount("/", StaticFiles::from("web"))
    .mount("/", routes![post])
    .launch();
}
