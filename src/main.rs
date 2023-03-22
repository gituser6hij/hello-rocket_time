#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use chrono::{DateTime, Local};
use std::time::SystemTime;

fn current_time() -> String {
    let current_time = SystemTime::now();
    let datetime: DateTime<Local> = current_time.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

#[get("/")]
fn index() -> String {
    let time = current_time();
    format!("Hello, world! The current time is: {}", time)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
