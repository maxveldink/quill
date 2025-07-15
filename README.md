# Quill

🚧🚧 Under Construction 🚧🚧

An open set of Lorcana primitives for crafting experiences for Illumineers.

## CLI Commands

The Quill CLI provides several commands for working with Lorcana decks and games:

### Game Command

Start a new game between two players:

```bash
cargo run --bin quill-cli -- game --player1 "Player 1" --player2 "Player 2"
```

### Validate Command

Validate a deck from a JSON file against different formats:

```bash
cargo run --bin quill-cli -- validate <deck-file.json> --format <format>
```

**Supported formats:**
- `testing`: Requires at least 10 cards
- `standard`: Requires 60+ cards, max 2 inks, max 4 copies of any card

**Example:**
```bash
cargo run --bin quill-cli -- validate my-deck.json --format standard
```

### JSON Deck Format

Decks should be stored in JSON format with the following structure:

```json
{
  "cards": [
    {
      "inkable": true,
      "ink_type": "Amber",
      "cost": 1,
      "card_type": "Character",
      "name": "Card Name",
      "version_name": "Version",
      "classifications": ["Storyborn", "Hero"],
      "strength": 2,
      "willpower": 2,
      "lore_value": 1,
      "rarity": "Common"
    }
  ]
}
```

**Supported ink types:** Amber, Amethyst, Emerald, Ruby, Sapphire, Steel

**Supported card types:** Character, Item, Location, Action, Song

**Supported classifications:** Storyborn, Hero, Princess, Ally

**Supported rarities:** Common, Uncommon, Rare, SuperRare, Legendary, Enchanted
