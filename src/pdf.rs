#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

use crate::*;
use extstd::*;
use pyo3::*;
use pyo3::types::IntoPyDict;

pub const name_text_mod: TextModifier = TextModifier::bold;
pub const attr_text_align: Alignment = Alignment::center;
pub const attr_text_mod: TextModifier = TextModifier::italic;
pub const font_name: &'static str = "Bitter";
pub const font_line_width: f64 = 0.1;
pub const rect_line_w: f64 = 0.2;
pub const default_text_align: Alignment = Alignment::left;
pub const default_text_mod: TextModifier = TextModifier::none;

pub struct PdfHandler<'p> {
    py: Python<'p>,
    pdf: &'p PyAny,
    image_aliases: std::collections::HashMap<&'static str, &'static str>
}

impl PdfHandler<'_> {
    pub fn new<'p>(py: Python<'p>) -> PdfHandler<'p> {
        let fpdf = py.import("fpdf").expect("ERR: fpdf not installed");
        let pdf = fpdf.getattr("FPDF").unwrap().call0().unwrap();
        let image_aliases = std::collections::HashMap::new();

        let mut s = PdfHandler::<'p> { py, pdf, image_aliases };
        s.init();
        s
    }

    pub fn init(&mut self) {
        //image aliases
        //self.image_aliases.insert("Test Complicated Card.png", "Test Simple Card.png");

        //fonts
        let entries = std::fs::read_dir("fonts").expect("Could not find directory fonts");
        for entry in entries {
            if let Result::Ok(entry) = entry {
                let name = entry.path().file_stem().unwrap().to_str().unwrap().to_string();
                let base_path = entry.path().as_path().to_str().unwrap().to_string();
                
                if entry.metadata().unwrap().is_dir() {
                    let mut end = "ttf";
                    let mut path = format!("{}/{}-Regular.{}", base_path, name, end);
                    if !std::fs::metadata(&path).is_ok() {
                        end = "otf";
                        path = format!("{}/{}-Regular.{}", base_path, name, end);
                    }

                    let args = vec![("family", &name as &str), ("fname", &path as &str)].into_py_dict(self.py);
                    self.pdf.call_method("add_font", (), Some(args)).unwrap();
    
                    let path = format!("{}/{}-Bold.{}", base_path, name, end);
                    let args = vec![("family", &name as &str), ("style", "B"), ("fname", &path as &str)].into_py_dict(self.py);
                    self.pdf.call_method("add_font", (), Some(args)).unwrap();
    
                    let path = format!("{}/{}-Italic.{}", base_path, name, end);
                    let args = vec![("family", &name as &str), ("style", "I"), ("fname", &path as &str)].into_py_dict(self.py);
                    self.pdf.call_method("add_font", (), Some(args)).unwrap();
                }
                else {
                    let args = vec![("family", &name as &str), ("fname", &base_path as &str)].into_py_dict(self.py);
                    self.pdf.call_method("add_font", (), Some(args)).unwrap();
                }
            }
        }
    }

    pub fn add_page(&self) {
        self.pdf.call_method0("add_page").unwrap();
    }

    pub fn set_font(&self, family: &str, size: f64) {
        let args = (family, "", size);
        self.pdf.call_method1("set_font", args).unwrap();
    }

    pub fn set_font_modded(&self, family: &str, size: f64, modifier: TextModifier) {
        let args: (&str, &str, f64) = (family, modifier.into(), size);
        self.pdf.call_method1("set_font", args).unwrap();
    }

    pub fn get_x(&self) -> f64 {
        self.pdf.call_method0("get_x").unwrap().extract().unwrap()
    }

    pub fn get_y(&self)-> f64 {
        self.pdf.call_method0("get_y").unwrap().extract().unwrap()
    }

    pub fn set_x(&self, x: f64) {
        let args = vec![("x", x)].into_py_dict(self.py);
        self.pdf.call_method("set_x", (), Some(args)).unwrap();
    }

    pub fn set_y(&self, y: f64) {
        let args = vec![("y", y)].into_py_dict(self.py);
        self.pdf.call_method("set_y", (), Some(args)).unwrap();
    }

    pub fn set_xy(&self, x: f64, y: f64) {
        let args = vec![("x", x), ("y", y)].into_py_dict(self.py);
        self.pdf.call_method("set_xy", (), Some(args)).unwrap();
    }

    pub fn text(&self, txt: &str, x: f64, y: f64) {
        let args = (x, y, txt);
        self.pdf.call_method1("text", args).unwrap();
    }

    pub fn write(&self, txt: &str) {
        let args = vec![("txt", txt)].into_py_dict(self.py);
        self.pdf.call_method("write", (), Some(args)).unwrap();
    }

    pub fn write_markdown(&self, txt: &str) {
        let args = vec![("text", txt)].into_py_dict(self.py);
        self.pdf.call_method("write_html", (), Some(args)).unwrap();
    }

    pub fn multi_cell(&self, txt: &str, w: f64, h: f64, align: Alignment) {
        let args: (f64, f64, &str, i32, &str) = (w, h, txt, 0, align.into());
        let kwargs = [("markdown", true)].into_py_dict(self.py);
        self.pdf.call_method("multi_cell", args, Some(kwargs)).unwrap();
    }

    pub fn multi_cell_h(&self, txt: &str, w: f64, h: f64, align: Alignment) -> f64 {
        let args: (f64, f64, &str, i32, &str) = (w, h, txt, 0, align.into());
        let kwargs = [("split_only", true), ("markdown", true)].into_py_dict(self.py);
        let strings = self.pdf.call_method("multi_cell", args, Some(kwargs)).expect("ERROR!!!").to_object(self.py);
        let strings: Vec<&str> = strings.extract(self.py).unwrap();
        h * strings.len() as f64
    }

    pub fn multi_cell_l(&self, txt: &str, w: f64, h: f64, align: Alignment) -> usize {
        let args: (f64, f64, &str, i32, &str) = (w, h, txt, 0, align.into());
        let kwargs = [("split_only", true), ("markdown", true)].into_py_dict(self.py);
        let strings = self.pdf.call_method("multi_cell", args, Some(kwargs)).unwrap().to_object(self.py);
        let strings: Vec<&str> = strings.extract(self.py).unwrap();
        strings.len()
    }

    fn split_on_lines<'a>(&self, txt: &'a str, w: f64, h: f64, align: Alignment, max_lines: usize) -> (&'a str, &'a str) {
        let num_chars = txt.num_chars();
        if num_chars == 0 {
            return ("", "");
        }

        let last = num_chars - 1;
        let mut i = 0;
        let mut split_at = last + 1;
        while self.multi_cell_l(txt.to(i + 1), w, h, align) <= max_lines {
            i += 1;
            if i == last {
                split_at = last + 1;
                break;
            }
            if txt.nth_char(i).unwrap() == ' ' {
                split_at = i;
            }
        }
        
        if split_at > last {
            (txt, "")
        }
        else {
            let split_at = txt.char_indices().nth(split_at).unwrap().0;
            let ret = txt.split_at(split_at);
            (ret.0.trim(), ret.1.trim())
        }
    }

    pub fn image(&self, name: &str, folder: &str, w: f64, h: f64) {
        let name = *self.image_aliases.get(name).unwrap_or(&name);
        let path = format!("{}/{}", folder, name);
        let args = types::PyTuple::new(self.py, &[path]);
        let kwargs = [("w", w), ("h", h)].into_py_dict(self.py);
        self.pdf.call_method("image", args, Some(kwargs)).expect(name);
    }

    pub fn has_image(&self, name: &str, folder: &str) -> bool {
        self.image_aliases.contains_key(name) || std::fs::metadata(&format!("{}/{}", folder, name)).is_ok()
    }

    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) {
        let args = (x, y, w, h);
        self.pdf.call_method1("rect", args).unwrap();
    }

    pub fn filled_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        let args = (x, y, w, h, "F");
        self.pdf.call_method1("rect", args).unwrap();
    }

    pub fn line(&self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let args = (x1, y1, x2, y2);
        self.pdf.call_method1("line", args).unwrap();
    }

    pub fn string_width(&self, string: &str) -> f64 {
        let args = types::PyTuple::new(self.py, &[string]);
        let w = self.pdf.call_method1("get_string_width", args).unwrap().to_object(self.py);
        w.extract(self.py).unwrap()
    }

    pub fn set_text_color(&self, r: f64, g: f64, b: f64) {
        let args = (r, g, b);
        self.pdf.call_method1("set_text_color", args).unwrap();
    }

    pub fn set_draw_color(&self, r: f64, g: f64, b: f64) {
        let args = (r, g, b);
        self.pdf.call_method1("set_draw_color", args).unwrap();
    }

    pub fn set_fill_color(&self, r: f64, g: f64, b: f64) {
        let args = (r, g, b);
        self.pdf.call_method1("set_fill_color", args).unwrap();
    }

    pub fn set_text_mode(&self, mode: &str) {
        self.pdf.setattr("text_mode", mode).unwrap();
    }

    pub fn set_line_width(&self, w: f64) {
        let args = types::PyTuple::new(self.py, &[w]);
        self.pdf.call_method1("set_line_width", args).unwrap();
        self.pdf.setattr("line_width", args).unwrap();
    }

    pub fn get_height_sum(&self, strings: &Vec<&str>, w: f64, h: f64, align: Alignment) -> usize {
        let mut l = 0;
        for string in strings {
            l += self.multi_cell_l(string, w, h, align);
        }
        l
    }

    pub fn output(&self) {
        let args = types::PyTuple::new(self.py, &["cards.pdf"]);
        self.pdf.call_method1("output", args).unwrap();
    }
}

