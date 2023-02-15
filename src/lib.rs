#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use serde::*;

//data
pub const default_subattrib_alloc: usize = 1;
pub const default_card_name_alloc: usize = 10;
pub const default_card_attribute_alloc: usize = 5;
pub const default_card_property_alloc: usize = 5;
pub const default_property_name_alloc: usize = 20;
pub const default_property_attribute_alloc: usize = 5;
pub const default_property_effect_alloc: usize = 50;
pub const default_action_text_alloc: usize = 100;
pub const default_effects_alloc: usize = 1;
pub const default_name_string_alloc: usize = 100;
pub const default_attr_string_alloc: usize = 100;
pub const string_indicator_char: char = '\"';

//page
pub const page_pad_t: f64 = 5.0;
pub const page_pad_l: f64 = 12.0;
pub const cards_per_column: usize = 3;
pub const cards_per_row: usize = 3;
pub const cards_per_page: usize = cards_per_column * cards_per_row;

//card
pub const card_outer_w: f64 = 63.0;
pub const card_outer_h: f64 = 88.0;
pub const card_inner_w: f64 = card_outer_w - card_pad * 2.0;
pub const card_inner_h: f64 = card_outer_h - card_pad;
pub const card_pad: f64 = 2.5;
pub const upper_alpha_base_h: f64 = name_h + gradient_h + main_attr_h;
pub const text_offset: f64 = 1.0;

//name
pub const name_font_size: f64 = 9.0;
pub const name_h: f64 = 7.0;
pub const advanced_sym_size: f64 = 12.0;
pub const advanced_offset_r: f64 = card_outer_w - advanced_sym_size;

//attr
pub const main_attr_font_size: f64 = 9.0;
pub const main_attr_text_pad_t: f64 = main_attr_font_size * 0.30;
pub const main_attr_text_pad_l: f64 = 0.9;
pub const main_attr_h: f64 = 3.5;
pub const main_attr_pad_b: f64 = 1.6;
pub const main_attr_pad_lr: f64 = 1.5;
pub const main_attr_w: f64 = card_outer_w - main_attr_pad_lr * 2.0;
pub const other_attr_h: f64 = 3.2;
pub const gradient_h: f64 = 3.0;

//prop
pub const default_font_size: f64 = 6.7;
pub const prop_h: f64 = 3.0;
pub const prop_sym_size: f64 = 2.0;
pub const prop_sym_pad_l: f64 = card_inner_w + text_offset - prop_sym_size;
pub const prop_sym_pad_t: f64 = (prop_h * prop_sym_l as f64 - prop_sym_size) / 2.0;
pub const prop_efct_pad_r: f64 = 0.1;
pub const prop_sym_l: usize = (prop_sym_size / prop_h - 0.00001) as usize + 1;
pub const prop_top_w: f64 = prop_sym_pad_l - prop_efct_pad_r;

pub mod serialize;
pub mod to_pdf;

type attr_num = f64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub n: String,
    pub a: Vec<Attribute>,
    pub f: Vec<attr_num>,
    pub s: Vec<String>,
}

impl Attribute {
    pub fn with_name(name: &str) -> Attribute {
        Attribute{ 
            n: name.to_string(), 
            a: Vec::<Attribute>::with_capacity(default_subattrib_alloc),
            f: Vec::<attr_num>::with_capacity(default_subattrib_alloc),
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

pub fn get_attribute_value(attributes: &Vec<Attribute>, name: &str) -> Option<attr_num> {
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

impl Card {
    pub fn new() -> Self {
        Self { 
            name: String::with_capacity(default_card_name_alloc), 
            attr: Vec::<Attribute>::with_capacity(default_card_attribute_alloc), 
            acti: Vec::<Property>::with_capacity(default_card_property_alloc),
            trig: Vec::<Property>::with_capacity(default_card_property_alloc),
            pass: Vec::<Property>::with_capacity(default_card_property_alloc), 
        }
    }

    pub fn with_name(s: impl Into<String>) -> Self {
        Self { 
            name: s.into(), 
            attr: Vec::<Attribute>::with_capacity(default_card_attribute_alloc), 
            acti: Vec::<Property>::with_capacity(default_card_property_alloc),
            trig: Vec::<Property>::with_capacity(default_card_property_alloc),
            pass: Vec::<Property>::with_capacity(default_card_property_alloc), 
        }
    }
}