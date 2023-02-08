#[macro_use] extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello {} {}!", name, age)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}


