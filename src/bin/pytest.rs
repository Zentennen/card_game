#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::os::windows::process;

use extrust::*;
use card_game::*;
use pyo3::*;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict};

const page_pad_t: f64 = 5.0;
const page_pad_l: f64 = 12.0;
const cards_per_column: usize = 3;
const cards_per_row: usize = 3;
const cards_per_page: usize = cards_per_column * cards_per_row;
const card_outer_w: f64 = 63.0;
const card_outer_h: f64 = 88.0;
const card_inner_w: f64 = card_outer_w - card_pad * 2.0;
const card_inner_h: f64 = card_outer_h - card_pad;
const card_pad: f64 = 2.0;
const name_h: f64 = 8.0;
const name_font_size: f64 = 9.0;
const main_attr_icon_w: f64 = 3.6;
const main_attr_text_pad_t: f64 = main_attr_font_size * 0.38;
const main_attr_text_pad_l: f64 = 0.8;
const main_attr_h: f64 = 3.6;
const main_attr_pad_b: f64 = 1.7;
const other_attr_h: f64 = 2.7;
const prop_h: f64 = 2.5;
const prop_pad: f64 = 2.3;
const main_attr_font_size: f64 = 7.0;
const font_size: f64 = 6.0;
const default_card_string_alloc: usize = 1000;

pub struct PdfHandler<'p> {
    py: Python<'p>,
    pdf: &'p PyAny,
    image_aliases: std::collections::HashMap<&'static str, &'static str>
}

impl Drop for PdfHandler<'_> {
    fn drop(&mut self) {
        self.output();
        println!("Deserialized successfully!");
    }
}

