use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub label: String,
    pub next: String,
    pub required_item: Option<String>, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Scene {
    pub id: String,
    pub title: String,
    pub text: String,
    pub choices: Option<Vec<Choice>>,
    pub ending: Option<String>,       
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Story {
    pub start_scene: String,
    pub initial_hp: u32,
    pub scenes: Vec<Scene>,
}