#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;
use rocket::http::ContentType;
use rocket::fs::FileServer;
use std::collections::HashMap;
use std::time::Duration;
use std::fs;

#[get("/")]
async fn index() -> (ContentType, Template) {
    fs::create_dir_all("images/").unwrap();
    let metadata = fs::metadata("images/img.jpg");
    if metadata.is_ok() {
        let modified = metadata.unwrap().modified().unwrap();
        let one_day = Duration::from_secs(60*60*24);
        
        if modified.elapsed().unwrap() >= one_day {
            let body = reqwest::get("https://picsum.photos/1200")
                .await
                .unwrap()
                .bytes()
                .await
                .unwrap();
            fs::write("images/img.jpg", body).unwrap();
        }
    } else {
        let body = reqwest::get("https://picsum.photos/1200")
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        fs::write("images/img.jpg", body).unwrap();
    }

    let context: HashMap<&str, &str> = HashMap::new();
    (ContentType::HTML, Template::render("index", &context))
}

#[rocket::main]
async fn main() {
    fs::create_dir_all("images/").unwrap();
    rocket::build()
        .mount("/static", FileServer::from("images"))
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch()
        .await
        .unwrap();
}
