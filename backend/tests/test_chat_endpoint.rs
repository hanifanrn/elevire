use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ChatResponse {
    response: String,
}

#[tokio::test]
async fn test_chat_endpoint() {
    let client = Client::new();
    let url = "http://localhost:8081/chat";

    let payload = json!({
        "prompts": "hello gemini"
    });

    let res = client
        .post(url)
        .json(&payload)
        .send()
        .await
        .expect("Failed to send request");

    assert!(
        res.status().is_success(),
        "Request failed with status: {}",
        res.status()
    );

    let response_json: ChatResponse = res.json().await.expect("Failed to parse response as JSON");

    assert!(
        !response_json.response.trim().is_empty(),
        "Expected a non-empty response from Gemini"
    );
}
