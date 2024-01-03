//analyze.rs
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::ask_chat_gpt;

// Définition des structures pour la communication avec l'API ChatGPT
#[derive(Deserialize)]
pub struct ChatAPIResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessageContent,
}

#[derive(Deserialize)]
pub struct ChatMessageContent {
    pub content: String,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub struct ChatAPIRequest {
    pub model: String,
    pub messages: Vec<UserChatMessage>,
}

#[derive(Serialize)]
#[cfg_attr(test, derive(Deserialize))]
pub struct UserChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct AssemblerCodeLine {
    pub address: String,
    pub instruction: String,
}

#[derive(Deserialize)]
pub struct CodeAnalysisRequest {
    pub action: String,
    pub r#type: String,
    pub code_asm: HashMap<String, Vec<Vec<String>>>, // Utiliser une HashMap
    pub code_c: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormattedChatResponse {
    pub comment: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ParsedChatAPIResponse {
    pub comment: Vec<(String, String, String)>,
}


// Constante pour la partie fixe du prompt utilisée dans chatbot
pub const ANALYZE_PROMPT: &str = r#"
Analyse le code assembleur et le code décompilé suivant pour détecter des vulnérabilités spécifiques et identifier les patterns cryptographiques. L'objectif est d'aider des reverseurs a comprendre plus efficacement les informations cruciales.

Vérifie les critères de vulnérabilité suivants : overflow de tampon, utilisation de fonctions non sécurisées (ex. gets, strcpy, rand), erreurs de formatage de chaînes, utilisation incorrecte des algorithmes cryptographiques, et tout autre problème de sécurité potentiel. (Ce ne sont que des exemples, tu dois être très précis dans les explications que tu donnes)

Pour chaque élément détecté, qu'il s'agisse d'une vulnérabilité ou d'un élément cryptographique, ajoute un commentaire directement sur la ligne concernée avec les détails suivants :
- Adresse ou ligne concernée
- Description détaillée, incluant le type d'élément détecté et pourquoi il est important ou problématique
- Niveau de gravité ou d'importance, indiqué par une couleur : rouge pour les éléments critiques, orange pour les moyens, jaune pour les mineurs.

Assure-toi de ne commenter que les lignes pertinentes.

Code Assembleur:
{code_assembleur}

Code Décompilé:
{code_decompile}

Format de réponse attendu :
{
  "comment": [
    ["adresse_ou_ligne", "commentaire_sur_le_code", "couleur"]
    // Autres commentaires ici
  ]
}"#;



// Endpoint pour le chatbot
#[post("/analyze", data = "<post_data>")]
pub async fn analyze(post_data: Json<CodeAnalysisRequest>) -> String {
    let mut formatted_code_asm = String::new();

    // Itérer sur chaque fonction dans la HashMap
    for (function_name, lines) in &post_data.code_asm {
        formatted_code_asm.push_str(&format!("Fonction {}:\n", function_name));
        for line in lines {
            if line.len() == 2 {
                formatted_code_asm.push_str(&format!("{}: {}\n", line[0], line[1]));
            }
        }
        formatted_code_asm.push_str("\n"); 
    }

    let code_c_json = serde_json::to_string(&post_data.code_c)
    .unwrap_or_else(|_| "Erreur lors de la conversion en JSON".to_string());

    let full_prompt = ANALYZE_PROMPT.replace("{code_assembleur}", &formatted_code_asm).replace("{code_decompile}", &code_c_json);



    match ask_chat_gpt(full_prompt, "analyze").await {
        Ok(response) => response,
        Err(_) => "Erreur lors de la communication avec ChatGPT".to_string(),
    }
}
