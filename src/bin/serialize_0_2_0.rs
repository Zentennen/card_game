#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{fs, iter::empty};
use extrust::*;
use card_game::*;

pub const nl_indicator_char: char = '$';
pub const optional_indicator_char: char = '*';

#[inline(always)]
fn process_cost_at(pos: usize, string: &mut String) -> Res<usize> {
    let mut num_end = pos;
    for (p, c) in string.bytes().enumerate().skip(pos) {
        if !string.is_char_boundary(p) {
            continue;
        }
        let c = string[p..].chars().next().unwrap();
        if !c.is_numeric() {
            num_end = p;
            break;
        }
    }

    if pos == num_end {
        num_end = string.len();
    }

    let s = &string[pos..num_end];
    let num = str::parse::<f64>(s)? * 2.0;
    string.replace_range(pos..num_end, &num.to_string());
    
    let mut end = pos;
    for (p, c) in string.bytes().enumerate().skip(pos) {
        if !string.is_char_boundary(p) {
            continue;
        }
        let c = string[p..].chars().next().unwrap();
        if !c.is_ascii_alphanumeric() {
            end = p;
            break;
        }
    }

    if end == pos {
        end = string.len();
    }

    Ok(end)
}

#[inline(always)]
fn inject_serialization_commands(string: String) -> Res<String> {
    let string = string.trim();
    let string = string.replacen("\n ", " ", usize::MAX);
    let string = string.replacen("  ", " ", usize::MAX);
    let string = string.replacen("Boost", "¤(Boost)", usize::MAX);
    let string = string.replacen("boost", "¤(boost)", usize::MAX);
    let string = string.replacen("experience", "¤(xp)", usize::MAX);
    let string = string.replacen("Experience", "¤(Xp)", usize::MAX);
    let string = string.replacen(" mana ", " ¤(mana) ", usize::MAX);
    let string = string.replacen(" mana.", " ¤(mana).", usize::MAX);
    let string = string.replacen(" mana,", " ¤(mana),", usize::MAX);
    let string = string.replacen(" Mana ", " ¤(Mana) ", usize::MAX);
    let string = string.replacen(" Mana.", " ¤(Mana).", usize::MAX);
    let string = string.replacen(" Mana,", " ¤(Mana),", usize::MAX);
    let string = string.replacen("I’m", "I am", usize::MAX);
    let string = string.replacen("I'm", "I am", usize::MAX);
    let string = string.replacen(". I am ", ". ¤(I am) ", usize::MAX);
    let string = string.replacen(" I am ", " ¤(i am) ", usize::MAX);
    let string = string.replacen(" I have", " ¤(i have)", usize::MAX);
    let string = string.replacen(". I have", " ¤(I have)", usize::MAX);
    let string = string.replacen(" my ", " ¤(my) ", usize::MAX);
    let string = string.replacen(". My ", ". ¤(My) ", usize::MAX);
    let string = string.replacen(" me ", " ¤(me) ", usize::MAX);
    let string = string.replacen(" me,", " ¤(me),", usize::MAX);
    let string = string.replacen(" me.", " ¤(me).", usize::MAX);
    let string = string.replacen(" Me ", " ¤(Me) ", usize::MAX);
    let string = string.replacen(" Me,", " ¤(Me),", usize::MAX);
    let string = string.replacen(" Me.", " ¤(Me).", usize::MAX);
    let string = string.replacen("\nI ", "\n¤(I) ", usize::MAX);
    let string = string.replacen(". I ", ". ¤(I) ", usize::MAX);
    let string = string.replacen(". I,", ". ¤(I),", usize::MAX);
    let string = string.replacen(". I.", ". ¤(I). ", usize::MAX);
    let string = string.replacen(" I ", " ¤(i) ", usize::MAX);
    let string = string.replacen(" I,", " ¤(i),", usize::MAX);
    let string = string.replacen(" I.", " ¤(i).", usize::MAX);
    let string = string.replacen("Cards ¤(I am) attached to", "The card ¤(I am) attached to", usize::MAX);
    let mut string = string.replacen("cards ¤(I am) attached to", "the card ¤(I am) attached to", usize::MAX);
    let string = &mut string;
    while let Some(pos) = string.find("Card with") {
        let mut end = pos;
        for (p, c) in string.bytes().enumerate().skip(pos + 10) {
            if !string.is_char_boundary(p) {
                continue;
            }
            let c = string[p..].chars().next().unwrap();
            if !c.is_ascii_alphabetic() {
                end = p;
                break;
            }
        }
        if end != pos {
            let concat = format!("¤(Cwsn({}))", &string[10+pos..end]);
            string.replace_range(pos..end, &concat);
        }
    }
    while let Some(pos) = string.find("card with") {
        let mut end = pos;
        for (p, c) in string.bytes().enumerate().skip(pos + 10) {
            if !string.is_char_boundary(p) {
                continue;
            }
            let c = string[p..].chars().next().unwrap();
            if !c.is_ascii_alphabetic() {
                end = p;
                break;
            }
        }
        if end != pos {
            let concat = format!("¤(cwsn({}))", &string[10+pos..end]);
            string.replace_range(pos..end, &concat);
        }
    }
    while let Some(pos) = string.find("Cards with") {
        let mut end = pos;
        for (p, c) in string.bytes().enumerate().skip(pos + 11) {
            if !string.is_char_boundary(p) {
                continue;
            }
            let c = string[p..].chars().next().unwrap();
            if !c.is_ascii_alphabetic() {
                end = p;
                break;
            }
        }
        if end != pos {
            let concat = format!("¤(Cswsn({}))", &string[11+pos..end]);
            string.replace_range(pos..end, &concat);
        }
    }
    while let Some(pos) = string.find("cards with") {
        let mut end = pos;
        for (p, c) in string.bytes().enumerate().skip(pos + 11) {
            if !string.is_char_boundary(p) {
                continue;
            }
            let c = string[p..].chars().next().unwrap();
            if !c.is_ascii_alphabetic() {
                end = p;
                break;
            }
        }
        if end != pos {
            let concat = format!("¤(cswsn({}))", &string[11+pos..end]);
            string.replace_range(pos..end, &concat);
        }
    }
    while let Some(pos) = string.find("Pay ") {
        let end = process_cost_at(pos + 4, string).unwrap();
        if end != pos {
            let concat = format!("¤(Pay({}))", &string[4+pos..end]);
            string.replace_range(pos..end, &concat);
        }
    }
    while let Some(pos) = string.find("pay ") {
        let end = process_cost_at(pos + 4, string).unwrap();
        if end != pos {
            let concat = format!("¤(pay({}))", &string[4+pos..end]);
            string.replace_range(pos..end, &concat);
        }
    }
    
    Ok(string.clone())
}

