#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::collections::HashMap;
use extrust::*;
use card_game::*;
use pyo3::*;
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

const default_card_string_alloc: usize = 1000;
const card_w: f64 = 1.0;
const card_h: f64 = 5.3;
//                                   0    1     2     3     4    5    6
const attrib_padding_l: [f64; 7] = [ 0.0, 27.7, 16.0, 11.0, 7.5, 4.0, 0.8 ];
const attrib_padding_t: f64 = 1.7;
const attrib_padding_b: f64 = 1.7;
const attrib_text_padding_t: f64 = 0.5;
const attrib_text_padding_r: f64 = 0.6;
const attrib_text_padding_l: f64 = 0.6;
const card_padding: f64 = 1.5;
const card_padding_t: f64 = 3.0;
const page_padding: u8 = 6;
const page_padding_l: u8 = 12;
const page_padding_r: u8 = 5;
const cards_per_column: usize = 3;
const cards_per_row: usize = 3;
const cards_per_page: usize = cards_per_column * cards_per_row;
const name_font_size: u8 = 9;
const main_attrib_font_size: u8 = 7;
const font_size: u8 = 6;
const line_spacing: f64 = 1.25;
const num_main_attribs: usize = 6;

struct PdfHandler<'p> {
    py: Python<'p>,
    pdf: &'p PyAny
}

impl PdfHandler<'_> {
    fn new<'p>(py: Python<'p>) -> PdfHandler<'p> {
        let fpdf = py.import("fpdf").unwrap();
        let pdf = fpdf.getattr("FPDF").unwrap().call0().unwrap();
        let s = PdfHandler::<'p> { py, pdf };
        s.init();
        s
    }

    fn init(&self) {
        //Load fonts
        let entries = std::fs::read_dir("deserialize/fonts").expect("Could not find directory deserialize/fonts");
        for entry in entries {
            if let Result::Ok(entry) = entry {
                let name = entry.file_name().to_str().unwrap().to_string();
                let base_path = entry.path().as_path().to_str().unwrap().to_string();
                
                let path = format!("{}/{}-Regular.ttf", base_path, name);
                let args = vec![("family", &name as &str), ("fname", &path as &str)].into_py_dict(self.py);
                self.pdf.call_method("add_font", (), Some(args)).unwrap();

                let path = format!("{}/{}-Bold.ttf", base_path, name);
                let args = vec![("family", &name as &str), ("style", "B"), ("fname", &path as &str)].into_py_dict(self.py);
                self.pdf.call_method("add_font", (), Some(args)).unwrap();

                let path = format!("{}/{}-Italic.ttf", base_path, name);
                let args = vec![("family", &name as &str), ("style", "I"), ("fname", &path as &str)].into_py_dict(self.py);
                self.pdf.call_method("add_font", (), Some(args)).unwrap();
            }
        }
    }
}

