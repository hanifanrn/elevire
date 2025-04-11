use std::env::var;

pub fn get_gemini_api_key() -> String {
    var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set in .env")
}