#[inline(always)]
fn find_next_property_position(string: &str, start: usize) -> Option<(PropertyType, usize)> {
    #[derive(Copy, Clone)]
    enum PriorityReadingState { integer_, period_, decimal_, beginning_ }
    let mut priority_reading_state = PriorityReadingState::beginning_;
    let mut prop = Option::<(PropertyType, usize)>::None;

    for (p, b) in string.bytes().enumerate().skip(start) {
        if !string.is_char_boundary(p) {
            continue;
        }
        let c = string[p..].chars().next().unwrap();
        match prop {
            None => match c {
                'A' => prop = Some((PropertyType::action_, p)),
                'T' => prop = Some((PropertyType::triggered_, p)),
                'P' => prop = Some((PropertyType::passive_, p)),
                _ => continue
            },
            Some((pt, _)) => match pt {
                PropertyType::action_ => {
                    if c == ';' {
                        return prop;
                    }
                    else {
                        prop = None;
                    }
                },
                PropertyType::triggered_ | PropertyType::passive_ => {
                    match priority_reading_state {
                        PriorityReadingState::beginning_ => {
                            if c.is_ascii_digit() {
                                priority_reading_state = PriorityReadingState::integer_;
                            }
                            else {
                                priority_reading_state = PriorityReadingState::beginning_;
                                prop = None;
                            }
                        }
                        PriorityReadingState::integer_ => {
                            if c.is_ascii_digit() {
                                continue;
                            }
                            else if c == '.' {
                                priority_reading_state = PriorityReadingState::period_;
                            }
                            else if c == ';' {
                                return prop;
                            }
                            else {
                                priority_reading_state = PriorityReadingState::beginning_;
                                prop = None;
                            }
                        }
                        PriorityReadingState::period_ => {
                            if c.is_ascii_digit() {
                                priority_reading_state = PriorityReadingState::decimal_;
                            }
                            else {
                                priority_reading_state = PriorityReadingState::beginning_;
                                prop = None;
                            }
                        }
                        PriorityReadingState::decimal_ => {
                            if c.is_ascii_digit() {
                                continue;
                            }
                            else if c == ';' {
                                return prop;
                            }
                            else {
                                priority_reading_state = PriorityReadingState::beginning_;
                                prop = None;
                            }
                        }
                    }
                },
            }
        }
    }

    None
}

