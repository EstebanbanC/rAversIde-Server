// highlight.rs
use rocket::get;

#[get("/highlight/<address>")]
pub fn highlight_address(address: String) -> String {
    format!("Address: {}", address)
}