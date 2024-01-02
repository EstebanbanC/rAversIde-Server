// highlight.rs
use rocket::get;

#[get("/highlight/<address>")]
pub fn highlight_address(address: String) -> String {
    format!("Address: {}", address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_address() {
        let test_address = "0x123456";
        let result = highlight_address(test_address.to_string());
        assert_eq!(result, format!("Address: {}", test_address));
    }
}