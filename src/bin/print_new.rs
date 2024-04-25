use card_game::serialize::*;
use card_game::pdf::*;
use card_game::Card;

fn main() {
    let mut cards = serialize_all_cards("cards", false);
    cards.extend(serialize_all_cards("commanders", true));
    
    let mut names = Vec::<&str>::with_capacity(cards.len());
    for card in &cards {
        if names.contains(&card.name.as_str()) {
            println!("Multiple cards named {}", &card.name);            
        }
        else {
            names.push(&card.name);
        }
    }
    
    add_cards_to_pdf(&cards);
}