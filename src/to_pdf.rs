#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::*;
use extstd::*;
use pyo3::*;
use pyo3::types::IntoPyDict;

pub const attr_text_align: Alignment = Alignment::center_;
pub const attr_text_mod: TextModifier = TextModifier::italic_;
pub const name_font_size: f64 = 9.0;
pub const name_text_mod: TextModifier = TextModifier::bold_;
pub const font_size: f64 = 6.5;
pub const font_name: &'static str = "Merriweather";
pub const font_line_width: f64 = 0.1;
pub const rect_line_w: f64 = 0.2;
pub const default_text_align: Alignment = Alignment::left_;
pub const default_text_mod: TextModifier = TextModifier::none_;

pub struct PdfHandler<'p> {
    py: Python<'p>,
    pdf: &'p PyAny,
    image_aliases: std::collections::HashMap<&'static str, &'static str>
}

impl PdfHandler<'_> {
    pub fn new<'p>(py: Python<'p>) -> PdfHandler<'p> {
        let fpdf = py.import("fpdf").unwrap();
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
        let entries = std::fs::read_dir("deserialize/fonts").expect("Could not find directory deserialize/fonts");
        for entry in entries {
            if let Result::Ok(entry) = entry {
                let name = entry.path().file_stem().unwrap().to_str().unwrap().to_string();
                let base_path = entry.path().as_path().to_str().unwrap().to_string();
                
                if entry.metadata().unwrap().is_dir() {
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
        self.pdf.call_method("set_xy", (), Some(args)).unwrap();
    }

    pub fn cell(&self, txt: &str, w: f64, h: f64) {
        let args = (w, h, txt);
        self.pdf.call_method1("cell", args).unwrap();
    }

    pub fn center_cell(&self, txt: &str, w: f64, h: f64) {
        let args = (w, h, txt);
        let kwargs = [("align", "C")].into_py_dict(self.py);
        self.pdf.call_method("cell", args, Some(kwargs)).unwrap();
    }

    pub fn multi_cell(&self, txt: &str, w: f64, h: f64, align: Alignment) {
        let args: (f64, f64, &str, i32, &str) = (w, h, txt, 0, align.into());
        self.pdf.call_method1("multi_cell", args).unwrap();
    }

    pub fn multi_cell_h(&self, txt: &str, w: f64, h: f64, align: Alignment) -> f64 {
        let args: (f64, f64, &str, i32, &str) = (w, h, txt, 0, align.into());
        let kwargs = [("split_only", true)].into_py_dict(self.py);
        let strings = self.pdf.call_method("multi_cell", args, Some(kwargs)).expect("ERROR!!!").to_object(self.py);
        let strings: Vec<&str> = strings.extract(self.py).unwrap();
        h * strings.len() as f64
    }

    pub fn multi_cell_l(&self, txt: &str, w: f64, h: f64, align: Alignment) -> usize {
        let args: (f64, f64, &str, i32, &str) = (w, h, txt, 0, align.into());
        let kwargs = [("split_only", true)].into_py_dict(self.py);
        let strings = self.pdf.call_method("multi_cell", args, Some(kwargs)).unwrap().to_object(self.py);
        let strings: Vec<&str> = strings.extract(self.py).unwrap();
        strings.len()
    }

    fn text_on_limited_lines<'a>(&self, txt: &'a str, w: f64, h: f64, align: Alignment, max_lines: usize) -> (&'a str, &'a str) {
        let last = txt.num_chars() - 1;
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
        let path = format!("deserialize/{}/{}", folder, name);
        let args = types::PyTuple::new(self.py, &[path]);
        let kwargs = [("w", w), ("h", h)].into_py_dict(self.py);
        self.pdf.call_method("image", args, Some(kwargs)).unwrap();
    }

    pub fn has_image(&self, name: &str, folder: &str) -> bool {
        self.image_aliases.contains_key(name) || std::fs::metadata(&format!("deserialize/{}/{}", folder, name)).is_ok()
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

    pub fn string_w(&self, string: &str) -> f64 {
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
        print("Deserialized successfully\n");
    }
}

fn split_limited(string: &str, prev_limited_l: &mut usize, ph: &PdfHandler, limited: &mut String, non_limited: &mut String, limited_l: &mut usize, non_limited_l: &mut usize) {
    let string = process_commands(string);
    let max_limited_l = i32::max(prop_sym_l as i32 - *prev_limited_l as i32, 0) as usize;
    if max_limited_l == 0 {
        *non_limited_l = ph.multi_cell_l(&string, card_inner_w, prop_h, default_text_align);
        non_limited.push_str(&string);
    }
    else {
        let (a, b) = ph.text_on_limited_lines(&string, prop_top_w, prop_h, default_text_align, max_limited_l);
        limited.push_str(a);
        non_limited.push_str(b);
        *limited_l = ph.multi_cell_l(&limited, prop_top_w, prop_h, default_text_align);
        if !non_limited.is_empty() {
            *non_limited_l = ph.multi_cell_l(&non_limited, card_inner_w, prop_h, default_text_align);
        }
    }

    *prev_limited_l += *limited_l;
}

pub struct DeserializedProperty {
    pub name_limited: String,
    pub name_non_limited: String,
    pub efct_limited: String,
    pub efct_non_limited: String,
    pub attr_limited: String,
    pub attr_non_limited: String,
    pub name_limited_l: usize,
    pub name_non_limited_l: usize,
    pub efct_limited_l: usize,
    pub efct_non_limited_l: usize,
    pub attr_limited_l: usize,
    pub attr_non_limited_l: usize,
}


impl DeserializedProperty {
    pub fn from_property(prop: &Property, ph: &PdfHandler) -> DeserializedProperty {
        let mut total_limited_l = 0;

        let mut name_limited = String::with_capacity(default_name_string_alloc);
        let mut name_non_limited = String::with_capacity(default_name_string_alloc);
        let mut name_limited_l = 0;
        let mut name_non_limited_l = 0;
        if !prop.name.is_empty() {
            split_limited(&prop.name, &mut total_limited_l, ph, &mut name_limited, &mut name_non_limited, &mut name_limited_l, &mut name_non_limited_l);
        }

        let mut attr_limited = String::with_capacity(default_attr_string_alloc);
        let mut attr_non_limited = String::with_capacity(default_attr_string_alloc);
        let mut attr_non_limited_l = 0;
        let mut attr_limited_l = 0;
        if !prop.attr.is_empty() {
            ph.set_font_modded(font_name, font_size, attr_text_mod);
            
            let mut attr = String::with_capacity(default_attr_string_alloc);
            for at in &prop.attr {
                add_attr_to_string(at, &mut attr);
                attr.push_str(", ");
            }
            attr.pop();
            attr.pop();
            attr = process_commands(&attr);

            split_limited(&attr, &mut total_limited_l, ph, &mut attr_limited, &mut attr_non_limited, &mut attr_limited_l, &mut attr_non_limited_l);

            ph.set_font_modded(font_name, font_size, default_text_mod);
        }

        let mut efct_limited = String::with_capacity(default_property_effect_alloc);
        let mut efct_non_limited = String::with_capacity(default_property_effect_alloc);
        let mut efct_limited_l = 0;
        let mut efct_non_limited_l = 0;
        split_limited(&prop.efct, &mut total_limited_l, ph, &mut efct_limited, &mut efct_non_limited, &mut efct_limited_l, &mut efct_non_limited_l);

        Self{ name_limited, name_non_limited, efct_limited, efct_non_limited, attr_limited, attr_non_limited, name_limited_l, name_non_limited_l, efct_non_limited_l, attr_non_limited_l, efct_limited_l, attr_limited_l }
    }

    pub fn add_to_pdf(&self, ph: &PdfHandler, base_x: f64, mut y: f64, prop_sym_name: &str) -> f64 {
        if !self.efct_non_limited.is_empty() {
            y -= self.efct_non_limited_l as f64 * prop_h;
            ph.set_xy(base_x, y);
            ph.multi_cell(&self.efct_non_limited, card_inner_w, prop_h, default_text_align);
        }
        if !self.efct_limited.is_empty() {
            y -= self.efct_limited_l as f64 * prop_h;
            ph.set_xy(base_x, y);
            ph.multi_cell(&self.efct_limited, prop_top_w, prop_h, default_text_align);
        }
        
        if !self.attr_non_limited.is_empty() {
            y -= self.attr_non_limited_l as f64 * prop_h;
            ph.set_font_modded(font_name, font_size, attr_text_mod);
            ph.set_xy(base_x, y);
            ph.multi_cell(&self.attr_non_limited, card_inner_w, prop_h, default_text_align);
        }
        if !self.attr_limited.is_empty() {
            y -= self.attr_limited_l as f64 * prop_h;
            ph.set_font_modded(font_name, font_size, attr_text_mod);
            ph.set_xy(base_x, y);
            ph.multi_cell(&self.attr_limited, prop_top_w, prop_h, default_text_align);
        }

        if !self.name_non_limited.is_empty() {
            y -= self.name_non_limited_l as f64 * prop_h;
            ph.set_font_modded(font_name, font_size, name_text_mod);
            ph.set_xy(base_x, y);
            ph.multi_cell(&self.name_non_limited, prop_top_w, prop_h, default_text_align);
        }
        if !self.name_non_limited.is_empty() {
            y -= self.name_limited_l as f64 * prop_h;
            ph.set_font_modded(font_name, font_size, name_text_mod);
            ph.set_xy(base_x, y);
            ph.multi_cell(&self.name_limited, card_inner_w, prop_h, default_text_align);
        }
    
        ph.set_xy(base_x + prop_sym_pad_l, y + prop_sym_pad_t);
        ph.image(prop_sym_name, "icons", prop_sym_size, prop_sym_size);

        y -= prop_h;
        y
    }

    pub fn get_height_sum(acti: &Vec<DeserializedProperty>, trig: &Vec<DeserializedProperty>, pass: &Vec<DeserializedProperty>) -> usize {
        let mut h = 0;
        for prop in acti {
            h += prop.attr_non_limited_l;
            h += prop.attr_limited_l;
            h += prop.efct_non_limited_l;
            h += prop.efct_limited_l;
            h += 1;
        }
        for prop in trig {
            h += prop.attr_non_limited_l;
            h += prop.attr_limited_l;
            h += prop.efct_non_limited_l;
            h += prop.efct_limited_l;
            h += 1;
        }
        for prop in pass {
            h += prop.attr_non_limited_l;
            h += prop.attr_limited_l;
            h += prop.efct_non_limited_l;
            h += prop.efct_limited_l;
            h += 1;
        }
        if !acti.is_empty() || !trig.is_empty() || !pass.is_empty() {
            h -= 1;
        }
        h
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TextModifier {
    none_, bold_, italic_, bold_italic_
}

impl Into<&str> for TextModifier {
    fn into(self) -> &'static str {
        match self {
            TextModifier::none_ => "",
            TextModifier::bold_ => "B",
            TextModifier::italic_ => "I",
            TextModifier::bold_italic_ => "BI"
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    left_, center_, right_
}

impl Into<&str> for Alignment {
    fn into(self) -> &'static str {
        match self {
            Alignment::left_ => "L",
            Alignment::center_ => "C",
            Alignment::right_ => "R"
        }
    }
}

fn get_attr_condition_string(string: &str) -> String {
    let attrs = string.split(',');
    let mut attr_string = String::with_capacity(100);
    for attr in attrs {
        let attr = attr.trim();
        if attr.starts_with('!') { 
            attr_string.push_str("non-");
            attr_string.push_str(&attr[1..]);
        } 
        else { 
            attr_string.push_str(attr);
        };
        attr_string.push_str(", ");
    }
    attr_string.pop();
    attr_string.pop();
    attr_string
}

pub fn process_commands(string: &str) -> String {
    if string.is_empty() {
        return String::new();
    }

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
    while let Some(pos) = string.find("¤(Cswa(") {
        let start = pos + 8;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Cswa()) was not correctly terminated");
        let attr_string = get_attr_condition_string(&string[start..end]);
        let replacement;
        if attr_string.starts_with('n') {
            replacement = format!("N{} cards", &attr_string[1..]);
        }
        else {
            replacement = format!("{} cards", &attr_string);
        }
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(cswa(") {
        let start = pos + 8;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(cswa()) was not correctly terminated");
        let replacement = format!("{} cards", get_attr_condition_string(&string[start..end]));
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Cwa(") {
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Cwa()) was not correctly terminated");
        let attr_string = get_attr_condition_string(&string[start..end]);
        let replacement;
        if attr_string.starts_with('n') {
            replacement = format!("N{} card", &attr_string[1..]);
        }
        else {
            replacement = format!("{} card", &attr_string);
        }
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(cwa(") {
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(cwa()) was not correctly terminated");
        let replacement = format!("{} card", get_attr_condition_string(&string[start..end]));
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Pay(") {
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(Pay()) was not correctly terminated");
        let replacement = format!("Pay {}", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(pay(") {
        let start = pos + 7;
        let end = start + string[start..].find(')').expect("EXPECT ERR: ¤(pay()) was not correctly terminated");
        let replacement = format!("pay {}", &string[start..end]);
        string.replace_range(pos..end + 2, &replacement);
    }
    while let Some(pos) = string.find("¤(Sum(") {
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

pub fn add_attr_to_string(attr: &Attribute, string: &mut String) {
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

pub fn get_main_attr_icon_data(card: &Card) -> Vec<(&str, String)> {
    let mut attribs: Vec<(&str, String)> = Vec::new();

    if let Some(val) = get_attribute_value(&card.attr, "Tribute") {
        if val != 0.0 {
            attribs.push(("drop.png", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Offense") {
        if val != 0.0 {
            attribs.push(("sword.png", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Defense") {
        if val != 0.0 {
            attribs.push(("shield.png", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Health") {
        if val != 1.0 {
            attribs.push(("heart.png", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Strength") {
        if val != 1.0 {
            attribs.push(("fist.png", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Power") {
        if val != 0.0 {
            attribs.push(("star.png", val.to_string()));
        }
    }

    attribs
}

pub fn get_other_attr_string(card: &Card) -> String {
    let mut string = String::with_capacity(default_attr_string_alloc);
    for attr in &card.attr {
        match &attr.n as &str {
            "Tribute" | "Offense" | "Defense" | "Health" | "Strength" | "Power" | "Advanced" => continue,
            _ => {
                add_attr_to_string(attr, &mut string);
                string.push_str(", ");
            }
        }
    }
    string.pop();
    string.pop();
    string
}

pub fn organize_property_data<'a>(card: &'a Card, 
    ph: &PdfHandler,
    short_acti: &mut Vec<&'a str>, 
    short_trig: &mut Vec<&'a str>, 
    short_pass: &mut Vec<&'a str>, 
    acti: &mut Vec<DeserializedProperty>, 
    trig: &mut Vec<DeserializedProperty>, 
    pass: &mut Vec<DeserializedProperty>
) 
{
    for prop in &card.pass {
        if prop.attr.is_empty() && prop.efct.is_empty() {
            short_pass.push(&prop.name);
        }
        else {
            pass.push(DeserializedProperty::from_property(&prop, ph))
        }
    }
    for prop in &card.trig {
        if prop.attr.is_empty() && prop.efct.is_empty() {
            short_trig.push(&prop.name);
        }
        else {
            trig.push(DeserializedProperty::from_property(&prop, ph))
        }
    }
    for prop in &card.acti {
        if prop.attr.is_empty() && prop.efct.is_empty() {
            short_acti.push(&prop.name);
        }
        else {
            acti.push(DeserializedProperty::from_property(&prop, ph))
        }
    }
}

pub fn add_short_prop_to_pdf(ph: &PdfHandler, prop_name: &str,  base_x: f64, mut y: f64, prop_sym_name: &str) -> f64 {
    let l = ph.multi_cell_l(prop_name, card_inner_w, prop_h, Alignment::left_);
    y -= l as f64 * prop_h;
    
    ph.set_xy(base_x, y + prop_sym_pad_t);
    ph.image(prop_sym_name, "icons", prop_sym_size, prop_sym_size);
    
    ph.set_xy(base_x + prop_sym_size, y);
    ph.multi_cell(prop_name, card_inner_w, prop_h, Alignment::left_);

    y
}

pub fn add_card_to_pdf(ph: &PdfHandler, card: &Card, base_x: f64, base_y: f64) {
    let image_name = &format!("{}.png", &card.name);
    if ph.has_image(image_name, "cards") {
        ph.set_xy(base_x, base_y);
        ph.image(image_name, "cards", card_outer_w, card_outer_h);
    }
    else {
        ph.rect(base_x, base_y, card_outer_w, card_outer_h);
    }

    //attribute alpha background
    let mut h = card_upper_alpha_h_short;
    let mut l = 0;
    let other_attr = process_commands(&get_other_attr_string(card));
    if !other_attr.is_empty() {
        h += main_attr_pad_b;
        ph.set_font_modded(font_name, font_size, attr_text_mod);
        l = ph.multi_cell_l(&other_attr, card_inner_w, other_attr_h, attr_text_align);
        h += other_attr_h * l as f64;
    }
    ph.set_xy(base_x, base_y);
    ph.image(&format!("upper{}.png", l), "alpha", card_outer_w, h);

    //collect data about deserialized properties
    ph.set_xy(base_x + card_pad - text_offset, base_y + 65.0);
    ph.set_font_modded(font_name, font_size, default_text_mod);
    let mut short_acti: Vec<&str> = Vec::with_capacity(card.acti.len());
    let mut short_trig: Vec<&str> = Vec::with_capacity(card.trig.len());
    let mut short_pass: Vec<&str> = Vec::with_capacity(card.pass.len());
    let mut acti: Vec<DeserializedProperty> = Vec::with_capacity(card.acti.len());
    let mut trig: Vec<DeserializedProperty> = Vec::with_capacity(card.trig.len());
    let mut pass: Vec<DeserializedProperty> = Vec::with_capacity(card.pass.len());
    organize_property_data(card, ph, &mut short_acti, &mut short_trig, &mut short_pass, &mut acti, &mut trig, &mut pass);

    //property alpha background
    ph.set_xy(base_x, base_y);
    let l = DeserializedProperty::get_height_sum(&acti, &trig, &pass);
    let short_l = ph.get_height_sum(&short_acti, card_inner_w - prop_sym_size, prop_h, default_text_align)
        + ph.get_height_sum(&short_trig, card_inner_w - prop_sym_size, prop_h, default_text_align)
        + ph.get_height_sum(&short_pass, card_inner_w - prop_sym_size, prop_h, default_text_align);
    let l = if short_l == 0 { l } else { l + short_l + 1 };
    let h = l as f64 * prop_h + gradient_h + card_pad;
    ph.set_xy(base_x, base_y + card_outer_h - h);
    ph.image(&format!("lower{}.png", l), "alpha", card_outer_w, h);

    //Add the corners for advanced cards
    if let Some(_) = get_attribute_ref_with_name(&card.attr, "Advanced") {
        ph.set_xy(base_x, base_y);
        ph.image("advanced_tl.png", "icons", advanced_sym_size, advanced_sym_size);
        ph.set_xy(base_x + advanced_offset_r, base_y);
        ph.image("advanced_tr.png", "icons", advanced_sym_size, advanced_sym_size);
    }

    //set the x and y coordinates to be inside the card margins
    let base_x = base_x + card_pad;
    let mut y = base_y;

    //name
    ph.set_xy(base_x, y);
    ph.set_font_modded(font_name, name_font_size, name_text_mod);
    ph.center_cell(&card.name, card_inner_w, name_h);

    //main attributes
    ph.set_font_modded(font_name, main_attr_font_size, default_text_mod);
    y += name_h;
    let main_attr_icon_data = get_main_attr_icon_data(card);
    if !main_attr_icon_data.is_empty() {
        let step_w = card_inner_w / main_attr_icon_data.len() as f64;
        let a = ph.string_w(&main_attr_icon_data[main_attr_icon_data.len() - 1].1);
        let w = step_w * (main_attr_icon_data.len() - 1) as f64 + main_attr_icon_w + main_attr_text_pad_l + a;
        let margin = (card_inner_w - w) / 2.0;
        for (i, (icon, val)) in main_attr_icon_data.iter().enumerate() {
            let x = base_x + margin + i as f64 * step_w;
            ph.set_xy(x, y);
            ph.image(icon, "icons", main_attr_icon_w, main_attr_h);
            ph.text(&val, x + main_attr_icon_w + main_attr_text_pad_l, y + main_attr_text_pad_t);
        }
    }

    //other attributes
    if !other_attr.is_empty() {
        y += main_attr_h + main_attr_pad_b;
        ph.set_xy(base_x, y);
        ph.set_font_modded(font_name, font_size, attr_text_mod);
        ph.multi_cell(&other_attr, card_inner_w, other_attr_h, attr_text_align);
    }

    //properties
    ph.set_font_modded(font_name, font_size, default_text_mod);
    y = base_y + card_inner_h;
    for prop in pass {
        y = prop.add_to_pdf(ph, base_x - text_offset, y, "passive.png");
    }
    for prop in trig {
        y = prop.add_to_pdf(ph, base_x - text_offset, y, "triggered.png");
    }
    for prop in acti {
        y = prop.add_to_pdf(ph, base_x - text_offset, y, "action.png");
    }
    ph.set_font_modded(font_name, font_size, name_text_mod);
    for prop in short_pass {
        y = add_short_prop_to_pdf(ph, prop, base_x, y, "passive.png");
    }
    for prop in short_trig {
        y = add_short_prop_to_pdf(ph, prop, base_x, y, "triggered.png");
    }
    for prop in short_acti {
        y = add_short_prop_to_pdf(ph, prop, base_x, y, "action.png");
    }
}


pub fn add_cards_to_pdf(ph: &PdfHandler, cards: &Vec<Card>) {
    ph.set_text_color(255.0, 255.0, 255.0);
    let num_cards = cards.len();
    for p in 0..if num_cards % cards_per_page == 0 { num_cards / cards_per_page } else { num_cards / cards_per_page + 1 } {
        ph.add_page();
        for r in 0..cards_per_column {
            for c in 0..cards_per_row {
                let i = p * cards_per_page + r * cards_per_row + c;
                if i < num_cards {
                    let x = page_pad_l as f64 + c as f64 * card_outer_w;
                    let y = page_pad_t as f64 + r as f64 * card_outer_h;
                    add_card_to_pdf(&ph, &cards[i], x, y)
                }
                else {
                    return;
                }
            }
        }
    }
}

pub fn add_all_cards_to_pdf() {
    Python::with_gil(|py| {
        let ph = PdfHandler::new(py);
        let cards = std::fs::read_to_string("cards.json").unwrap();
        let cards: Vec<Card> = serde_json::from_str(&cards).unwrap();
        add_cards_to_pdf(&ph, &cards);
        ph.output();
    });
}
