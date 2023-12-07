use reqwest::Client;
use std::env;
use crate::analyze::{ChatAPIRequest, ChatAPIResponse, ParsedChatAPIResponse, FormattedChatResponse};
use serde_json;

pub async fn ask_chat_gpt(prompt: String) -> Result<String, reqwest::Error> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = Client::new();
    let request_body = ChatAPIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![crate::analyze::UserChatMessage {
            role: "user".to_string(),
            content: prompt,
        }],
    };

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&request_body)
        .send()
        .await?;

    let response_body: String = response.text().await?;
    let chat_api_response: ChatAPIResponse = serde_json::from_str(&response_body)
        .expect("Erreur lors de la désérialisation de la réponse");
    
    if let Some(choice) = chat_api_response.choices.get(0) {
        let parsed_response: ParsedChatAPIResponse = serde_json::from_str(&choice.message.content)
            .expect("Erreur lors de la désérialisation de la réponse de ChatGPT");
    
        let formatted_response = FormattedChatResponse {
            comment: parsed_response.comment.into_iter().map(|(address, comment, color)| {
                vec![address, comment, color]
            }).collect(),
        };
    
        let json_response = serde_json::to_string(&formatted_response)
            .expect("Erreur lors de la sérialisation de la réponse formatée");
    
        Ok(json_response)
    } else {
        Ok("Pas de réponse disponible".to_string())
    }
}




// Endpoint pour les commentaires
#[get("/comments/<comment>")]
pub fn comments(comment: String) -> String {
    let comment_ai = "here is the response of the AI";
    format!("Comment is {}, the response is : \n{}",comment, comment_ai)
}


