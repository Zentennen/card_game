#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use crate::*;
use extstd::*;

pub const nl_indicator_char: char = '$';
pub const optional_indicator_char: char = '*';

fn find_next_property_position(string: &str, start: usize) -> Option<(PropertyType, usize)> {
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

        match c {
            'A' => return Some((PropertyType::action, p)),
            'T' => return Some((PropertyType::triggered, p)),
            'P' => return Some((PropertyType::passive, p)),
            _ => continue
        }
    }

    None
}

fn parse_attributes(attributes: &mut Vec<Attribute>, s: &str) {
    let s = s.trim();
    let attribute_strings = s.split(',');
    for attribute_string in attribute_strings {
        let attribute_string = attribute_string.trim();
        if attribute_string.is_empty() {
            continue;
        }
        let name_end = attribute_string.find(|c: char| !c.is_alphabetic() && c != ' ' && c != '¤' && c != '(' && c != ')');
        let name;
        let subattribute_part;
        if let Some(name_end) = name_end {
            name = attribute_string[..name_end].trim();
            subattribute_part = attribute_string[name_end..].trim();
        }
        else {
            name = attribute_string;
            subattribute_part = "";
        }
        let name = name.trim();
        let mut attribute = Attribute::with_name(name);
        if subattribute_part != "" {
            let error_string = format!("ERROR: Failed to parse subattribute '{}' as part of attribute '{}'", subattribute_part, attribute_string);
            let sub = str::parse::<f64>(subattribute_part).expect(&error_string);
            attribute.f.push(sub);
        }
        attributes.push(attribute);
    }
}

fn parse_action(s: &str) -> Res<Property> {
    let parts: Vec<&str> = s.split(";").collect();
    let mut current = parts.len() - 1;
    let mut property = Property::with_effect(parts[current].trim());

    current -= 1;
    if current <= 0 { return Ok(property); }

    parse_attributes(&mut property.attr, parts[current].trim());
    Ok(property)
}

fn parse_triggered(s: &str) -> Res<Property> {
    let parts: Vec<&str> = s.split(";").collect();
    let mut current = parts.len() - 1;
    let mut property = Property::with_effect(parts[current].trim());

    current -= 1;
    if current <= 0 { return Ok(property); }

    parse_attributes(&mut property.attr, parts[current].trim());
    Ok(property)
}

fn parse_passive(s: &str) -> Res<Property> {
    let parts: Vec<&str> = s.split(";").collect();
    let mut current = parts.len() - 1;
    let mut property = Property::with_effect(parts[current].trim());

    current -= 1;
    if current == 0 { return Ok(property); }

    parse_attributes(&mut property.attr, parts[current].trim());
    Ok(property)
}

fn parse_property(card: &mut Card, s: &str, property_type: PropertyType) -> Maybe {
    match property_type {
        PropertyType::action => {
            card.acti.push(parse_action(s)?)
        }
        PropertyType::triggered => {
            card.trig.push(parse_triggered(s)?)
        }
        PropertyType::passive => {
            card.pass.push(parse_passive(s)?)
        }
    };
    ok
}

fn process_card(card: &mut Card, s: &str) -> Maybe {
    let prev = find_next_property_position(s, 0);
    if let Some((mut prop, mut offset)) = prev {
        parse_attributes(&mut card.attr, &s[..offset]);
        while let Some(next) = find_next_property_position(s, offset + 1) {
            let substr = &s[offset..next.1];
            parse_property(card, substr, prop)?;
            (prop, offset) = next;
        }
        parse_property(card, &s[offset..], prop)?;
        ok
    }
    else {
        parse_attributes(&mut card.attr, s);
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

pub fn serialize_all_cards(directory: &str) -> Vec<Card> {
    let mut cards = Vec::<Card>::with_capacity(100_000);

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

    for card in cards.iter_mut() {
        card.attr.sort_by(|a, b| { a.n.cmp(&b.n) });
    }

    cards
}