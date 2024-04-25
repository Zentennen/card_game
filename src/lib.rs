#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use serde::*;
use std::collections::HashMap;

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
pub const page_pad_t: f64 = 13.0;
pub const page_pad_l: f64 = 13.0;
pub const cards_per_column: usize = 3;
pub const cards_per_row: usize = 3;
pub const cards_per_page: usize = cards_per_column * cards_per_row;

//card
pub const card_outer_width: f64 = 59.0;
pub const card_outer_height: f64 = 82.6;
pub const card_separation_width: f64 = card_outer_width + 3.0;
pub const card_separation_height: f64 = card_outer_height + 3.0;
pub const card_inner_width: f64 = card_outer_width - card_pad * 2.0;
pub const card_inner_height: f64 = card_outer_height - card_pad;
pub const card_pad: f64 = 2.5;
pub const text_offset: f64 = 1.0;
pub const card_pixel_width: usize = (card_outer_width * pixels_per_mm as f64) as usize;
pub const attributes: [&str; 10] = ["Offense", "Defense", "Strength", "Health", "Power", "Speed", "Salvage", "Morale", "Tactics", "Logistics"];

//name
pub const name_font_size: f64 = 8.5;
pub const name_h: f64 = 7.0;

//icons
pub const icon_size: f64 = 3.5;
pub const icon_pad_vertical: f64 = 1.5;
pub const icon_pad_h: f64 = 1.5;
pub const icon_row_height: f64 = icon_size + icon_pad_vertical;
pub const icon_text_font_size: f64 = 8.0;
pub const icon_text_pad_t: f64 = icon_text_font_size * 0.35;
pub const icon_text_pad_l: f64 = 1.0;
pub const commander_icon_size: f64 = 9.0;
pub const commander_offset_right: f64 = card_outer_width - commander_icon_size;

//attribute
pub const attribute_height: f64 = 3.2;

//property
pub const default_font_size: f64 = 6.5;
pub const property_height: f64 = 3.0;
pub const vertical_property_pad: f64 = 2.0;
pub const prop_sym_size: f64 = 2.5;
pub const prop_sym_pad_l: f64 = card_inner_width + text_offset - prop_sym_size;
pub const prop_sym_pad_t: f64 = (property_height * prop_sym_l as f64 - prop_sym_size) / 2.0;
pub const prop_efct_pad_r: f64 = 0.1;
pub const prop_sym_l: usize = (prop_sym_size / property_height - 0.00001) as usize + 1;
pub const prop_top_w: f64 = prop_sym_pad_l - prop_efct_pad_r;

//alpha
pub const max_alpha: u8 = 124;
pub const pixels_per_alpha_step: usize = 1;
pub const gradient_buffer_pixels: usize = 6;
pub const pixels_per_mm: f64 = 40.0;
pub const alpha_gradient_pixel_height: usize = (max_alpha as usize + gradient_buffer_pixels) * pixels_per_alpha_step;
pub const alpha_gradient_height: f64 = alpha_gradient_pixel_height as f64 / pixels_per_mm;
pub const upper_alpha_base_height: f64 = name_h + alpha_gradient_height;
pub const lower_alpha_base_height: f64 = card_pad + alpha_gradient_height;

pub mod serialize;
pub mod pdf;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Card {
    pub name: String,
    pub flavor_text: String,
    pub commander: bool,
    pub attributes: HashMap<String, String>,
    pub types: Vec<String>,
    pub abiilities: Vec<String>,
    pub reactions: Vec<String>,
    pub traits: Vec<String>,
}

impl Card {
    pub fn new() -> Self {
        Self { 
            name: String::with_capacity(default_card_name_alloc), 
            flavor_text: String::with_capacity(default_card_property_alloc), 
            commander: false,
            attributes: HashMap::<String, String>::with_capacity(default_card_attribute_alloc), 
            types: Vec::<String>::with_capacity(default_card_property_alloc),
            abiilities: Vec::<String>::with_capacity(default_card_property_alloc),
            reactions: Vec::<String>::with_capacity(default_card_property_alloc),
            traits: Vec::<String>::with_capacity(default_card_property_alloc),
        }
    }

    pub fn with_name(s: impl Into<String>) -> Self {
        Self { 
            name: s.into(), 
            flavor_text: String::with_capacity(default_card_property_alloc), 
            commander: false,
            attributes: HashMap::<String, String>::with_capacity(default_card_attribute_alloc), 
            types: Vec::<String>::with_capacity(default_card_property_alloc),
            abiilities: Vec::<String>::with_capacity(default_card_property_alloc),
            reactions: Vec::<String>::with_capacity(default_card_property_alloc),
            traits: Vec::<String>::with_capacity(default_card_property_alloc),
        }
    }
}