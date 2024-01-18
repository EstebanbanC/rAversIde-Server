//chatbot.rs
use rocket::serde::{json::Json, Deserialize};
use std::collections::HashMap;
use serde_json;
use rocket::State;

use crate::utils::ask_chat_gpt_chatbot;
// Définition des structures pour la communication avec l'API ChatGPT

#[derive(Deserialize)]
pub struct QuestionRequest {
    pub action: String,
    pub question: String,
    pub code_c: HashMap<String, String>,
}

pub struct WsChannel {
    pub tx: tokio::sync::broadcast::Sender<String>,
}

impl WsChannel {
    pub fn new(tx: tokio::sync::broadcast::Sender<String>) -> Self {
        WsChannel { tx }
    }
}

// Constante pour la partie fixe du prompt utilisée dans chatbot
pub const CHATBOT_PROMPT: &str = r#"Je vais te poser une question sur différent sujets : Assembleur, Cybersécurité, Programation. Tu devras répondre à la question 
en te basant sur le code décompilé qui te sera envoyé.

{question}

Code Décompilé:
{code_decompile}

Format de réponse attendu :
{
    "answer": "réponse"
}"#;

// Endpoint pour le chatbot
#[post("/handle_chatbot", data = "<post_data>")]
pub async fn handle_chatbot(post_data: Json<QuestionRequest>, ws_channel: &State<WsChannel>) -> String {
    
    let code_c_json = serde_json::to_string(&post_data.code_c)
    .unwrap_or_else(|_| "Erreur lors de la conversion en JSON".to_string());
    let full_prompt = CHATBOT_PROMPT.replace("{question}", &post_data.question).replace("{code_decompile}", &code_c_json);


    match ask_chat_gpt_chatbot(full_prompt,  ws_channel.inner().tx.clone()).await {
        Ok(()) => "OK".to_string(),
        Err(_) => "Erreur lors de la communication avec ChatGPT".to_string(),
    }
}