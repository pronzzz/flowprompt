use crate::config;
use serde::{Deserialize, Serialize};
use std::fs;
// use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub id: String,
    pub alias: String,
    pub description: String,
    pub tags: Vec<String>,
    pub template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptFile {
    pub prompts: Vec<Prompt>,
}

pub fn load_prompts() -> Vec<Prompt> {
    let path = config::get_config_dir().join("prompts.json");
    if !path.exists() {
        return vec![];
    }

    let content = fs::read_to_string(path).unwrap_or_else(|_| "{}".to_string());
    let data: PromptFile = serde_json::from_str(&content).unwrap_or(PromptFile { prompts: vec![] });
    data.prompts
}

pub fn save_prompt(prompt: Prompt) -> Result<(), String> {
    let mut prompts = load_prompts();
    prompts.push(prompt);

    let path = config::get_config_dir().join("prompts.json");
    let data = PromptFile { prompts };
    let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;

    fs::write(path, json).map_err(|e| e.to_string())
}
