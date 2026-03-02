use std::collections::HashSet;

use crate::story::Story;
use crate::game::*;

pub mod story;
pub mod game;


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

pub fn parse_command(line :&str) -> Result<Box<dyn GameCommand>,ParseError>{
    let mut parts = line.split_whitespace();
    let command = parts.next(); 
    let arg = parts.next();     

    match command {
        Some("look") =>Ok(Box::new(LookCommand)),
        Some("inventory")=>{Ok(Box::new(InventoryCommand))}
        Some("status")=>{Ok(Box::new(StatusCommand))}
        Some("quit")=>{Ok(Box::new(QuitCommand))}
        Some("choose") => {
            match arg {
                Some(n) => Ok(Box::new(ChooseCommand { n: n.parse().map_err(|_| ParseError::InvalidArg)? })),
                None => Err(ParseError::MissingValueForChooseCommand)
            }
        }
        None => Err(ParseError::MissingCommand),
        _ => Err(ParseError::UnknownCommand)
    }

}