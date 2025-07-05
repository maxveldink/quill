#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player { name }
    }
}

#[derive(Debug)]
pub struct Game {
    pub player1: Player,
    pub player2: Player,
    pub current_turn: u8,
}

impl Game {
    pub fn new(player1_name: String, player2_name: String) -> Game {
        Game {
            player1: Player::new(player1_name),
            player2: Player::new(player2_name),
            current_turn: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_creation() {
        let player = Player::new("Max".to_string());

        assert_eq!(player.name, "Max");
    }

    #[test]
    fn test_game_creation() {
        let game = Game::new("Max".to_string(), "AJ".to_string());

        assert_eq!(game.player1.name, "Max");
        assert_eq!(game.player2.name, "AJ");
        assert_eq!(game.current_turn, 1);
    }
}
