use tp2::Story;
use std::fs;

fn main() {
    let content = fs::read_to_string("story1.yaml")
        .expect("Impossible de lire story1.yaml");

    let scene: Story = serde_yaml::from_str(&content)
        .expect("Erreur de parsing YAML");

    println!("{:#?}", scene);
}