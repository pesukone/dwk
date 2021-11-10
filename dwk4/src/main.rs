use uuid::Uuid;
use std::time::SystemTime;
use time::OffsetDateTime;
use std::fs;

fn main() {
    let uuid = Uuid::new_v4();
    loop {
        let now = OffsetDateTime::from(SystemTime::now());
        fs::create_dir_all("files/").unwrap();
        fs::write("files/timestamp.txt", format!("{}: {}", now.to_string(), uuid))
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5000));
    }
}
