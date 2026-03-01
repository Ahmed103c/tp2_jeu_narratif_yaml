use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

pub fn validate_story(story: &Story) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    let mut seen = HashSet::new();
    for scene in &story.scenes {
        if !seen.insert(scene.id.as_str()) {
            errors.push(format!("ID de scène dupliqué : '{}'", scene.id));
        }
    }

    let scene_ids: HashSet<&str> = story.scenes.iter()
        .map(|s| s.id.as_str())
        .collect();
      
    if !scene_ids.contains(story.start_scene.as_str()) {
        errors.push(format!(
            "start_scene '{}' n'existe pas parmi les scènes",
            story.start_scene
        ));
    }

    for scene in &story.scenes {
        match &scene.choices {
            Some(choices) => {
                for choice in choices {
                    if !scene_ids.contains(choice.next.as_str()) {
                        errors.push(format!(
                            "Scène '{}' : le choix '{}' pointe vers '{}' qui n'existe pas",
                            scene.id, choice.label, choice.next
                        ));
                    }
                }
            }
            None => {} 
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}