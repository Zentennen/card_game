use card_game::serialize::*;
use card_game::to_pdf::*;

fn main() {
    let cards = serialize_all_cards();
    add_all_cards_to_pdf(&cards);
}