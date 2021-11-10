#[macro_use] extern crate rocket;

use rocket::State;
use std::sync::Mutex;
use std::fs;

struct Counter { count: Mutex<u32> }

#[get("/")]
fn pingpong(state: &State<Counter>) -> String {
    let mut temp = state.count.lock().unwrap();
    *temp += 1;
    fs::create_dir_all("persistent-files/").unwrap();
    fs::write("persistent-files/count.txt", temp.to_string())
        .unwrap();

    format!("pong {}", *temp)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Counter { count: Mutex::new(0) })
        .mount("/pingpong", routes![pingpong])
}
