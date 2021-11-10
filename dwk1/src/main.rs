#[macro_use] extern crate rocket;

use std::fs;

#[get("/")]
fn log_output() -> String {
    let hash = fs::read_to_string("files/timestamp.txt").unwrap();
    let count = fs::read_to_string("persistent-files/count.txt")
        .unwrap_or(0.to_string())
        .parse::<u32>()
        .unwrap();

    format!("{}\nPings / Pongs: {}", hash, count)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/log", routes![log_output])
}
