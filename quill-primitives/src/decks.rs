use crate::cards::{Card, InkType};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        Self { cards: Vec::new() }
    }

    pub fn with_cards(cards: Vec<Card>) -> Self {
        Self { cards }
    }

    pub fn card_count(&self) -> usize {
        self.cards.len()
    }

    pub fn inks(&self) -> Vec<InkType> {
        self.cards
            .iter()
            .map(|card| card.ink_type.clone())
            .unique()
            .sorted()
            .collect()
    }

    pub fn card_breakdown(&self) -> HashMap<&Card, u8> {
        let mut card_counts = HashMap::new();
        for card in &self.cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        card_counts
    }
}

impl Default for Deck {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DeckFormat {
    fn validate(&self, deck: &Deck) -> Result<(), DeckValidationError>;
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum DeckValidationError {
    #[error("Deck needs at least {required} cards, but has {actual}")]
    InsufficientCards { required: usize, actual: usize },
    #[error("Deck needs at most {max} inks, but has {actual:?}")]
    TooManyInks { max: usize, actual: Vec<InkType> },
    #[error("Too many copies of {card}: {count}")]
    TooManyCopies { card: Card, count: u8 },
}

pub struct TestingDeckFormat;
impl DeckFormat for TestingDeckFormat {
    fn validate(&self, deck: &Deck) -> Result<(), DeckValidationError> {
        if deck.card_count() >= 10 {
            Ok(())
        } else {
            Err(DeckValidationError::InsufficientCards {
                required: 10,
                actual: deck.card_count(),
            })
        }
    }
}

pub struct StandardDeckFormat;
impl DeckFormat for StandardDeckFormat {
    fn validate(&self, deck: &Deck) -> Result<(), DeckValidationError> {
        if deck.card_count() < 60 {
            return Err(DeckValidationError::InsufficientCards {
                required: 60,
                actual: deck.card_count(),
            });
        }

        if deck.inks().len() > 2 {
            return Err(DeckValidationError::TooManyInks {
                max: 2,
                actual: deck.inks(),
            });
        }

        for (card, count) in deck.card_breakdown() {
            if count > 4 {
                return Err(DeckValidationError::TooManyCopies {
                    card: card.clone(),
                    count,
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{
        create_flounder_voice_of_reason_card, create_goons_maleficents_underlings_card,
        create_kida_atlantean_card,
    };

    use super::*;

    #[test]
    fn test_deck_creation() {
        let deck = Deck::new();
        assert_eq!(deck.card_count(), 0);
    }

    #[test]
    fn test_deck_with_cards() {
        let deck = Deck::with_cards(vec![create_kida_atlantean_card()]);
        assert_eq!(deck.card_count(), 1);
    }

    #[test]
    fn test_deck_inks_monocolor() {
        let deck = Deck::with_cards(vec![create_kida_atlantean_card(); 2]);
        assert_eq!(deck.inks(), vec![InkType::Amber]);
    }

    #[test]
    fn test_deck_inks_with_dual_inks() {
        let deck = Deck::with_cards(vec![
            create_flounder_voice_of_reason_card(),
            create_kida_atlantean_card(),
        ]);
        assert_eq!(deck.inks(), vec![InkType::Amber, InkType::Sapphire]);
    }

    #[test]
    fn test_deck_inks_with_many_cards() {
        let deck = Deck::with_cards(vec![
            create_flounder_voice_of_reason_card(),
            create_kida_atlantean_card(),
            create_flounder_voice_of_reason_card(),
            create_goons_maleficents_underlings_card(),
            create_kida_atlantean_card(),
        ]);
        assert_eq!(
            deck.inks(),
            vec![InkType::Amber, InkType::Sapphire, InkType::Steel]
        );
    }

    #[test]
    fn test_deck_card_breakdown() {
        let deck = Deck::with_cards(vec![
            create_kida_atlantean_card(),
            create_flounder_voice_of_reason_card(),
            create_goons_maleficents_underlings_card(),
            create_kida_atlantean_card(),
        ]);

        let breakdown = deck.card_breakdown();

        assert_eq!(breakdown.len(), 3);
        assert_eq!(breakdown[&create_kida_atlantean_card()], 2);
        assert_eq!(breakdown[&create_flounder_voice_of_reason_card()], 1);
        assert_eq!(breakdown[&create_goons_maleficents_underlings_card()], 1);
    }

    #[test]
    fn test_testing_deck_format_with_no_cards() {
        let deck = Deck::new();
        let result = TestingDeckFormat.validate(&deck);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DeckValidationError::InsufficientCards {
                required: 10,
                actual: 0,
            }
        );
    }

    #[test]
    fn test_testing_deck_format_with_10_cards() {
        let deck = Deck::with_cards(vec![create_kida_atlantean_card(); 10]);
        let result = TestingDeckFormat.validate(&deck);
        assert!(result.is_ok());
    }

    #[test]
    fn test_standard_deck_format_with_no_cards_returns_error() {
        let deck = Deck::new();
        let result = StandardDeckFormat.validate(&deck);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DeckValidationError::InsufficientCards {
                required: 60,
                actual: 0,
            }
        );
    }

    #[test]
    fn test_standard_deck_format_with_more_than_2_inks_returns_error() {
        let mut cards = vec![create_flounder_voice_of_reason_card(); 58];
        cards.push(create_goons_maleficents_underlings_card());
        cards.push(create_kida_atlantean_card());
        let deck = Deck::with_cards(cards);

        let result = StandardDeckFormat.validate(&deck);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DeckValidationError::TooManyInks {
                max: 2,
                actual: vec![InkType::Amber, InkType::Sapphire, InkType::Steel],
            }
        );
    }

    #[test]
    fn test_standard_deck_format_with_60_cards_but_more_than_4copies_of_one_card_returns_error() {
        let deck = Deck::with_cards(vec![create_kida_atlantean_card(); 60]);
        let result = StandardDeckFormat.validate(&deck);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DeckValidationError::TooManyCopies {
                card: create_kida_atlantean_card(),
                count: 60,
            }
        );
    }
}