#[inline(always)]
fn parse_attributes(attributes: &mut Vec<Attribute>, s: &str) {
    let s = s.trim();
    let attribute_strings = s.split(',');
    for attribute_string in attribute_strings {
        let attribute_string = attribute_string.trim();
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
        let mut attribute = match name.trim() {
            "Flip" => Attribute::with_name("¤(flip)"),
            "Any Phase" => Attribute::with_name("¤(any_phase)"),
            "Combat Only" => Attribute::with_name("¤(co)"),
            "Hand Only" => Attribute::with_name("¤(ho)"),
            "Quick" => Attribute::with_name("¤(quick)"),
            "Instant" => Attribute::with_name("¤(instant)"),
            "Passing" => Attribute::with_name("¤(passing)"),
            _ => Attribute::with_name(name)
        };
        if subattribute_part != "" {
            let error_string = format!("ERROR: Failed to parse subattribute '{}' as part of attribute '{}'", subattribute_part, attribute_string);
            let sub = str::parse::<f64>(subattribute_part).expect(&error_string);
            attribute.f.push(sub);
        }
        attributes.push(attribute);
    }
}

#[inline(always)]
fn parse_action(s: &str) -> Res<Property> {
    let parts: Vec<&str> = s.split(";").collect();
    let mut current = parts.len() - 1;

    if current == 1 {
        let cost_ix = 4;
        let effect_str = parts[1].trim();
        if &effect_str[..cost_ix] == "SUM " {
            let mut action = Property::with_effect("¤(Sum(");
            let mut effect_string = effect_str.to_string();
            let end = process_cost_at(cost_ix, &mut effect_string)?;
            action.efct.push_str(&effect_string[cost_ix..end].trim());
            action.efct.push_str("))");
            action.attr.push(Attribute::with_name("¤(ho)"));
            return Ok(action);
        }
        else if &effect_str[..cost_ix] == "EVO " {
            let mut action = Property::with_effect("¤(Evo(");
            let mut effect_string = effect_str.to_string();
            let end = process_cost_at(cost_ix, &mut effect_string)?;
            action.efct.push_str(&effect_string[cost_ix..end].trim());
            action.efct.push_str("))");
            action.attr.push(Attribute::with_name("¤(ho)"));
            return Ok(action);
        }
        else if &effect_str[..cost_ix] == "EQU " {
            let mut action = Property::with_effect("¤(Equ(");
            let mut effect_string = effect_str.to_string();
            let end = process_cost_at(cost_ix, &mut effect_string)?;
            action.efct.push_str(&effect_string[cost_ix..end].trim());
            action.efct.push_str("))");
            action.attr.push(Attribute::with_name("¤(ho)"));
            return Ok(action);
        }
        panic!("ERROR: Invalid action property: {}", s);
    }

    let mut effect = String::with_capacity(default_property_effect_alloc);
    if current > 1 {
        effect.push_str(parts[current - 1].trim());
        effect.push_str(". ");
    }
    effect.push_str(parts[current].trim());
    let mut action = Property::with_effect_string(effect);
    current -= 2;
    if current <= 0 { return Ok(action); }   

    parse_attributes(&mut action.attr, parts[current].trim());

    Ok(action)
}

#[inline(always)]
fn parse_triggered(s: &str) -> Res<Property> {
    let parts: Vec<&str> = s.split(";").collect();
    let mut current = parts.len() - 1;

    if current == 1 {
        panic!("ERROR: Invalid triggered property: {}", s);
    }
    
    let mut effect = String::with_capacity(default_property_effect_alloc);
    if current > 1 {
        effect.push_str(parts[current - 1].trim());
        effect.push_str(". ");
    }
    effect.push_str(parts[current].trim());

    let mut triggered = Property::with_effect_string(effect);
    let prio = str::parse::<f64>(&parts[0].trim()[1..])?;
    if prio != 5.0 {
        let mut attr = Attribute::with_name("Priority");
        attr.f.push(prio);
        triggered.attr.push(attr);
    }

    current -= 2;
    if current <= 0 { return Ok(triggered); }   

    parse_attributes(&mut triggered.attr, parts[current].trim());

    return Ok(triggered);
}

