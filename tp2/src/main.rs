use tp2::Story;
use std::fs;
use tp2::validate_story;

fn main() {
    let content = fs::read_to_string("story1.yaml")
        .expect("Impossible de lire story1.yaml");

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
}