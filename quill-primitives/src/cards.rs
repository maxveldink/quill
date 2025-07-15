use std::{fmt, hash::Hasher};

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub enum CardType {
    Character,
    Item,
    Location,
    Action,
    Song,
}

#[derive(
    Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum InkType {
    Amber,
    Amethyst,
    Emerald,
    Ruby,
    Sapphire,
    Steel,
}

impl fmt::Display for InkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InkType::Amber => write!(f, "🟡"),
            InkType::Amethyst => write!(f, "🟣"),
            InkType::Emerald => write!(f, "🟢"),
            InkType::Ruby => write!(f, "🔴"),
            InkType::Sapphire => write!(f, "🔵"),
            InkType::Steel => write!(f, "⚪"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Classification {
    Storyborn,
    Hero,
    Princess,
    Ally,
}

#[derive(Debug, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    SuperRare,
    Legendary,
    Enchanted,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub inkable: bool,
    pub ink_type: InkType,
    pub cost: u8,
    pub card_type: CardType,
    pub name: String,
    pub version_name: String,
    pub classifications: Vec<Classification>,
    pub strength: u8,
    pub willpower: u8,
    pub lore_value: u8,
    pub rarity: Rarity,
}

impl Card {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        inkable: bool,
        ink_type: InkType,
        cost: u8,
        card_type: CardType,
        name: String,
        version_name: String,
        classifications: Vec<Classification>,
        strength: u8,
        willpower: u8,
        lore_value: u8,
        rarity: Rarity,
    ) -> Card {
        Card {
            inkable,
            ink_type,
            cost,
            card_type,
            name,
            version_name,
            classifications,
            strength,
            willpower,
            lore_value,
            rarity,
        }
    }
}

impl core::hash::Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.version_name.hash(state);
    }
}

impl core::cmp::Eq for Card {}

impl core::cmp::PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.version_name == other.version_name
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cost_string = if self.inkable {
            format!("({})", self.cost)
        } else {
            format!("({})", self.cost + 1)
        };
        write!(
            f,
            "{} {} {}-{} {}⚔️ | {}🛡️ | {}✨",
            self.ink_type,
            cost_string,
            self.name,
            self.version_name,
            self.strength,
            self.willpower,
            self.lore_value
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::create_kida_atlantean_card;

    use super::*;

    #[test]
    fn test_ink_type_display() {
        assert_eq!(format!("{}", InkType::Amber), "🟡");
        assert_eq!(format!("{}", InkType::Amethyst), "🟣");
        assert_eq!(format!("{}", InkType::Emerald), "🟢");
        assert_eq!(format!("{}", InkType::Ruby), "🔴");
        assert_eq!(format!("{}", InkType::Sapphire), "🔵");
        assert_eq!(format!("{}", InkType::Steel), "⚪");
    }

    #[test]
    fn test_card_creation() {
        let card = create_kida_atlantean_card();

        assert_eq!(card.name, "Kida");
        assert_eq!(card.version_name, "Atlantean");
        assert_eq!(card.inkable, true);
        assert_eq!(card.ink_type, InkType::Amber);
        assert_eq!(card.cost, 1);
        assert_eq!(card.card_type, CardType::Character);
        assert_eq!(
            card.classifications,
            vec![
                Classification::Storyborn,
                Classification::Hero,
                Classification::Princess
            ]
        );
        assert_eq!(card.strength, 2);
        assert_eq!(card.willpower, 2);
        assert_eq!(card.lore_value, 1);
        assert_eq!(card.rarity, Rarity::Common);
    }

    #[test]
    fn test_card_display() {
        let card = create_kida_atlantean_card();
        assert_eq!(format!("{}", card), "🟡 (1) Kida-Atlantean 2⚔️ | 2🛡️ | 1✨");
    }

    #[test]
    fn test_card_hash_implementation() {
        use std::collections::HashSet;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Test that hash is consistent for same card
        let card1 = create_kida_atlantean_card();
        let card2 = create_kida_atlantean_card();

        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();

        card1.hash(&mut hasher1);
        card2.hash(&mut hasher2);

        assert_eq!(
            hasher1.finish(),
            hasher2.finish(),
            "Same cards should have same hash"
        );

        // Test that hash only depends on name and version_name
        let mut card_with_different_stats = create_kida_atlantean_card();
        card_with_different_stats.strength = 99;
        card_with_different_stats.willpower = 99;
        card_with_different_stats.lore_value = 99;
        card_with_different_stats.cost = 99;
        card_with_different_stats.ink_type = InkType::Ruby;

        let mut hasher3 = DefaultHasher::new();
        card_with_different_stats.hash(&mut hasher3);

        assert_eq!(
            hasher1.finish(),
            hasher3.finish(),
            "Hash should only depend on name and version_name"
        );

        // Test that different cards have different hashes
        let different_card = Card::new(
            true,
            InkType::Sapphire,
            2,
            CardType::Character,
            "Different Name".to_string(),
            "Different Version".to_string(),
            vec![Classification::Ally],
            3,
            3,
            2,
            Rarity::Uncommon,
        );

        let mut hasher4 = DefaultHasher::new();
        different_card.hash(&mut hasher4);

        assert_ne!(
            hasher1.finish(),
            hasher4.finish(),
            "Different cards should have different hashes"
        );

        // Test HashSet behavior with custom hash
        let mut card_set = HashSet::new();
        card_set.insert(card1);
        card_set.insert(card2);
        card_set.insert(card_with_different_stats);
        card_set.insert(different_card);

        // Should only have 2 unique cards (card1/card2/card_with_different_stats are the same, different_card is unique)
        assert_eq!(
            card_set.len(),
            2,
            "HashSet should deduplicate based on hash implementation"
        );
    }
}
