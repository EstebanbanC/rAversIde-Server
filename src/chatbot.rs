// rename.rs
use rocket::get;

#[get("/chatbot/<prompt>")]
pub fn handle_chatbot(prompt: String) -> String {
    format!("Response : '{}'", prompt)
}