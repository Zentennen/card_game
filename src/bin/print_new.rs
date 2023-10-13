use card_game::serialize::*;
use card_game::pdf::*;

fn main() {
    let mut cards = serialize_all_cards("cards", false);
    cards.extend(serialize_all_cards("commanders", true));
    add_cards_to_pdf(&cards);
}