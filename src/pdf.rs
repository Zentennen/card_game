#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use crate::*;
use extstd::*;
use pyo3::*;
use pyo3::types::IntoPyDict;

pub const name_text_mod: TextModifier = TextModifier::bold_;
pub const attr_text_align: Alignment = Alignment::center_;
pub const attr_text_mod: TextModifier = TextModifier::italic_;
pub const font_name: &'static str = "Bitter";
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
        self.pdf.call_method("image", args, Some(kwargs)).unwrap();
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
    }
}

fn process_command(string: &str) -> String {
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
            command.push_str(&process_command(&substring[..params_start]));
            command.push(' ');
            command.push_str(&substring[params_start + 1 .. end]);
            command.push_str("**");
            string.replace_range(start .. start + end + 3, &command);
        }
        else {
            if let Some(end) = substring.find(' ') {
                command.push_str(&process_command(&substring[..end]));
                command.push_str("**");
                string.replace_range(start .. start + end, &command);
            }
            else {
                command.push_str(&process_command(&substring[..]));
                command.push_str("**");
                string.replace_range(start.., &command);
            }
        }
    } 
}

fn split_limited(string: &str, prev_limited_l: &mut usize, ph: &PdfHandler, limited: &mut String, non_limited: &mut String, limited_l: &mut usize, non_limited_l: &mut usize) {
    let max_limited_l = i32::max(prop_sym_l as i32 - *prev_limited_l as i32, 0) as usize;
    if max_limited_l == 0 {
        *non_limited_l = ph.multi_cell_l(&string, card_inner_width, prop_height, default_text_align);
        non_limited.push_str(&string);
    }
    else {
        let (a, b) = ph.split_on_lines(&string, prop_top_w, prop_height, default_text_align, max_limited_l);
        limited.push_str(a);
        non_limited.push_str(b);
        *limited_l = ph.multi_cell_l(&limited, prop_top_w, prop_height, default_text_align);
        if !non_limited.is_empty() {
            *non_limited_l = ph.multi_cell_l(&non_limited, card_inner_width, prop_height, default_text_align);
        }
    }

    *prev_limited_l += *limited_l;
}

pub struct DeserializedProperty {
    pub efct_limited: String,
    pub efct_non_limited: String,
    pub attr_limited: String,
    pub attr_non_limited: String,
    pub efct_limited_l: usize,
    pub efct_non_limited_l: usize,
    pub attr_limited_l: usize,
    pub attr_non_limited_l: usize,
}

impl DeserializedProperty {
    pub fn from_property(prop: &Property, ph: &PdfHandler) -> DeserializedProperty {
        let mut total_limited_l = 0;

        let mut attr_limited = String::with_capacity(default_attr_string_alloc);
        let mut attr_non_limited = String::with_capacity(default_attr_string_alloc);
        let mut attr_non_limited_l = 0;
        let mut attr_limited_l = 0;
        if !prop.attr.is_empty() {
            ph.set_font_modded(font_name, default_font_size, attr_text_mod);
            let mut attr = String::with_capacity(default_attr_string_alloc);
            for at in &prop.attr {
                add_attr_to_string(at, &mut attr);
                attr.push_str(", ");
            }
            attr.pop();
            attr.pop();
            process_commands(&mut attr);
            split_limited(&attr, &mut total_limited_l, ph, &mut attr_limited, &mut attr_non_limited, &mut attr_limited_l, &mut attr_non_limited_l);
        }

        let mut efct = prop.efct.to_string();
        let mut efct_limited = String::with_capacity(default_property_effect_alloc);
        let mut efct_non_limited = String::with_capacity(default_property_effect_alloc);
        let mut efct_limited_l = 0;
        let mut efct_non_limited_l = 0;
        ph.set_font_modded(font_name, default_font_size, default_text_mod);
        process_commands(&mut efct);
        split_limited(&efct, &mut total_limited_l, ph, &mut efct_limited, &mut efct_non_limited, &mut efct_limited_l, &mut efct_non_limited_l);

        Self{ efct_limited, efct_non_limited, attr_limited, attr_non_limited, efct_non_limited_l, attr_non_limited_l, efct_limited_l, attr_limited_l }
    }

