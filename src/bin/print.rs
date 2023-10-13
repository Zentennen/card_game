use card_game::serialize::*;
use card_game::pdf::*;
use card_game::Card;

fn main() {
    let mut cards: Vec<Card>;
    if let Ok(cards_json) = std::fs::read_to_string("cards.json") {
        cards = serde_json::from_str(&cards_json).unwrap();
    }
    else {
        cards = Vec::with_capacity(1000);
    }
    let mut new_cards = serialize_all_cards("cards", false);
    new_cards.extend(serialize_all_cards("commanders", true));

    for new_card in new_cards {
        if let Some(card) = cards.iter_mut().find(|c| c.name == new_card.name && c.commander == new_card.commander) {
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
        if lines.contains(&card.name.as_str().trim()) {
            cards_to_print.push(card);
        }
    }
    
    add_cards_to_pdf(&cards_to_print);
}