#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::*;
use extstd::*;

fn find_next_property_position(string: &str, start: usize) -> Option<(char, usize)> {
    for (p, b) in string.bytes().enumerate().skip(start) {
        if !string.is_char_boundary(p) {
            continue;
        }
        let mut chars = string[p..].chars();
        let c = chars.next().unwrap();

        let c2 = chars.next();
        if let None = c2 { continue; }
        let c2 = c2.unwrap();
        if c2 != ';' { continue; }

        return Some((c, p));
    }

    None
}

fn initialize_card(card: &mut Card, s: &str) {
    let s = s.trim();
    let strings = s.split(',');
    for string in strings {
        let string = string.trim();
        if string.is_empty() {
            continue;
        }

        let space = string.find(' ');
        if let Some(space) = space {
            let name = string[..space].trim();
            
            if attributes.contains(&name) {
                let value = string[space..].trim();
                card.attributes.push(Attribute{ n: name.to_string(), v: value.to_string() });
            }
            else {
                card.types.push(name.to_string());
            }
        }
        else {
            card.types.push(string.to_string());
        }
    }
}

fn process_card(card: &mut Card, s: &str) -> Maybe {
    let prev = find_next_property_position(s, 0);
    if let Some((mut prop, mut offset)) = prev {
        initialize_card(card, &s[..offset]);
        while let Some(next) = find_next_property_position(s, offset + 1) {
            let substr = &s[offset..next.1];
            match next.0 {
                'A' => card.abiilities.push(substr[2..].trim().to_string()),
                'R' => card.reactions.push(substr[2..].trim().to_string()),
                'T' => card.traits.push(substr[2..].trim().to_string()),
                _ => anyhow::bail!("Invalid property char: {}", next.0)
            }
            (prop, offset) = next;
        }

        let substr = &s[offset..];
        match prop {
            'A' => card.abiilities.push(substr.to_string()),
            'R' => card.reactions.push(substr.to_string()),
            'T' => card.traits.push(substr.to_string()),
            _ => anyhow::bail!("Invalid property char: {}", prop)
        }

        ok
    }
    else {
        initialize_card(card, s);
        ok
    }
}

fn process_card_strings(card_strings: Vec<String>) -> Vec<Card> {
    let mut cards = Vec::<Card>::with_capacity(card_strings.len());

    for card_string in card_strings {
        let mut name = true;
        for line in card_string.lines() {
            if name {
                cards.push(Card::with_name(line.to_string()));
                name = false;
            }
            else {
                let card = cards.last_mut().unwrap();
                process_card(card, line).expect(&format!("EXPECT ERR: Failed to process card: '{}'", line));
                name = true;
            }
        }
    }

    cards
}

fn split_string_by_cards(string: &str) -> Vec<String> {
    let mut card_strings = Vec::<String>::with_capacity(string.lines().count());
    
    let mut card_string = String::with_capacity(1000);
    for line in string.lines() {
        if line.is_empty() {
            if !card_string.is_empty() {
                card_strings.push(card_string.clone());
                card_string.clear();
            }
        }
        else if card_string.is_empty() {
            card_string.push_str(line);
            card_string.push('\n');
        }
        else {
            card_string.push_str(line);
            card_string.push(' ');
        }
    }
    card_strings.push(card_string.clone());

    card_strings
}

fn parse_txt_string(string: String) -> Vec<Card> {
    let card_strings = split_string_by_cards(&string);
    let cards = process_card_strings(card_strings);

    return cards;
}

pub fn serialize_to_json(cards: &Vec<Card>) {
    let cards = serde_json::to_string(cards).unwrap();
    std::fs::write("./cards.json", cards).unwrap();
}

pub fn serialize_all_cards(directory: &str, commanders: bool) -> Vec<Card> {
    let mut cards = Vec::<Card>::with_capacity(10000);

    let entries = std::fs::read_dir(directory).expect(directory);
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

    for card in cards.iter_mut() {
        card.attributes.sort_by(|a, b| { a.n.cmp(&b.n) });
        card.commander = commanders;
    }

    cards
}