// main.rs
#[macro_use] extern crate rocket;

mod highlight;
mod chatbot;
mod rename;
mod utils;
mod analyze;

use rocket::launch;

// Endpoint pour la page d'accueil
#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![
        utils::comments, 
        highlight::highlight_address, 
        rename::rename_function, 
        rename::rename_variable,
        chatbot::handle_chatbot,
        analyze::analyze
    ])
}

