use clap::{Parser, Subcommand};
use quill_engine::Game;
use quill_primitives::decks::{Deck, DeckFormat, StandardDeckFormat, TestingDeckFormat};
use serde_json;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "quill")]
#[command(about = "A Lorcana toolbox for crafting experiences for Illumineers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Game {
        #[arg(long, default_value = "Player 1")]
        player1: String,
        #[arg(long, default_value = "Player 2")]
        player2: String,
    },
    /// Validate a deck from JSON file
    Validate {
        #[arg(value_name = "FILE")]
        file: PathBuf,
        /// Deck format to validate against (testing or standard)
        #[arg(long, default_value = "standard")]
        format: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Game { player1, player2 } => {
            run_game(player1, player2);
        }
        Commands::Validate { file, format } => {
            validate_deck(file, format);
        }
    }
}

fn run_game(player1: &str, player2: &str) {
    println!("Welcome to Quill!");
    println!("🪶🪶🪶🪶🪶🪶🪶🪶🪶🪶🪶🪶\n");

    let game = Game::new(player1.to_string(), player2.to_string());

    display_game_state(&game);
}

fn validate_deck(file_path: &PathBuf, format: &str) {
    println!("Validating deck from: {}", file_path.display());

    // Read and parse the JSON file
    let json_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let deck: Deck = match serde_json::from_str(&json_content) {
        Ok(deck) => deck,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            std::process::exit(1);
        }
    };

    // Choose the format validator
    let validator: Box<dyn DeckFormat> = match format.to_lowercase().as_str() {
        "testing" => Box::new(TestingDeckFormat),
        "standard" => Box::new(StandardDeckFormat),
        _ => {
            eprintln!(
                "Unknown format: {}. Supported formats: testing, standard",
                format
            );
            std::process::exit(1);
        }
    };

    // Validate the deck
    match validator.validate(&deck) {
        Ok(()) => {
            println!("✅ Deck is valid for {} format!", format);
            display_deck_info(&deck);
        }
        Err(e) => {
            println!("❌ Deck validation failed: {}", e);
            display_deck_info(&deck);
            std::process::exit(1);
        }
    }
}

fn display_deck_info(deck: &Deck) {
    println!("\n📊 Deck Information:");
    println!("Total cards: {}", deck.card_count());
    println!("Inks: {:?}", deck.inks());

    let breakdown = deck.card_breakdown();
    println!("\n📋 Card breakdown:");
    for (card, count) in breakdown {
        println!("  {}x {}", count, card);
    }
}

fn display_game_state(game: &Game) {
    println!("\n{}", "=".repeat(50));
    println!("Turn: {}", game.current_turn);
    println!("{}", "=".repeat(50));

    println!("\n👤 {}", game.player1.name);
    println!("\n👤 {}", game.player2.name);
}