pub struct DeserializedProperty {
    pub keywords: Vec<String>,
    pub efct_limited: String,
    pub efct_non_limited: String,
    pub efct_limited_h: f64,
    pub efct_non_limited_h: f64,
}

impl DeserializedProperty {
    pub fn from_property(prop: &String, ph: &PdfHandler) -> DeserializedProperty {
        let mut total_limited_l = 0;
        let mut efct = prop.clone();
        let mut efct_limited = String::with_capacity(default_property_effect_alloc);
        let mut efct_non_limited = String::with_capacity(default_property_effect_alloc);
        let mut efct_limited_l = 0;
        let mut efct_non_limited_l = 0;
        let mut keywords = Vec::<String>::with_capacity(10);
        ph.set_font_modded(font_name, default_font_size, default_text_mod);
        process_commands(&mut efct);
        split_keywords(&mut efct, &mut keywords);
        split_limited(&efct, &mut total_limited_l, ph, &mut efct_limited, &mut efct_non_limited, &mut efct_limited_l, &mut efct_non_limited_l);

        if efct.contains(|c| c == '¤') {
            panic!("Unprocessed commands");
        }
        if efct.starts_with(' ') {
            panic!("White space in beginning of property: {efct}");
        }

        Self { 
            keywords,
            efct_limited, 
            efct_non_limited, 
            efct_non_limited_h: efct_non_limited_l as f64 * property_height, 
            efct_limited_h: efct_limited_l as f64 * property_height, 
        }
    }