fn process_commands(string: String) -> String {
    let string = string.replacen("¤(ho)", "Hand Only", usize::MAX);
    let string = string.replacen("¤(co)", "Combat Only", usize::MAX);
    let string = string.replacen("¤(any_phase)", "Any Phase", usize::MAX);
    let string = string.replacen("¤(flip)", "Flip", usize::MAX);
    let string = string.replacen("¤(quick)", "Quick", usize::MAX);
    let string = string.replacen("¤(instant)", "Instant", usize::MAX);
    let string = string.replacen("¤(passing)", "Passing", usize::MAX);
    let string = string.replacen("¤(Choose_your_c)", "Choose a card you control", usize::MAX);
    let string = string.replacen("¤(choose_your_c)", "choose a card you control", usize::MAX);
    let string = string.replacen("¤(Boost)", "Boost", usize::MAX);
    let string = string.replacen("¤(Boost)", "boost", usize::MAX);
    let string = string.replacen("¤(I have)", "This card has", usize::MAX);
    let string = string.replacen("¤(i have)", "this card has", usize::MAX);
    let string = string.replacen("¤(C_to_lane)", "Card's lane", usize::MAX);
    let string = string.replacen("¤(c_to_lane)", "card's lane", usize::MAX);
    let string = string.replacen("¤(Choose_your_pos)", "Choose a position on your board", usize::MAX);
    let string = string.replacen("¤(choose_your_pos)", "choose a position on your board", usize::MAX);
    let string = string.replacen("¤(Choose_your_r)", "Choose one of your reserves", usize::MAX);
    let string = string.replacen("¤(choose_your_r)", "choose one of your reserves", usize::MAX);
    let string = string.replacen("¤(Choose_your_cpos)", "Choose one of your field positions", usize::MAX);
    let string = string.replacen("¤(choose_your_cpos)", "choose one of your field positions", usize::MAX);
    let string = string.replacen("¤(Pos_to_lane)", "Position's lane", usize::MAX);
    let string = string.replacen("¤(pos_to_lane)", "position's lane", usize::MAX);
    let string = string.replacen("¤(Move_me_to_r)", "Move this card to the chosen reserve", usize::MAX);
    let string = string.replacen("¤(move_me_to_r)", "move this card to the chosen reserve", usize::MAX);
    let string = string.replacen("¤(Attach_me_to_chosen)", "Attach this card to the chosen card", usize::MAX);
    let string = string.replacen("¤(attach_me_to_chosen)", "attach this card to the chosen card", usize::MAX);
    let string = string.replacen("¤(Xp)", "XP", usize::MAX);
    let string = string.replacen("¤(xp)", "XP", usize::MAX);
    let string = string.replacen("¤(Mana)", "Mana", usize::MAX);
    let string = string.replacen("¤(mana)", "mana", usize::MAX);
    let string = string.replacen("¤(Me)", "This card", usize::MAX);
    let string = string.replacen("¤(me)", "this card", usize::MAX);
    let string = string.replacen("¤(I)", "This card", usize::MAX);
    let string = string.replacen("¤(I am)", "This card is", usize::MAX);
    let string = string.replacen("¤(i)", "this card", usize::MAX);
    let string = string.replacen("¤(i am)", "this card is", usize::MAX);
    let string = string.replacen("¤(my)", "this card's", usize::MAX);
    let string = string.replacen("¤(My)", "This card's", usize::MAX);
    let string = string.replacen("¤(Set_my_pos_to_chosen)", "Move this card to the chosen position", usize::MAX);
    let mut string = string.replacen("¤(set_my_pos_to_chosen)", "move this card to the chosen position", usize::MAX);
    
    while let Some(pos) = string.find("¤(mod(") {
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(mod()) was not correctly terminated");
        let substr = &string[start..end];
        let num = substr.split(',').nth(1).unwrap().trim();
        let num: f64 = str::parse(num).unwrap();
        let attr = substr.split(',').nth(0).unwrap().trim();
        if num >= 0.0 {
            let replacement = format!("+{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
        else {
            let replacement = format!("-{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
    }
    while let Some(pos) = string.find("¤(p_mods_one(") {
        let start = pos + 14;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(p_mods_one()) was not correctly terminated");
        let substr = &string[start..end];
        let num = substr.split(',').nth(1).unwrap().trim();
        let num: f64 = str::parse(num).unwrap();
        let attr = substr.split(',').nth(0).unwrap().trim();
        if num >= 0.0 {
            let replacement = format!("has +{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
        else {
            let replacement = format!("has -{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
    }
    while let Some(pos) = string.find("¤(p_mods_many(") {
        let start = pos + 15;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(p_mods_many()) was not correctly terminated");
        let substr = &string[start..end];
        let num = substr.split(',').nth(1).unwrap().trim();
        let num: f64 = str::parse(num).unwrap();
        let attr = substr.split(',').nth(0).unwrap().trim();
        if num >= 0.0 {
            let replacement = format!("have +{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
        else {
            let replacement = format!("have -{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
    }
    while let Some(pos) = string.find("¤(p_gives_one(") {
        let start = pos + 15;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(p_gives_one()) was not correctly terminated");
        let substr = &string[start..end];
        let attr = substr.split(',').nth(0).unwrap().trim();
        let replacement = format!("has the {} attribute", attr);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(p_gives_many(") {
        let start = pos + 16;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(p_gives_many()) was not correctly terminated");
        let substr = &string[start..end];
        let attr = substr.split(',').nth(0).unwrap().trim();
        let replacement = format!("have the {} attribute", attr);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(act_mods_one(") {
        let start = pos + 16;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(act_mods_one()) was not correctly terminated");
        let substr = &string[start..end];
        let num = substr.split(',').nth(1).unwrap().trim();
        let num: f64 = str::parse(num).unwrap();
        let attr = substr.split(',').nth(0).unwrap().trim();
        if num >= 0.0 {
            let replacement = format!("gets +{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
        else {
            let replacement = format!("gets -{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
    }
    while let Some(pos) = string.find("¤(act_mods_many(") {
        let start = pos + 17;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(act_mods_many()) was not correctly terminated");
        let substr = &string[start..end];
        let num = substr.split(',').nth(1).unwrap().trim();
        let num: f64 = str::parse(num).unwrap();
        let attr = substr.split(',').nth(0).unwrap().trim();
        if num >= 0.0 {
            let replacement = format!("get +{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
        else {
            let replacement = format!("get -{} {}", num, attr);
            string.replace_range(pos..end + 2, &replacement);
        }
    }
    while let Some(pos) = string.find("¤(Cswsn(") {
        let start = pos + 9;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Cswsn()) was not correctly terminated");
        let replacement = format!("{} cards", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(cswsn(") {
        let start = pos + 9;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(cswsn()) was not correctly terminated");
        let replacement = format!("{} cards", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Cwsn(") {
        let start = pos + 8;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Cwa()) was not correctly terminated");
        let replacement = format!("{} card", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(cwsn(") {
        let start = pos + 8;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(cwa()) was not correctly terminated");
        let replacement = format!("{} card", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Cswosn(") {
        let start = pos + 10;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Cswosn()) was not correctly terminated");
        let replacement = format!("Non-{} cards", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(cswosn(") {
        let start = pos + 10;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(cswosn()) was not correctly terminated");
        let replacement = format!("non-{} cards", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Cwosn(") {
        let start = pos + 9;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Cwosn()) was not correctly terminated");
        let replacement = format!("Non-{} card", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(cwosn(") {
        let start = pos + 9;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(cwosn()) was not correctly terminated");
        let replacement = format!("non-{} card", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Pay("){
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Pay()) was not correctly terminated");
        let replacement = format!("Pay {}", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(pay("){
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(pay()) was not correctly terminated");
        let replacement = format!("pay {}", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Sum("){
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Sum()) was not correctly terminated");
        let replacement = format!("Move this card to an empty friendly field position of your choice, then pay {}.", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Evo("){
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Evo()) was not correctly terminated");
        let replacement = format!("Move this card to a friendly reserve of your choice, then pay {}.", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Equ("){
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Equ()) was not correctly terminated");
        let replacement = format!("Attach this card to a friendly non-attached card of your choice, then pay {}.", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }

    let string = string.replacen("¤(zone)", "zone", usize::MAX);
    let string = string.replacen("¤(Zone)", "Zone", usize::MAX);

    return string;
}

fn add_attr_to_string(attr: &Attribute, string: &mut String) {
    string.push_str(&attr.n.replacen(' ', "\u{A0}", usize::MAX));
    if attr.count_subs() > 0 {
        string.push_str("\u{A0}(");
        
        for sub in &attr.f {
            string.push_str(&sub.to_string());
            string.push_str(",\u{A0}");
        }

        for sub in &attr.s {
            string.push(string_indicator_char);
            string.push_str(sub);
            string.push(string_indicator_char);
            string.push_str(",\u{A0}");
        }

        for sub in &attr.a {
            sub.add_rules_text_to_string(string);
            string.push_str(",\u{A0}");
        }

        string.pop();
        string.pop();
        string.push(')');
    }
}

fn main() -> Maybe {
    Python::with_gil(|py| {
        let ph = PdfHandler::new(py);
    });
    
    println!("Deserialized successfully");
    ok
}