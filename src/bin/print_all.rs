use card_game::serialize::*;
use card_game::pdf::*;
use card_game::Card;

fn get_cards_to_print() -> Vec<&str> {
    let names = Vec::<&str>::new();
    let entries = std::fs::read_dir(directory).expect("Could not find directory");
    for entry in entries {
        if let Result::Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if ext == "txt" {
                    println!("Processing file: {}", &entry.path().to_str().unwrap());
                    let string = std::fs::read_to_string(entry.path());
                    if let Ok(string) = string {
                        cards.append(&mut parse_txt_string(string));
                    }
                    else {
                        println!("Error reading file");
                    }
                }
            }
        }
    }

    names
}

fn main() {
    let new_cards = serialize_all_cards();
    let cards = std::fs::read_to_string("cards.json").unwrap();
    let mut cards: Vec<Card> = serde_json::from_str(&cards).unwrap();

    for new_card in new_cards {
        if let Some(card) = cards.iter_mut().find(|c| c.name == new_card.name) {
            *card = new_card;
        }
        else {
            cards.push(new_card);
        }
    }

    serialize_to_json(&cards);

    add_all_cards_to_pdf(&cards, false);
}