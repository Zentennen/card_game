use card_game::serialize::*;
use card_game::pdf::*;

fn main() {
    let cards = serialize_all_cards("cards");
    let commanders = serialize_all_cards("commanders");
    add_all_cards_to_pdf(&cards, false);
    add_all_cards_to_pdf(&commanders, true);
}