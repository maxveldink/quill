#[cfg(test)]
use crate::cards::{Card, CardType, Classification, InkType, Rarity};

#[cfg(test)]
pub fn create_kida_atlantean_card() -> Card {
    Card::new(
        true,
        InkType::Amber,
        1,
        CardType::Character,
        "Kida".to_string(),
        "Atlantean".to_string(),
        vec![
            Classification::Storyborn,
            Classification::Hero,
            Classification::Princess,
        ],
        2,
        2,
        1,
        Rarity::Common,
    )
}

#[cfg(test)]
pub fn create_flounder_voice_of_reason_card() -> Card {
    Card::new(
        true,
        InkType::Sapphire,
        1,
        CardType::Character,
        "Flounder".to_string(),
        "Voice of Reason".to_string(),
        vec![Classification::Storyborn, Classification::Ally],
        2,
        2,
        1,
        Rarity::Common,
    )
}

#[cfg(test)]
pub fn create_goons_maleficents_underlings_card() -> Card {
    Card::new(
        true,
        InkType::Steel,
        1,
        CardType::Character,
        "Goons".to_string(),
        "Maleficent's Underlings".to_string(),
        vec![Classification::Storyborn, Classification::Ally],
        2,
        2,
        1,
        Rarity::Common,
    )
}
