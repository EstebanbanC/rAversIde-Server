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
pub const ANALYZE_PROMPT: &str = r#"Analyse le code assembleur et le code décompilé suivant pour détecter des vulnérabilités spécifiques. Utilise les critères de vulnérabilité suivants : overflow de tampon, utilisation de fonctions non sécurisées (ex. gets, strcpy, rand), erreurs de formatage de chaînes, et tout autre problème de sécurité potentiel. Pour chaque vulnérabilité détectée, ajoute un commentaire directement sur la ligne concernée dans le format suivant :

- Adresse ou ligne où la vulnérabilité a été détectée
- Description détaillée de la vulnérabilité, incluant le type de vulnérabilité et pourquoi elle est problématique
- Niveau de gravité indiqué par une couleur : rouge pour les vulnérabilités critiques, orange pour les vulnérabilités moyennes, jaune pour les vulnérabilités mineures.

Assure-toi de ne commenter que les lignes où des vulnérabilités ont été identifiées. Ignorer les lignes sans vulnérabilités.

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
