//rename.rs
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::utils::ask_chat_gpt;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct RenameRequest {
    pub items: Vec<RenameItem>,
    pub code_asm: HashMap<String, Vec<Vec<String>>>, // Utiliser une HashMap
    pub code_c: HashMap<String, String>,
}

#[derive(Deserialize, Serialize)]
pub struct RenameItem {
    pub item_type: String, // "fonction" ou "variable"
    pub old_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct RenameResponse {
    pub rename: Vec<Vec<String>>, // [["type", "old_name", "new_name"], ...]
}

#[derive(Serialize, Deserialize)]
pub struct ParsedRenamedResponse {
    pub rename: Vec<(String, String, String)>,
}

pub const RENAME_FUNCTION_PROMPT: &str = r#"Suggère des noms plus descriptifs pour les fonctions suivantes en conservant leur signification initiale. Le renommage doit améliorer la lisibilité et la compréhension du code. Pour chaque élément, fournis un nom plus approprié.

Liste des éléments à renommer :
{rename_list}

Code Décompilé:
{code_decompile}

Code Assembleur:
{code_assembleur}

Format de réponse attendu :
{
  "rename": [
    ["type", "ancien_nom", "nouveau_nom"]
    // Autres éléments ici
  ]
}"#;


pub const RENAME_VARIABLE_PROMPT: &str = r#"Suggère des noms plus descriptifs pour les variables des fonctions suivantes en conservant leur signification initiale et en prenant en compte le contexte (code). Le renommage doit améliorer la lisibilité et la compréhension du code. Pour chaque élément, fournis des noms de variables plus approprié.

Liste des éléments à renommer et SEULEMENT ceux-ci:
{rename_list}

Code Décompilé:
{code_decompile}

Code Assembleur:
{code_assembleur}

Format de réponse attendu :
{
  "rename": [
    ["type", "ancien_nom", "nouveau_nom"]
    // Autres éléments ici
  ]
}"#;




// Endpoint pour la fonction rename
#[post("/renameFunction", data = "<rename_function_data>")]
pub async fn rename_function(rename_function_data: Json<RenameRequest>) -> String {    

    let json = serde_json::to_string(&rename_function_data.code_asm)
    .unwrap_or_else(|_| "Erreur lors de la conversion en JSON".to_string());

    println!("{}", json);
    let mut formatted_code_asm = String::new();

    // Itérer sur chaque fonction dans la HashMap
    for (function_name, lines) in &rename_function_data.code_asm {
        formatted_code_asm.push_str(&format!("Fonction {}:\n", function_name));
        for line in lines {
            if line.len() == 2 {
                formatted_code_asm.push_str(&format!("{}: {}\n", line[0], line[1]));
            }
        }
        formatted_code_asm.push_str("\n"); // Ajoute un saut de ligne entre les fonctions
    }
    let code_c_json = serde_json::to_string(&rename_function_data.code_c)
    .unwrap_or_else(|_| "Erreur lors de la conversion en JSON".to_string());
    let full_prompt = RENAME_FUNCTION_PROMPT.replace("{rename_list}", &format_rename_data(&rename_function_data))
    .replace("{code_assembleur}", &formatted_code_asm)
    .replace("{code_decompile}", &code_c_json);

    match ask_chat_gpt(full_prompt, "rename").await {
        Ok(response) => response,
        Err(_) => "Erreur lors de la communication avec ChatGPT".to_string(),
    }
}


#[post("/renameVariable", data = "<rename_variable_data>")]
pub async fn rename_variable(rename_variable_data: Json<RenameRequest>) -> String {

    let json = serde_json::to_string(&rename_variable_data.code_asm)
    .unwrap_or_else(|_| "Erreur lors de la conversion en JSON".to_string());
    println!("{}", json);
    let mut formatted_code_asm = String::new();

    // Itérer sur chaque fonction dans la HashMap
    for (function_name, lines) in &rename_variable_data.code_asm {
        formatted_code_asm.push_str(&format!("Fonction {}:\n", function_name));
        for line in lines {
            if line.len() == 2 {
                formatted_code_asm.push_str(&format!("{}: {}\n", line[0], line[1]));
            }
        }
        formatted_code_asm.push_str("\n"); // Ajoute un saut de ligne entre les fonctions
    }
    let code_c_json = serde_json::to_string(&rename_variable_data.code_c)
    .unwrap_or_else(|_| "Erreur lors de la conversion en JSON".to_string());
    let full_prompt = RENAME_VARIABLE_PROMPT.replace("{rename_list}", &format_rename_data(&rename_variable_data)).replace("{code_assembleur}", &formatted_code_asm).replace("{code_decompile}", &code_c_json);

    match ask_chat_gpt(full_prompt, "rename").await {
        Ok(response) => response,
        Err(_) => "Erreur lors de la communication avec ChatGPT".to_string(),
    }
}

// Fonction pour formater les données de renommage
fn format_rename_data(rename_data: &Json<RenameRequest>) -> String {
    let mut formatted_data = String::new();
    for item in &rename_data.items {
        formatted_data.push_str(&format!("Type: {}, Nom actuel: {}\n", item.item_type, item.old_name));
    }
    formatted_data
}