use rocket_contrib::templates::Template;

#[derive(Serialize)]
struct TemplateContext {
  title: String,
  text: String,
}

#[get("/")]
fn index() -> Template {
  let ctx = TemplateContext {
    title: String::from("Hello!"),
    text: String::from("Hello world!!!"),
  };
  Template::render("hello", &ctx)
}

pub fn start() {
  rocket::ignite()
    .mount("/", routes![index])
    .attach(Template::fairing())
    .launch();
}
