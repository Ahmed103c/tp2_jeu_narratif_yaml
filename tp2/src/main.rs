use tp2::{parse_command, story::Story,game::*};
use std::fs;
use tp2::validate_story;
use std::io;

fn main() {
    let content = fs::read_to_string("story.yaml")
        .expect("Impossible de lire story.yaml");

    let scene: Story = serde_yaml::from_str(&content)
        .expect("Erreur de parsing YAML");

    println!("{:#?}", scene);

    match validate_story(&scene) {
        Ok(()) => println!("Scénario valide !\n"),
        Err(errors) => {
            eprintln!("Scénario invalide :");
            for e in errors {
                eprintln!("  - {}", e);
            }
            std::process::exit(1);
        }
    }
    let mut state = GameState {
        current_scene_id: scene.start_scene.clone(),
        hp: scene.initial_hp,
        inventory: Vec::new(),
        status: GameStatus::Playing,
    };

    loop {
        println!("Entrez une commande (look, choose N, inventory, status, quit) :");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        let input = input.trim();

        match parse_command(input) {
            Err(ParseError::UnknownCommand) => println!("Commande inconnue !"),
            Err(ParseError::MissingCommand) => println!("Entrez une commande !"),
            Err(ParseError::MissingValueForChooseCommand) => println!("choose nécessite un numéro ex: choose 2"),
            Err(ParseError::InvalidArg) => println!("Argument invalide !"),
           Ok(command) => {
                match run_command(command, &scene, &mut state) {
                    Ok(CommandOutcome::Display(msg)) => println!("{}", msg),
                    Ok(CommandOutcome::Quit) => { println!("Au revoir !"); break; }
                    Ok(CommandOutcome::Win) => { println!("Vous avez gagné !"); break; }
                    Ok(CommandOutcome::GameOver) => { println!("Game Over !"); break; }
                    Ok(CommandOutcome::SceneChanged) => {
                        if let Ok(CommandOutcome::Display(msg)) = LookCommand.execute(&scene, &mut state) {
                            println!("{}", msg);
                        }
                    }
                    Err(GameError::SceneNotFound) => eprintln!("Scène introuvable !"),
                    Err(GameError::NoChoicesAvailable) => println!("Pas de choix disponibles."),
                    Err(GameError::InvalidChoice) => println!("Ce choix n'existe pas !"),
                }
            }
        }
    }
}
