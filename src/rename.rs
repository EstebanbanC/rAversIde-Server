// rename.rs
use rocket::get;

#[get("/rename/<old>")]
pub fn rename_variable(old: String) -> String {
    format!("The variable '{}' has been renamed", old)
}