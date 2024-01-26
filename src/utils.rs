//utils.rs
use std::env;
use crate::analyze::{ChatAPIRequest, ChatAPIResponse, ParsedChatAPIResponse, FormattedChatResponse};
use crate::rename::{ParsedRenamedResponse,RenameResponse};
use reqwest::Client;
use futures::StreamExt;
use serde_json::{json, Value};

pub async fn ask_chat_gpt(prompt: String, requete:&str) -> Result<String, reqwest::Error> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let client = Client::new();
    let request_body = ChatAPIRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![crate::analyze::UserChatMessage {
            role: "user".to_string(),
            content: prompt,
        }]
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
        if requete == "analyze"
        {
            println!("{}", choice.message.content.clone());
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
        }
        else if requete == "rename"
        {
            let parsed_response: ParsedRenamedResponse = serde_json::from_str(&choice.message.content)
                .expect("Erreur lors de la désérialisation de la réponse de ChatGPT");
        
            let formatted_response = RenameResponse {
                rename: parsed_response.rename.into_iter().map(|(r#type, old_name, new_name)| {
                    vec![r#type, old_name, new_name]
                }).collect(),
            };
        
            let json_response = serde_json::to_string(&formatted_response)
                .expect("Erreur lors de la sérialisation de la réponse formatée");
        
            Ok(json_response)
        }
        else if requete == "chatbot"
        {
            Ok(choice.message.content.clone())
        }
        else
        {
            Ok("Pas de réponse disponible".to_string())
        }
    } else {
        Ok("Pas de réponse disponible".to_string())
    }
}



pub async fn ask_chat_gpt_chatbot(prompt: String, tx: tokio::sync::broadcast::Sender<String>) -> Result<(), Box<dyn std::error::Error>> {
    let api_key: String = env::var("OPENAI_API_KEY")?;
    let client = reqwest::Client::new();

    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [{"role": "user", "content": prompt}],
        "stream": true,
    });

    let response = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&body)
        .send()
        .await?
        .bytes_stream();

    let tx_clone = tx.clone(); // Clone tx
    response
        .for_each_concurrent(None, move |chunk| {
            let tx = tx_clone.clone();
            async move {
                match chunk {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        let parts = text.split("data: ").skip(1); // Split and ignore the first empty element
                        for part in parts {
                            let json_str = format!("{{\"data\": {}}}", part);
                            if let Ok(json) = serde_json::from_str::<Value>(&json_str) {
                                if let Some(content) = json["data"]["choices"][0]["delta"]["content"].as_str() {
                                    let _ = tx.send(content.to_string());
                                }                                
                            }
                        }
                    },
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
        })
        .await;
    
    let _ = tx.send(" FIN".to_string());

    // let mut combined_response = String::new();
    // while let Some(chunk) = rx.recv().await {
    //     combined_response.push_str(&chunk);

    //     println!("combined response : {}", combined_response);
    // }
    Ok(())
}

