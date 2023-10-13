use card_game::serialize::*;
use card_game::pdf::*;
use card_game::Card;

fn main() {
    let cards = std::fs::read_to_string("cards.json").unwrap();
    let mut cards: Vec<Card> = serde_json::from_str(&cards).unwrap();
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
    add_cards_to_pdf(&cards);
}