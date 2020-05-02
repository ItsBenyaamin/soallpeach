#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::config::{Config, Environment};
use rocket::{State, Data};
use rocket::response::status;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::Read;
use rocket::http::Status;

#[post("/", data = "<data>")]
fn sum_route(counter: State<'_, HitCount>, data: Data) -> Result<status::Custom<String>, String> {
    let mut str_data = String::new();
    data.open().read_to_string(&mut str_data).expect("error in reading data!");
    let int_number: usize = str_data.parse().expect("data is not a number!");
    let result = int_number + counter.count.load(Ordering::SeqCst);
    counter.count.store(result, Ordering::SeqCst);
    Ok(status::Custom(Status::Ok,format!("{}", result)))
}

#[get("/count")]
fn count_route(counter: State<'_, HitCount>) -> Result<status::Custom<String>, String> {
    Ok(status::Custom(Status::Ok, format!("{}", counter.count.load(Ordering::SeqCst))))
}

struct HitCount {
    count: AtomicUsize
}

fn main() {
    let config = Config::build(Environment::Production)
        .address("localhost")
        .secret_key("sa4KVi6JOzGizfmXxckcUyrYXTU4IGgKFXHVrZeH050=")
        .port(8080)
        .finalize();
    match config {
        Ok(config)=> {
            rocket::custom(config)
                .manage(HitCount{count: AtomicUsize::new(0)})
                .mount("/", routes![sum_route, count_route]).launch();
        }
        _ => {panic!("there is a problem!")}
    }
}
