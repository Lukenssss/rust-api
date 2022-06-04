use rocket::{fs::NamedFile, http::ContentType};

#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, "<h1>Title</h1>")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}