#[inline(always)]
fn parse_passive(s: &str) -> Res<Property> {
    let parts: Vec<&str> = s.split(";").collect();
    let mut current = parts.len() - 1;
    
    let mut passive = Property::with_effect(parts[current].trim());
    let prio = str::parse::<f64>(&parts[0].trim()[1..]).expect(format!("ERROR: Failed to parse passive priority from: '{}'", &parts[0]).as_str());
    if prio != 5.0 {
        let mut attr = Attribute::with_name("Priority");
        attr.f.push(prio);
        passive.attr.push(attr);
    }

    current -= 1;
    if current == 0 { return Ok(passive); }

    parse_attributes(&mut passive.attr, parts[current].trim());
    
    Ok(passive)
}

fn parse_property(card: &mut Card, s: &str, property_type: PropertyType) -> Maybe {
    match property_type {
        PropertyType::action_ => {
            card.acti.push(parse_action(s)?)
        }
        PropertyType::triggered_ => {
            card.trig.push(parse_triggered(s)?)
        }
        PropertyType::passive_ => {
            card.pass.push(parse_passive(s)?)
        }
    };
    ok
}

#[inline(always)]
fn process_card(card: &mut Card, s: &str) -> Maybe {
    let mut prev = find_next_property_position(s, 0).expect(&format!("No property found on card: {s}"));
    parse_attributes(&mut card.attr, &s[..prev.1]);
    while let Some(next) = find_next_property_position(s, prev.1 + 1) {
        let substr = &s[prev.1..next.1];
        parse_property(card, substr, prev.0)?;
        prev = next;
    }
    parse_property(card, &s[prev.1..], prev.0)?;
    ok
}

#[inline(always)]
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

#[inline(always)]
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

#[inline(always)]
fn parse_txt_string(string: String) -> Vec<Card> {
    let string = inject_serialization_commands(string);
    if let Err(e) = string {
        panic!("Failed to inject serialization commands: {e:?}");
    }
    let string = string.unwrap();
    let string = string.replacen("Summon", "A; SUM", usize::MAX);
    let string = string.replacen("Evoke", "A; EVO", usize::MAX);
    let string = string.replacen("Equip", "A; EQU", usize::MAX);

    let card_strings = split_string_by_cards(&string);
    let cards = process_card_strings(card_strings);

    return cards;
}

fn main() {
    let mut cards = Vec::<Card>::with_capacity(100_000);

    let entries = fs::read_dir("serialize_0_2_0").expect("Could not find directory serialize_0_2_0");
    for entry in entries {
        if let Result::Ok(entry) = entry {
            if let Some(ext) = entry.path().extension() {
                if ext == "txt" {
                    println!("Processing file: {}", &entry.path().to_str().unwrap());
                    let string = fs::read_to_string(entry.path());
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
        if let Some(attribute) = get_attribute_mut_with_name(&mut card.attr, "Offense") {
            attribute.f[0] = 2.0 * attribute.f[0];
        }
        else {
            card.attr.push(Attribute{n: "Offense".to_string(), f: vec![0.0], a: vec![], s: vec![]});
        }
        if let Some(attribute) = get_attribute_mut_with_name(&mut card.attr, "Defense") {
            attribute.f[0] = 2.0 * attribute.f[0];
        }
        else {
            card.attr.push(Attribute{n: "Defense".to_string(), f: vec![0.0], a: vec![], s: vec![]});
        }
        if let Some(attribute) = get_attribute_mut_with_name(&mut card.attr, "Power") {
            attribute.f[0] = 2.0 * attribute.f[0];
        }
        else {
            card.attr.push(Attribute{n: "Power".to_string(), f: vec![0.0], a: vec![], s: vec![]});
        }
        if !has_attribute_with_name(&card.attr, "Health") {
            card.attr.push(Attribute{n: "Health".to_string(), f: vec![1.0], a: vec![], s: vec![]});
        }
        if !has_attribute_with_name(&card.attr, "Lethality") {
            card.attr.push(Attribute{n: "Lethality".to_string(), f: vec![1.0], a: vec![], s: vec![]});
        }
        if let Some(attribute) = get_attribute_mut_with_name(&mut card.attr, "Tribute") {
            attribute.f[0] = 2.0 * attribute.f[0] - 1.0;
        }
        if let Some(attribute) = get_attribute_mut_with_name(&mut card.attr, "Lethality") {
            attribute.n.clear();
            attribute.n.push_str("Strength");
        }
    }

    cout("Writing to json");
    let cards_as_string = serde_json::to_string(&cards).unwrap();
    let cards_as_string = cards_as_string.replacen("  ", " ", usize::MAX);
    fs::write("./cards.json", cards_as_string).expect("ERROR: Failed to write output!");
    println!("{} card(s) serialized successfully", cards.len());
}