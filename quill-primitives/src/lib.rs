use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum CardType {
    Character,
    Item,
    Location,
    Action,
    Song,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Classification {
    Storyborn,
    Hero,
    Princess,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    SuperRare,
    Legendary,
    Enchanted,
}

#[derive(Debug, PartialEq, Clone)]
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
    use super::*;

    fn create_card() -> Card {
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
        let card = create_card();

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
        let card = create_card();
        assert_eq!(format!("{}", card), "🟡 (1) Kida-Atlantean 2⚔️ | 2🛡️ | 1✨");
    }
}
