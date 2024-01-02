//chatbot.rs
use rocket::serde::{json::Json, Deserialize, Serialize};
use reqwest::Client;
use std::{env, string};
use std::collections::HashMap;
use serde_json;

use crate::utils::ask_chat_gpt;
use crate::analyze::{ChatAPIRequest, ChatAPIResponse, ParsedChatAPIResponse, FormattedChatResponse, ChatChoice, ChatMessageContent, UserChatMessage};
// Définition des structures pour la communication avec l'API ChatGPT

#[derive(Deserialize)]
pub struct QuestionRequest {
    pub action: String,
    pub question: String,
    pub code_asm: HashMap<String, Vec<Vec<String>>>, // Utiliser une HashMap
    pub code_c: HashMap<String, String>,
}

// Constante pour la partie fixe du prompt utilisée dans chatbot
pub const CHATBOT_PROMPT: &str = r#"Je vais te poser une question sur différent sujets : Assembleur, Cybersécurité, Programation. Tu devras répondre à la question 
en te basant sur le code assembleur et décompilé qui te sera envoyé.

{question}

Code Assembleur:
{code_assembleur}

Code Décompilé:
{code_decompile}

Format de réponse attendu :
{
    "answer": "réponse"
}"#;

// Endpoint pour le chatbot
#[post("/handle_chatbot", data = "<post_data>")]
pub async fn handle_chatbot(post_data: Json<QuestionRequest>) -> String {
    let mut formatted_code_asm = String::new();

    // Itérer sur chaque fonction dans la HashMap
    for (function_name, lines) in &post_data.code_asm {
        formatted_code_asm.push_str(&format!("Fonction {}:\n", function_name));
        for line in lines {
            if line.len() == 2 {
                formatted_code_asm.push_str(&format!("{}: {}\n", line[0], line[1]));
            }
        }
        formatted_code_asm.push_str("\n"); // Ajoute un saut de ligne entre les fonctions
    }

    let code_c_json = serde_json::to_string(&post_data.code_c)
    .unwrap_or_else(|_| "Erreur lors de la conversion en JSON".to_string());
    let full_prompt = CHATBOT_PROMPT.replace("{code_assembleur}", &formatted_code_asm).replace("{question}", &post_data.question).replace("{code_decompile}", &code_c_json);



    match ask_chat_gpt(full_prompt, "chatbot").await {
        Ok(response) => response,
        Err(_) => "Erreur lors de la communication avec ChatGPT".to_string(),
    }
}