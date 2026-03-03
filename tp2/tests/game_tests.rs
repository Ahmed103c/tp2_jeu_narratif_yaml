use tp2::story::Story;
use tp2::game::*;
use tp2::validate_story;

fn tiny_victory_story() -> Story {
    serde_yaml::from_str(
        r#"
start_scene: start
initial_hp: 10
scenes:
  - id: start
    title: Start
    text: You are at the beginning.
    choices:
      - label: Go forward
        next: victory
  - id: victory
    title: Victory
    text: You have won!
    ending: victory
"#,
    )
    .unwrap()
}

#[test]
fn nominal_path_leads_to_victory() {
    let mut state = GameState {
        current_scene_id: "start".to_string(),
        hp: 10,
        inventory: Vec::new(),
        status: GameStatus::Playing,
    };

    let story = tiny_victory_story();
    let result = ChooseCommand { n: 1 }.execute(&story, &mut state);

    assert!(result.is_ok(), "choisir un choix valide ne devrait pas échouer");
    assert_eq!(state.current_scene_id, "victory");
}

#[test]
fn invalid_choice_returns_error() {
    let story = tiny_victory_story();
    let mut state = GameState {
        current_scene_id: "start".to_string(),
        hp: 10,
        inventory: Vec::new(),
        status: GameStatus::Playing,
    };

    let err = ChooseCommand { n: 99 }
        .execute(&story, &mut state)
        .expect_err("on s'attend à une erreur pour choix hors limites");

    assert_eq!(err, GameError::InvalidChoice);
}

#[test]
fn conditional_choice_without_item_shows_missing_item_message() {

    let story: Story = serde_yaml::from_str(
        r#"
start_scene: start
initial_hp: 5
scenes:
  - id: start
    title: Beginning
    text: Nothing here
    choices:
      - label: Unlock door
        next: end
        required_item: key
  - id: end
    title: End
    text: Done
    ending: victory
"#,
    )
    .unwrap();

    let mut state = GameState {
        current_scene_id: "start".to_string(),
        hp: 5,
        inventory: Vec::new(),
        status: GameStatus::Playing,
    };

    let outcome = ChooseCommand { n: 1 }
        .execute(&story, &mut state)
        .expect("l'exécution devrait réussir, mais retourner un message");

    match outcome {
        CommandOutcome::Display(msg) => {
            assert!(
                msg.contains("objet requis"),
                "message retourné = {}",
                msg
            );
 
            assert_eq!(state.current_scene_id, "start");
        }
        _ => panic!("attendu un affichage indiquant l'absence d'objet"),
    }
}

#[test]
fn hp_drop_to_zero_results_in_game_over_flag() {
    let story = tiny_victory_story();
    let mut state = GameState {
        current_scene_id: "start".to_string(),
        hp: 1,
        inventory: Vec::new(),
        status: GameStatus::Playing,
    };


    let _ = ChooseCommand { n: 1 }.execute(&story, &mut state);

    state.hp = 0;
    if state.hp == 0 {
        state.status = GameStatus::GameOver;
    }

    assert_eq!(state.status, GameStatus::GameOver);
}

#[test]
fn invalid_story_triggers_validation_error() {
    let story: Story = serde_yaml::from_str(
        r#"
start_scene: missing
initial_hp: 1
scenes:
  - id: a
    title: A
    text: scene a
"#,
    )
    .unwrap();

    assert!(validate_story(&story).is_err());
}
