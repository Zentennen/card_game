#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use serde::*;

pub const default_subattrib_alloc: usize = 1;
pub const default_card_name_alloc: usize = 10;
pub const default_card_attribute_alloc: usize = 5;
pub const default_card_property_alloc: usize = 5;
pub const default_property_name_alloc: usize = 10;
pub const default_property_attribute_alloc: usize = 5;
pub const default_property_effect_alloc: usize = 30;
pub const default_action_text_alloc: usize = 100;
pub const default_effects_alloc: usize = 1;
pub const default_attribute_string_alloc: usize = 100;
pub const string_indicator_char: char = '\"';

type attrib_num = f64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub n: String,
    pub a: Vec<Attribute>,
    pub f: Vec<attrib_num>,
    pub s: Vec<String>,
}

impl Attribute {
    pub fn with_name(name: &str) -> Attribute {
        Attribute{ 
            n: name.to_string(), 
            a: Vec::<Attribute>::with_capacity(default_subattrib_alloc),
            f: Vec::<attrib_num>::with_capacity(default_subattrib_alloc),
            s: Vec::<String>::with_capacity(default_subattrib_alloc),
        }
    }

    pub fn count_subs(&self) -> usize {
        self.a.len() + self.f.len() + self.s.len()
    }

    pub fn add_rules_text_to_string(&self, s: &mut String) {
        let string = self.n.replacen(' ', "\u{A0}", usize::MAX);
        s.push_str(&string);
        if self.count_subs() > 0 {
            s.push_str("\u{A0}(");
            
            for sub in &self.f {
                s.push_str(&sub.to_string());
                s.push_str(",\u{A0}");
            }

            for sub in &self.s {
                s.push(string_indicator_char);
                s.push_str(sub);
                s.push(string_indicator_char);
                s.push_str(",\u{A0}");
            }

            for sub in &self.a {
                sub.add_rules_text_to_string(s);
                s.push_str(",\u{A0}");
            }

            s.pop();
            s.pop();
            s.push(')');
        }
    }
}

pub fn get_attribute_ref_with_name<'a>(attributes: &'a Vec<Attribute>, name: &str) -> Option<&'a Attribute> {
    for attribute in attributes {
        if attribute.n == name {
           return Some(attribute);
        }
    }
    None
}

pub fn has_attribute_with_name(attributes: &Vec<Attribute>, name: &str) -> bool {
    for attribute in attributes {
        if attribute.n == name {
            return true;
        }
    }
    false
}

pub fn get_attribute_mut_with_name<'a>(attributes: &'a mut Vec<Attribute>, name: &str) -> Option<&'a mut Attribute> {
    for attribute in attributes {
        if attribute.n == name {
           return Some(attribute);
        }
    }
    None
}

pub fn get_attribute_value(attributes: &Vec<Attribute>, name: &str) -> Option<attrib_num> {
    for attribute in attributes {
        if attribute.n == name && attribute.f.len() > 0 {
           return Some(attribute.f[0]);
        }
    }
    None
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    pub attr: Vec<Attribute>, 
    pub efct: String,
}

impl Property {
    pub fn new() -> Self {
        Self { 
            attr: Vec::<Attribute>::with_capacity(default_property_attribute_alloc),
            efct: String::with_capacity(default_property_effect_alloc),
        }
    }

    pub fn with_effect(effect: &str) -> Self {
        Self { 
            attr: Vec::<Attribute>::with_capacity(default_property_attribute_alloc),
            efct: effect.to_string(),
        }
    }

    pub fn with_effect_string(effect: String) -> Self {
        Self { 
            attr: Vec::<Attribute>::with_capacity(default_property_attribute_alloc),
            efct: effect,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum PropertyType {
    action_, triggered_, passive_
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub attr: Vec<Attribute>,
    pub acti: Vec<Property>,
    pub trig: Vec<Property>,
    pub pass: Vec<Property>,
}

pub fn empty_card() -> Card {
    Card { 
        name: String::with_capacity(default_card_name_alloc), 
        attr: Vec::<Attribute>::with_capacity(default_card_attribute_alloc), 
        acti: Vec::<Property>::with_capacity(default_card_property_alloc),
        trig: Vec::<Property>::with_capacity(default_card_property_alloc),
        pass: Vec::<Property>::with_capacity(default_card_property_alloc), 
    }
}

pub fn card_with_name(s: String) -> Card {
    Card { 
        name: s, 
        attr: Vec::<Attribute>::with_capacity(default_card_attribute_alloc), 
        acti: Vec::<Property>::with_capacity(default_card_property_alloc),
        trig: Vec::<Property>::with_capacity(default_card_property_alloc),
        pass: Vec::<Property>::with_capacity(default_card_property_alloc), 
    }
}