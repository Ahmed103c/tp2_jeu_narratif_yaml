use crate::Story;

pub enum GameStatus {
    Playing,
    Win,
    GameOver,
}
pub struct GameState{
    pub current_scene_id: String,
    pub hp: u32,
    pub inventory: Vec<String>,
    pub status: GameStatus,
}
pub enum GameError {
    SceneNotFound,
    NoChoicesAvailable,
    InvalidChoice,
}

pub enum CommandOutcome {
    Display(String),  
    Quit,             
    SceneChanged,     
}

pub trait GameCommand {
    fn execute(&self, scenario:&Story, state: &mut GameState) -> Result<CommandOutcome, GameError>;
}

pub struct LookCommand;
pub struct ChooseCommand { pub n: u32 }
pub struct InventoryCommand;
pub struct StatusCommand;
pub struct QuitCommand;

impl GameCommand for LookCommand {
    fn execute(&self, scenario:&Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        let current_scene = scenario.scenes.iter()
                .find(|s| s.id == state.current_scene_id);

        match current_scene {
            Some(current_scene) => {
                let mut display = current_scene.text.clone();
    
                if let Some(choices) = &current_scene.choices {
                    display.push_str("\n\nChoix disponibles :");
                    for (i, choice) in choices.iter().enumerate() {
                        display.push_str(&format!("\n  {}. {}", i + 1, choice.label));
                    }
                }
                
                Ok(CommandOutcome::Display(display))
            }
            None => {
                Err(GameError::SceneNotFound)
            }
        }
    }
}

impl GameCommand for ChooseCommand {
    fn execute(&self, scenario: &Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        // 1. Trouver la scène courante
        let current_scene = scenario.scenes.iter()
            .find(|s| s.id == state.current_scene_id)
            .ok_or(GameError::SceneNotFound)?;

        // 2. Récupérer les choix
        let choices = current_scene.choices.as_ref()
            .ok_or(GameError::NoChoicesAvailable)?;

        // 3. Récupérer le choix demandé
        let choice = choices.get((self.n - 1) as usize)
            .ok_or(GameError::InvalidChoice)?;

        // 4. Vérifier l'objet requis
        if let Some(required_item) = &choice.required_item {
            if !state.inventory.contains(required_item) {
                return Ok(CommandOutcome::Display(
                    format!("Vous n'avez pas l'objet requis : {}", required_item)
                ));
            }
        }
        // 5. Changer de scène
        state.current_scene_id = choice.next.clone();
        Ok(CommandOutcome::SceneChanged)
    }
}

impl GameCommand for InventoryCommand {
    fn execute(&self, scenario:&Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        let inventory_list = if state.inventory.is_empty() {
            "Votre inventaire est vide.".to_string()
        } else {
            format!("Votre inventaire : {}", state.inventory.join(", "))
        };
        Ok(CommandOutcome::Display(inventory_list))
    }
}
impl GameCommand for StatusCommand {
    fn execute(&self, scenario:&Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        Ok(CommandOutcome::Display(format!("Scene : {} ,HP : {}", scenario.scenes.iter().find(|s| s.id == state.current_scene_id).map(|s| s.title.clone()).unwrap_or_else(|| "Inconnue".into()), state.hp)))
    }
}
impl GameCommand for QuitCommand {
    fn execute(&self, scenario:&Story, state: &mut GameState) -> Result<CommandOutcome, GameError> {
        Ok(CommandOutcome::Quit)
    }
}

pub enum ParseError {
    UnknownCommand,
    MissingCommand,
    MissingValueForChooseCommand,
    InvalidArg,
}

