#[macro_use] extern crate rocket;

#[get("/")]
fn home() -> &'static str {
  "Welcome to Sweet Home "
}

#[get("/properties/<title>")]
fn GetProperty(title: &str) -> String {
  format!("Your property is available for sale, soon we will have news from {}!", title)
}

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount("/", routes![home])
    .mount("/", routes![GetProperty])
}