    pub fn add_to_pdf(&self, ph: &PdfHandler, x: f64, mut y: f64, prop_sym_name: &str) -> f64 {
        ph.set_font_modded(font_name, default_font_size, default_text_mod);
        if !self.efct_non_limited.is_empty() {
            y -= self.efct_non_limited_h;
            ph.set_xy(x, y);
            ph.multi_cell(&self.efct_non_limited, card_inner_width, property_height, default_text_align);
        }
        if !self.efct_limited.is_empty() {
            y -= self.efct_limited_h;
            ph.set_xy(x, y);
            ph.multi_cell(&self.efct_limited, prop_top_w, property_height, default_text_align);
        }
        for keyword in self.keywords.iter().rev() {
            y -= property_height;
            ph.set_xy(x, y);
            ph.multi_cell(&keyword, card_inner_width, property_height, default_text_align);
        }
    
        ph.set_xy(x + prop_sym_pad_l, y + prop_sym_pad_t);
        ph.image(prop_sym_name, "icons", prop_sym_size, prop_sym_size);

        y - vertical_property_pad
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TextModifier {
    none, bold, italic, bold_italic
}

impl Into<&str> for TextModifier {
    fn into(self) -> &'static str {
        match self {
            TextModifier::none => "",
            TextModifier::bold => "B",
            TextModifier::italic => "I",
            TextModifier::bold_italic => "BI"
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    left, center, right
}

impl Into<&str> for Alignment {
    fn into(self) -> &'static str {
        match self {
            Alignment::left => "L",
            Alignment::center => "C",
            Alignment::right => "R"
        }
    }
}

fn process_command_name(string: &str) -> String {
    let parts = string.split('_');
    let mut result = String::with_capacity(string.len() + 20);
    for part in parts {
        result.extend(part.nth_char(0).unwrap().to_uppercase());
        result.extend(part.chars().skip(1));
        result.push(' ');
    }
    result.pop();
    result
}

fn process_commands(string: &mut String) {
    while let Some(start) = string.find('¤') {
        let substring = &string[2 + start ..];
        let mut command = String::with_capacity(string.capacity() + 20);
        command.push_str("**");
        
        if let Some(params_start) = substring.find('(') {
            let end = substring.find(')').unwrap();
            command.push_str(&process_command_name(&substring[..params_start]));
            command.push(' ');
            command.push_str(&substring[params_start + 1 .. end]);
            command.push_str("**");
            string.replace_range(start .. start + end + 3, &command);
        }
        else {
            if let Some(end) = substring.find(' ') {
                command.push_str(&process_command_name(&substring[..end]));
                command.push_str("**");
                string.replace_range(start .. start + end + 2, &command);
            }
            else {
                command.push_str(&process_command_name(&substring[..]));
                command.push_str("**");
                string.replace_range(start.., &command);
            }
        }
    } 
}

fn split_keywords(string: &mut String, main_effects: &mut Vec<String>) {
    let mut s = &string[..];
    while s.starts_with("**") {
        let substring = &s[2..];
        let end = substring.find("**").unwrap() + 4;
        let (a, b) = s.split_at(end);
        let a = a.trim();
        let b = b.trim();
        main_effects.push(a.to_string());
        s = b;
    }

    let s = s.to_string();
    string.clear();
    string.push_str(&s);
}

fn split_limited(string: &str, prev_limited_l: &mut usize, ph: &PdfHandler, limited: &mut String, non_limited: &mut String, limited_l: &mut usize, non_limited_l: &mut usize) {
    let max_limited_l = i32::max(prop_sym_l as i32 - *prev_limited_l as i32, 0) as usize;
    if max_limited_l == 0 {
        *non_limited_l = ph.multi_cell_l(&string, card_inner_width, property_height, default_text_align);
        non_limited.push_str(&string);
    }
    else {
        let (a, b) = ph.split_on_lines(&string, prop_top_w, property_height, default_text_align, max_limited_l);
        limited.push_str(a);
        non_limited.push_str(b);
        if !limited.is_empty() {
            *limited_l = ph.multi_cell_l(&limited, prop_top_w, property_height, default_text_align);
        }
        if !non_limited.is_empty() {
            *non_limited_l = ph.multi_cell_l(&non_limited, card_inner_width, property_height, default_text_align);
        }
    }

    *prev_limited_l += *limited_l;
}

fn get_height_of_properties(acti: &Vec<DeserializedProperty>, trig: &Vec<DeserializedProperty>, pass: &Vec<DeserializedProperty>) -> f64 {
    let mut h = 0.0;

    for prop in acti {
        h += prop.efct_non_limited_h;
        h += prop.efct_limited_h;
        h += prop.keywords.len() as f64 * property_height;
        h += vertical_property_pad;
    }

    for prop in trig {
        h += prop.efct_non_limited_h;
        h += prop.efct_limited_h;
        h += prop.keywords.len() as f64 * property_height;
        h += vertical_property_pad;
    }

    for prop in pass {
        h += prop.efct_non_limited_h;
        h += prop.efct_limited_h;
        h += prop.keywords.len() as f64 * property_height;
        h += vertical_property_pad;
    }

    h - vertical_property_pad
}

fn count_icon_rows(ph: &PdfHandler, card: &Card) -> usize {
    if card.attributes.len() == 0 {
        return 0;
    }

    let mut n = 1;
    let mut w = 0.0;
    for (_, value) in &card.attributes {
        w += icon_size + icon_text_pad_l + ph.string_width(&value);
        if w > card_inner_width {
            n += 1;
            w -= card_inner_width;
        }
    }

    n
}

fn add_attributes(ph: &PdfHandler, x: f64, mut y: f64, card: &Card) -> f64 {
    if card.attributes.is_empty() {
        return y;
    }

    ph.set_font_modded(font_name, icon_text_font_size, default_text_mod);
    let mut icon = 0;
    let total_width: f64 = card.attributes.iter().map(|a| icon_size + icon_text_pad_l + ph.string_width(&a.1)).sum();
    let rows = (total_width / card_inner_width).ceil();
    let average_width_per_row = total_width / rows;

    while icon < card.attributes.len() {
        let mut icons_this_row = 0;
        let mut width = 0.0;

        while width <= average_width_per_row && icon + icons_this_row < card.attributes.len() {
            let w = icon_size + icon_text_pad_l + ph.string_width(&card.attributes[icon + icons_this_row].1);
            if width + w > card_inner_width {
                break;
            }

            icons_this_row += 1;
            width += w;
        }

        let horizontal_padding = (card_outer_width - width) / icons_this_row as f64;
        let mut x = x + horizontal_padding / 2.0;
        for _ in 0..icons_this_row {
            let (image, text) = &card.attributes[icon];
            icon += 1;
            ph.set_xy(x, y);
            ph.image(&format!("{}.png", image), "icons", icon_size, icon_size);
            ph.text(text, x + icon_size + icon_text_pad_l, y + icon_text_pad_t);
            x += horizontal_padding + ph.string_width(text) + icon_size + icon_text_pad_l;
        }

        y += icon_row_height;
    }

    y
}

fn add_card(ph: &PdfHandler, card: &Card, base_x: f64, base_y: f64) {
    print(&card.name);
    let mut y = base_y;

    //card image
    let image_name = &format!("{}.png", &card.name);
    if ph.has_image(image_name, "card images") {
        ph.set_xy(base_x, base_y);
        ph.image(image_name, "card images", card_outer_width, card_outer_height);
    }

    //attribute alpha background
    //let rows = icon_data.len().div_ceil(max_icons_per_row);
    ph.set_font_modded(font_name, icon_text_font_size, default_text_mod);
    let mut h = upper_alpha_base_height + count_icon_rows(ph, card) as f64 * icon_row_height;
    
    let mut types = String::with_capacity(default_attr_string_alloc);
    for t in &card.types {
        types.push_str(t);
        types.push_str(", ");
    }

    types.pop();
    types.pop();
    process_commands(&mut types);
    if !types.is_empty() {
        ph.set_font_modded(font_name, default_font_size, attr_text_mod);
        let l = ph.multi_cell_l(&types, card_inner_width, attribute_height, attr_text_align);
        h += attribute_height * l as f64;
    }
    else if !card.attributes.is_empty() {
        h -= icon_pad_vertical;
    }

    ph.set_xy(base_x, base_y);
    ph.image(&format!("upper_{}.png", h), "alpha", card_outer_width, h);

    //collect data about deserialized properties
    ph.set_xy(base_x + card_pad - text_offset, base_y + 65.0);
    ph.set_font_modded(font_name, default_font_size, default_text_mod);
    let acti: Vec<DeserializedProperty> = card.abilities.iter().rev().map(|p| { DeserializedProperty::from_property(p, ph) }).collect();
    let trig: Vec<DeserializedProperty> = card.reactions.iter().rev().map(|p| { DeserializedProperty::from_property(p, ph) }).collect();
    let pass: Vec<DeserializedProperty> = card.traits.iter().rev().map(|p| { DeserializedProperty::from_property(p, ph) }).collect();

    //property alpha background
    ph.set_xy(base_x, base_y);
    
    //flavor text
    let mut flavor_text_h = 0.0;
    if !card.flavor_text.is_empty() {
        ph.set_font_modded(font_name, default_font_size, attr_text_mod);
        flavor_text_h = ph.multi_cell_h(&card.flavor_text, card_inner_width, property_height, default_text_align) + vertical_property_pad;
    }

    let h = get_height_of_properties(&acti, &trig, &pass) + lower_alpha_base_height + flavor_text_h;
    ph.set_xy(base_x, base_y + card_outer_height - h);
    ph.image(&format!("lower_{}.png", h), "alpha", card_outer_width, h);

    //Add the corners for heroes
    if card.commander {
        ph.set_xy(base_x, base_y);
        ph.image("commander_left.png", "icons", commander_icon_size, commander_icon_size);
        ph.set_xy(base_x + commander_offset_right, base_y);
        ph.image("commander_right.png", "icons", commander_icon_size, commander_icon_size);
    }

    //name
    ph.set_xy(base_x, y);
    ph.set_font_modded(font_name, name_font_size, name_text_mod);
    ph.multi_cell(&card.name, card_outer_width, name_h, Alignment::center);
    
    //main attributes
    y += name_h;
    y = add_attributes(ph, base_x, y, card);
    
    //other attributes
    let x = base_x + card_pad;
    if !types.is_empty() {
        ph.set_xy(x, y);
        ph.set_font_modded(font_name, default_font_size, attr_text_mod);
        ph.multi_cell(&types, card_inner_width, attribute_height, attr_text_align);
    }

    //properties
    y = base_y + card_inner_height;

    if flavor_text_h != 0.0 {
        y -= flavor_text_h - vertical_property_pad;
        ph.set_xy(x - text_offset, y);
        ph.set_font_modded(font_name, default_font_size, attr_text_mod);
        ph.multi_cell(&card.flavor_text, card_inner_width, property_height, default_text_align);
        y -= vertical_property_pad;
    }

    ph.set_font_modded(font_name, default_font_size, default_text_mod);
    for prop in &pass {
        y = prop.add_to_pdf(ph, x - text_offset, y, "trait.png");
    }
    for prop in &trig {
        y = prop.add_to_pdf(ph, x - text_offset, y, "reaction.png");
    }
    for prop in &acti {
        y = prop.add_to_pdf(ph, x - text_offset, y, "ability.png");
    }
}

fn add_cards(ph: &PdfHandler, cards: &Vec<Card>) {
    ph.set_text_color(255.0, 255.0, 255.0);
    let num_cards = cards.len();
    for page in 0..if num_cards % cards_per_page == 0 { num_cards / cards_per_page } else { num_cards / cards_per_page + 1 } {
        ph.add_page();
        let cards_left = num_cards - page * cards_per_page;
        let bg_width = std::cmp::min(cards_per_row, cards_left) as f64 * card_separation_width + 3.0;
        let bg_height = std::cmp::min(cards_per_column, (cards_left + cards_per_row - 1) / cards_per_row) as f64 * card_separation_height + 3.0;
        ph.filled_rect(page_pad_l - 3.0, page_pad_t - 3.0, bg_width, bg_height);
        for r in 0..cards_per_column {
            for c in 0..cards_per_row {
                let i = page * cards_per_page + r * cards_per_row + c;
                if i < num_cards {
                    let x = page_pad_l as f64 + c as f64 * card_separation_width;
                    let y = page_pad_t as f64 + r as f64 * card_separation_height;
                    add_card(&ph, &cards[i], x, y);
                }
                else {
                    return;
                }
            }
        }
    }
}

pub fn add_cards_to_pdf(cards: &Vec<Card>) {
    Python::with_gil(|py| {
        println!("Printing {} cards as pdf...", cards.len());
        let ph = PdfHandler::new(py);
        add_cards(&ph, cards);
        ph.output();
        println!("{} cards printed.", cards.len());
    });
}