    pub fn add_to_pdf(&self, ph: &PdfHandler, base_x: f64, mut y: f64, prop_sym_name: &str) -> f64 {
        let x = base_x;

        //ph.set_xy(x, y + prop_sym_pad_t);
        //ph.image(prop_sym_name, "icons", prop_sym_size, prop_sym_size);

        ph.set_font_modded(font_name, default_font_size, default_text_mod);
        if !self.efct_non_limited.is_empty() {
            y -= self.efct_non_limited_l as f64 * prop_height;
            ph.set_xy(x, y);
            ph.multi_cell(&self.efct_non_limited, card_inner_width, prop_height, default_text_align);
        }
        if !self.efct_limited.is_empty() {
            y -= self.efct_limited_l as f64 * prop_height;
            ph.set_xy(x, y);
            ph.multi_cell(&self.efct_limited, prop_top_w, prop_height, default_text_align);
        }
        
        ph.set_font_modded(font_name, default_font_size, attr_text_mod);
        if !self.attr_non_limited.is_empty() {
            y -= self.attr_non_limited_l as f64 * prop_height;
            ph.set_xy(x, y);
            ph.multi_cell(&self.attr_non_limited, card_inner_width, prop_height, default_text_align);
        }
        if !self.attr_limited.is_empty() {
            y -= self.attr_limited_l as f64 * prop_height;
            ph.set_xy(x, y);
            ph.multi_cell(&self.attr_limited, prop_top_w, prop_height, default_text_align);
        }
    
        ph.set_xy(x + prop_sym_pad_l, y + prop_sym_pad_t);
        ph.image(prop_sym_name, "icons", prop_sym_size, prop_sym_size);

        y -= prop_half_height;
        y
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

pub fn get_property_halflines(acti: &Vec<DeserializedProperty>, trig: &Vec<DeserializedProperty>, pass: &Vec<DeserializedProperty>) -> usize {
    let mut h = 0;

    for prop in acti {
        h += prop.attr_non_limited_l * 2;
        h += prop.attr_limited_l * 2;
        h += prop.efct_non_limited_l * 2;
        h += prop.efct_limited_l * 2;
        h += 1;
    }

    for prop in trig {
        h += prop.attr_non_limited_l * 2;
        h += prop.attr_limited_l * 2;
        h += prop.efct_non_limited_l * 2;
        h += prop.efct_limited_l * 2;
        h += 1;
    }

    for prop in pass {
        h += prop.attr_non_limited_l * 2;
        h += prop.attr_limited_l * 2;
        h += prop.efct_non_limited_l * 2;
        h += prop.efct_limited_l * 2;
        h += 1;
    }

    //there will be one extra halfline added from the last property that doesn't need to be there, unless there are no properties.
    if !acti.is_empty() || !trig.is_empty() || !pass.is_empty() {
        h -= 1;
    }

    h
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

    if let Some(val) = get_attribute_value(&card.attr, "Strength") {
        if val != 1.0 {
            attribs.push(("fist.png", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Health") {
        if val != 1.0 {
            attribs.push(("heart.png", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Power") {
        if val != 0.0 {
            attribs.push(("star.png", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Speed") {
        if val != 1.0 {
            attribs.push(("boot.png", val.to_string()));
        }
    }

    attribs
}

pub fn get_other_attr_string(card: &Card) -> String {
    let mut string = String::with_capacity(default_attr_string_alloc);
    for attr in &card.attr {
        match &attr.n as &str {
            "Tribute" | "Offense" | "Defense" | "Health" | "Strength" | "Power" | "Advanced" | "Speed" => continue,
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

pub fn add_entity_to_pdf(ph: &PdfHandler, card: &Card, base_x: f64, base_y: f64, hero: bool) {
    let image_name = &format!("{}.png", &card.name);
    if ph.has_image(image_name, "card images") {
        ph.set_xy(base_x, base_y);
        ph.image(image_name, "card images", card_outer_width, card_outer_height);
    }
    //else {
    //    ph.rect(base_x, base_y, card_outer_width, card_outer_height);
    //}

    //attribute alpha background
    let main_attr_icon_data = get_main_attr_icon_data(card);
    let mut h = upper_alpha_base_height;
    let mut l = 0;
    let mut other_attr = get_other_attr_string(card);
    process_commands(&mut other_attr);
    if !other_attr.is_empty() {
        h += main_attr_pad_b;
        ph.set_font_modded(font_name, default_font_size, attr_text_mod);
        l = ph.multi_cell_l(&other_attr, card_inner_width, other_attr_height, attr_text_align);
        h += other_attr_height * l as f64;
    }
    ph.set_xy(base_x, base_y);
    ph.image(&format!("upper{}.png", l), "alpha", card_outer_width, h);

    //collect data about deserialized properties
    ph.set_xy(base_x + card_pad - text_offset, base_y + 65.0);
    ph.set_font_modded(font_name, default_font_size, default_text_mod);
    let acti: Vec<DeserializedProperty> = card.acti.iter().rev().map(|p| { DeserializedProperty::from_property(p, ph)}).collect();
    let trig: Vec<DeserializedProperty> = card.trig.iter().rev().map(|p| { DeserializedProperty::from_property(p, ph)}).collect();
    let pass: Vec<DeserializedProperty> = card.pass.iter().rev().map(|p| { DeserializedProperty::from_property(p, ph)}).collect();

    //property alpha background
    ph.set_xy(base_x, base_y);
    let l = get_property_halflines(&acti, &trig, &pass);
    let h = l as f64 * prop_half_height + alpha_gradient_height + card_pad;
    ph.set_xy(base_x, base_y + card_outer_height - h);
    ph.image(&format!("lower{}.png", l), "alpha", card_outer_width, h);

    //Add the corners for advanced cards
    if hero {
        ph.set_xy(base_x, base_y);
        ph.image("advanced_tl.png", "icons", advanced_sym_size, advanced_sym_size);
        ph.set_xy(base_x + advanced_offset_r, base_y);
        ph.image("advanced_tr.png", "icons", advanced_sym_size, advanced_sym_size);
    }

    let mut y = base_y;
    
    //name
    ph.set_xy(base_x, y);
    ph.set_font_modded(font_name, name_font_size, name_text_mod);
    ph.multi_cell(&card.name, card_outer_width, name_h, Alignment::center_);
    
    //main attributes
    ph.set_font_modded(font_name, main_attr_font_size, default_text_mod);
    y += name_h;
    if !main_attr_icon_data.is_empty() {
        let step_w = main_attr_weight / main_attr_icon_data.len() as f64;
        let last_main_attr_w = ph.string_w(&main_attr_icon_data[main_attr_icon_data.len() - 1].1) + main_attr_height + main_attr_text_pad_l;
        let w = step_w * (main_attr_icon_data.len() - 1) as f64 + last_main_attr_w;
        let base_x = base_x + (main_attr_weight - w) / 2.0 + main_attr_pad_lr;
        for (i, (icon, val)) in main_attr_icon_data.iter().enumerate() {
            let x = base_x + i as f64 * step_w;
            ph.set_xy(x, y);
            ph.image(icon, "icons", main_attr_height, main_attr_height);
            ph.text(&val, x + main_attr_height + main_attr_text_pad_l, y + main_attr_text_pad_t);
        }
    }

    let x = base_x + card_pad;

    //other attributes
    if !other_attr.is_empty() {
        y += main_attr_height + main_attr_pad_b;
        ph.set_xy(x, y);
        ph.set_font_modded(font_name, default_font_size, attr_text_mod);
        ph.multi_cell(&other_attr, card_inner_width, other_attr_height, attr_text_align);
    }

    //properties
    ph.set_font_modded(font_name, default_font_size, default_text_mod);
    y = base_y + card_inner_height;
    for prop in pass {
        y = prop.add_to_pdf(ph, x - text_offset, y, "passive.png");
    }
    for prop in trig {
        y = prop.add_to_pdf(ph, x - text_offset, y, "triggered.png");
    }
    for prop in acti {
        y = prop.add_to_pdf(ph, x - text_offset, y, "action.png");
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
                    let x = page_pad_l as f64 + c as f64 * card_separation_width;
                    let y = page_pad_t as f64 + r as f64 * card_separation_height;
                    add_entity_to_pdf(&ph, &cards[i], x, y, false);
                }
                else {
                    return;
                }
            }
        }
    }
}

pub fn add_all_cards_to_pdf(cards: &Vec<Card>) {
    Python::with_gil(|py| {
        println!("Printing {} cards as pdf...", cards.len());
        let ph = PdfHandler::new(py);
        add_cards_to_pdf(&ph, cards);
        ph.output();
        println!("{} cards printed.", cards.len());
    });
}
