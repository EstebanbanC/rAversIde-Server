use rocket::serde::{json::Json, Deserialize, Serialize};


use crate::utils::ask_chat_gpt;

// Définition des structures pour la requête et la réponse de renommage
#[derive(Deserialize)]
pub struct RenameRequest {
    pub items: Vec<RenameItem>,
}

#[derive(Deserialize)]
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

Format de réponse attendu :
{
  "rename": [
    ["type", "ancien_nom", "nouveau_nom"]
    // Autres éléments ici
  ]
}"#;


pub const RENAME_VARIABLE_PROMPT: &str = r#"Suggère des noms plus descriptifs pour les variables des fonctions suivantes en conservant leur signification initiale. Le renommage doit améliorer la lisibilité et la compréhension du code. Pour chaque élément, fournis des noms de variables plus approprié.

Liste des éléments à renommer :
{rename_list}

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
    let formatted_data = format_rename_data(&rename_function_data);
    let full_prompt = RENAME_FUNCTION_PROMPT.replace("{rename_list}", &formatted_data);

    match ask_chat_gpt(full_prompt, "rename").await {
        Ok(response) => response,
        Err(_) => "Erreur lors de la communication avec ChatGPT".to_string(),
    }
}

#[post("/renameVariable", data = "<rename_variable_data>")]
pub async fn rename_variable(rename_variable_data: Json<RenameRequest>) -> String {
    let formatted_data = format_rename_data(&rename_variable_data);
    let full_prompt = RENAME_VARIABLE_PROMPT.replace("{rename_list}", &formatted_data);

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