impl PdfHandler<'_> {
    pub fn new<'p>(py: Python<'p>) -> PdfHandler<'p> {
        let fpdf = py.import("fpdf").unwrap();
        let pdf = fpdf.getattr("FPDF").unwrap().call0().unwrap();
        let mut image_aliases = std::collections::HashMap::new();
        image_aliases.insert("Test Complicated Card.png", "Test Simple Card.png");

        let s = PdfHandler::<'p> { py, pdf, image_aliases };
        s.init();
        s
    }

    pub fn init(&self) {
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

    pub fn add_page(&self) {
        self.pdf.call_method0("add_page").unwrap();
    }

    pub fn set_font(&self, family: &str, size: f64) {
        let args = (family, "", size);
        self.pdf.call_method1("set_font", args).unwrap();
    }

    pub fn set_font_i(&self, family: &str, size: f64) {
        let args = (family, "I", size);
        self.pdf.call_method1("set_font", args).unwrap();
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

    pub fn multi_cell(&self,txt: &str, w: f64, h: f64) {
        let args = (w, h, txt);
        self.pdf.call_method1("multi_cell", args).unwrap();
    }

    pub fn center_multi_cell(&self, txt: &str, w: f64, h: f64) {
        let args = (w, h, txt);
        let kwargs = [("align", "C")].into_py_dict(self.py);
        self.pdf.call_method("multi_cell", args, Some(kwargs)).unwrap();
    }

    pub fn image(&self, name: &str, w: f64, h: f64) {
        let name = *self.image_aliases.get(name).unwrap_or(&name);
        let path = format!("deserialize/images/{}", name);
        let args = types::PyTuple::new(self.py, &[path]);
        let kwargs = [("w", w), ("h", h)].into_py_dict(self.py);
        self.pdf.call_method("image", args, Some(kwargs)).unwrap();
    }

    pub fn has_image(&self, name: &str) -> bool {
        cout(name);
        self.image_aliases.contains_key(name) || std::fs::metadata(&format!("deserialize/images/{}", name)).is_ok()
    }

    pub fn rect(&self, x: f64, y: f64, w: f64, h: f64) {
        let args = (x, y, w, h);
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

    pub fn multi_cell_h(&self, txt: &str, w: f64, h: f64) -> f64 {
        let args = (w, h, txt);
        let kwargs = [("split_only", true)].into_py_dict(self.py);
        let strings = self.pdf.call_method("multi_cell", args, Some(kwargs)).unwrap().to_object(self.py);
        let strings: Vec<&str> = strings.extract(self.py).unwrap();
        h * strings.len() as f64
    }

    pub fn output(&self) {
        let args = types::PyTuple::new(self.py, &["cards.pdf"]);
        self.pdf.call_method1("output", args).unwrap();
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

fn get_main_attr_icon_data(card: &Card) -> Vec<(&str, String)> {
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
            attribs.push(("heart.jpg", val.to_string()));
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Lethality") {
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

fn deserialize_prop(ph: &PdfHandler, prop: &Property, base_x: f64, y: &mut f64) {
    let efct = process_commands(prop.efct.to_string());
    let h = ph.multi_cell_h(&efct, card_inner_w, prop_h);
    *y -= h;
    ph.set_xy(base_x, *y);
    ph.multi_cell(&efct, card_inner_w, prop_h);
    
    if prop.attr.len() > 0 {
        ph.set_font_i("Helvetica", font_size);
        
        let mut string = String::with_capacity(default_attr_string_alloc);
        for attr in &prop.attr {
            add_attr_to_string(attr, &mut string);
            string.push_str(", ");
        }
        string.pop();
        string.pop();
        let string = process_commands(string);
        
        let h = ph.multi_cell_h(&string, card_inner_w, prop_h);
        *y -= h;
        ph.set_xy(base_x, *y);
        ph.multi_cell(&string, card_inner_w, prop_h);
        
        ph.set_font("Helvetica", font_size);
    }

    *y -= prop_pad;
}

fn deserialize_card(ph: &PdfHandler, card: &Card, base_x: f64, base_y: f64) {
    //name
    let mut y = base_y;
    ph.set_xy(base_x, y);
    ph.set_font("Helvetica", name_font_size);
    ph.center_cell(&card.name, card_inner_w, name_h);

    //main attributes
    ph.set_font("Helvetica", main_attr_font_size);
    y += name_h;
    let main_attr_icon_data = get_main_attr_icon_data(card);
    let step_w = card_inner_w / main_attr_icon_data.len() as f64;
    let a = ph.string_w(&main_attr_icon_data[main_attr_icon_data.len() - 1].1);
    let w = step_w * (main_attr_icon_data.len() - 1) as f64 + main_attr_icon_w + main_attr_text_pad_l + a;
    let margin = (card_inner_w - w) / 2.0;
    for (i, (icon, val)) in main_attr_icon_data.iter().enumerate() {
        let x = base_x + margin + i as f64 * step_w;
        ph.set_xy(x, y);
        ph.image(icon, main_attr_icon_w, main_attr_h);
        ph.text(&val, x + main_attr_icon_w + main_attr_text_pad_l, y + main_attr_text_pad_t);
    }

    //other attributes
    let mut other_attr = String::with_capacity(default_attr_string_alloc);
    for attr in &card.attr {
        match &attr.n as &str {
            "Level" | "Tribute" | "Offense" | "Defense" | "Health" | "Lethality" | "Power" => continue,
            _ => {
                add_attr_to_string(attr, &mut other_attr);
                other_attr.push_str(", ");
            }
        }
    }
    other_attr.pop();
    other_attr.pop();
    y += main_attr_h + main_attr_pad_b;
    ph.set_xy(base_x, y);
    ph.set_font_i("Helvetica", font_size);
    ph.center_multi_cell(&process_commands(other_attr), card_inner_w, other_attr_h);
    
    //properties
    ph.set_font("Helvetica", font_size);
    y = base_y + card_inner_h;
    for prop in card.pass.iter().rev() {
        deserialize_prop(ph, prop, base_x, &mut y);
    }
    for prop in card.trig.iter().rev() {
        deserialize_prop(ph, prop, base_x, &mut y);
    }
    for prop in card.acti.iter().rev() {
        deserialize_prop(ph, prop, base_x, &mut y);
    }
}

fn main() -> Maybe {
    Python::with_gil(|py| {
        let ph = PdfHandler::new(py);
        let cards = std::fs::read_to_string("cards.json").unwrap();
        let cards: Vec<Card> = serde_json::from_str(&cards).unwrap();
        let num_cards = cards.len();
        for p in 0..if num_cards % cards_per_page == 0 { num_cards / cards_per_page } else { num_cards / cards_per_page + 1 } {
            ph.add_page();
            for r in 0..cards_per_column {
                for c in 0..cards_per_row {
                    let i = p * cards_per_page + r * cards_per_row + c;
                    if i < num_cards {
                        let x = page_pad_l as f64 + c as f64 * card_outer_w;
                        let y = page_pad_t as f64 + r as f64 * card_outer_h;

                        let name = &format!("{}.png", &cards[i].name);
                        if ph.has_image(name) {
                            ph.set_xy(x, y);
                            ph.image(name, card_outer_w, card_outer_h);
                        }
                        else {
                            ph.rect(x, y, card_outer_w, card_outer_h);
                        }

                        let base_x = x + card_pad;
                        let base_y = y;
                        deserialize_card(&ph, &cards[i], base_x, base_y)
                    }
                    else {
                        return;
                    }
                }
            }
        }
        
        ph.output();
    });
    
    ok
}