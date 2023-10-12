use card_game::serialize::*;
use card_game::pdf::*;
use card_game::Card;

fn main() {
    let cards = std::fs::read_to_string("cards.json").unwrap();
    let mut cards: Vec<Card> = serde_json::from_str(&cards).unwrap();
    let new_cards = serialize_all_cards("cards");

    for new_card in new_cards {
        if let Some(card) = cards.iter_mut().find(|c| c.name == new_card.name) {
            *card = new_card;
        }
        else {
            cards.push(new_card);
        }
    }

    serialize_to_json(&cards);

    let mut cards_to_print = Vec::<Card>::new();
    let lines = std::fs::read_to_string("print.txt").unwrap();
    let lines: Vec<&str> = lines.lines().collect();
    for card in cards {
        if lines.contains(&card.name.as_str()) {
            cards_to_print.push(card);
        }
    }

    add_cards_to_pdf(&cards_to_print, false);
}