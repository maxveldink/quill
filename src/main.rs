mod cards;

fn main() {
    use cards::*;

    let cards = [Card::new(
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
    )];

    println!("{}", cards[0]);
}
