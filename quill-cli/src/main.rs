use quill_engine::Game;

fn main() {
    println!("Welcome to Quill!");
    println!("🪶🪶🪶🪶🪶🪶🪶🪶🪶🪶🪶🪶\n");

    let game = Game::new("Max".to_string(), "AJ".to_string());

    display_game_state(&game);
}

fn display_game_state(game: &Game) {
    println!("\n{}", "=".repeat(50));
    println!("Turn: {}", game.current_turn);
    println!("{}", "=".repeat(50));

    println!("\n👤 {}", game.player1.name);
    println!("\n👤 {}", game.player2.name);
}
