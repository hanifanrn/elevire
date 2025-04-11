use crate::apis::v1::chat::chat_model::{
    ChatRequest, ChatResponse, Content, GeminiRequest, GeminiResponse, Part,
};
use crate::utils::env::get_gemini_api_key;
use axum::Json;
use reqwest;

pub async fn handle_chat(Json(payload): Json<ChatRequest>) -> Json<ChatResponse> {
    println!("->> {:12} - handler_chat", "HANDLER");

    let api_key = get_gemini_api_key();

    match call_gemini(&payload.prompt, &api_key).await {
        Ok(text) => Json(ChatResponse { response: text }),
        Err(err_msg) => {
            eprintln!("Error: {}", err_msg);
            Json(ChatResponse {
                response: "Something went wrong while talking to Gemini.".into(),
            })
        }
    }
}

pub async fn call_gemini(prompt: &str, api_key: &str) -> Result<String, String> {
    let gemini_url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
        api_key
    );

    let request_body = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let client = reqwest::Client::new();
    let response = client.post(&gemini_url).json(&request_body).send().await;

    match response {
        Ok(resp) => {
            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                return Err(format!("Gemini error {}: {}", status, body));
            }

            match resp.json::<GeminiResponse>().await {
                Ok(data) => {
                    let text = data
                        .candidates
                        .first()
                        .and_then(|c| c.content.parts.first())
                        .map(|p| p.text.clone())
                        .unwrap_or_else(|| "Gemini returned no content.".to_string());
                    Ok(text)
                }
                Err(e) => Err(format!("Failed to parse Gemini response: {:?}", e)),
            }
        }
        Err(e) => Err(format!("Request to Gemini failed: {:?}", e)),
    }
}
