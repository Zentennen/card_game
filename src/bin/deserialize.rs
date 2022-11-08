#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

//todo
//better layout for main attribs
//resize main attribs
//card text size
//property layout

use std::collections::HashMap;
use extrust::*;
use card_game::*;
use genpdf::*;
use genpdf::elements::*;
use genpdf::fonts::*;
use genpdf::render::Area;
use genpdf::style::Style;
use genpdf::style::StyledString;

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

struct LayeredLayout {
    elems: Vec<Box<dyn Element>>
}

impl LayeredLayout {
    fn new() -> Self {
        Self { elems: Vec::with_capacity(2) }
    }

    fn push(&mut self, elem: impl Element + 'static) {
        self.elems.push(Box::new(elem));
    }
}

impl Element for LayeredLayout {
    fn render(&mut self, context: &Context, area: Area<'_>, style: style::Style) -> Result<RenderResult, error::Error> {
        let mut result = RenderResult::default();
        for elem in self.elems.iter_mut() {
            let res = elem.render(context, area.clone(), style)?;
            result.size.width = Mm::max(result.size.width, res.size.width);
            result.size.height = Mm::max(result.size.height, res.size.height);
        }
        Ok(result)
    }
}

struct HorizontalLayout {
    elements: Vec<Box<dyn Element>>,
    render_idx: usize,
}

fn stack_horizontal(a: Size, b: Size) -> Size {
    let mut size = a;
    size.width = a.width + b.width;
    size.height = a.height.max(b.height);
    size
}

impl HorizontalLayout {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
            render_idx: 0,
        }
    }

    fn push<E: Element + 'static>(&mut self, element: E) {
        self.elements.push(Box::new(element));
    }

    fn element<E: Element + 'static>(mut self, element: E) -> Self {
        self.push(element);
        self
    }
}

impl Element for HorizontalLayout {
    fn render(
        &mut self,
        context: &Context,
        mut area: render::Area<'_>,
        style: Style,
    ) -> Result<RenderResult, error::Error> {
        let mut result = RenderResult::default();
        while area.size().width > From::from(0.0f64) && self.render_idx < self.elements.len() {
            let element_result = self.elements[self.render_idx].render(context, area.clone(), style)?;
            area.add_offset(Position::new(element_result.size.width, 0i32));
            result.size = stack_horizontal(result.size, element_result.size);
            if element_result.has_more {
                result.has_more = true;
                return Ok(result);
            }
            self.render_idx += 1;
        }
        result.has_more = self.render_idx < self.elements.len();
        Ok(result)
    }
}

struct Resources {
    images: HashMap<String, Image>,
    fonts: HashMap<String, FontFamily<Font>>,
    styles: HashMap<String, Style>
}

impl Resources {
    fn new(doc: &mut Document) -> Self {
        let mut r = Self { images: HashMap::with_capacity(100), fonts: HashMap::with_capacity(100), styles: HashMap::with_capacity(100) };

        //Load images
        let entries = std::fs::read_dir("deserialize/images").expect("Could not find directory deserialize/images");
        for entry in entries {
            if let Result::Ok(entry) = entry {
                let name = entry.file_name().to_str().unwrap().to_string();
                r.images.insert(name, Image::from_path(entry.path()).unwrap());
            }
        }

        //Load fonts
        let entries = std::fs::read_dir("deserialize/fonts").expect("Could not find directory deserialize/fonts");
        for entry in entries {
            if let Result::Ok(entry) = entry {
                let name = entry.file_name().to_str().unwrap().to_string();
                let path = entry.path().as_path().to_str().unwrap().to_string();
                let font = doc.add_font_family(fonts::from_files(path, &name, None).unwrap());
                r.fonts.insert(name, font);
            }
        }

        for font in &r.fonts {
            let mut name = String::with_capacity(100);
            name.push_str(font.0);
            name.push_str("Bold");
            let style = Style::new().with_font_family(*font.1).bold();
            r.styles.insert(name, style);

            let mut name = String::with_capacity(100);
            name.push_str(font.0);
            name.push_str("Italic");
            let style = Style::new().with_font_family(*font.1).italic();
            r.styles.insert(name, style);
        }

        r
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

fn create_main_attr_par(val: f64, st: Style, margins: Margins) -> impl Element {
    Paragraph::new(style::StyledString::new(val.to_string(), st))
        .aligned(Alignment::Left)
        .padded(margins)
}

fn create_main_attr_layout(card: &Card, resources: &Resources) -> Vec<HorizontalLayout> {
    let mut attribs: Vec<HorizontalLayout> = Vec::with_capacity(num_main_attribs);
    let attrib_text_margins = Margins::trbl(attrib_text_padding_t, attrib_text_padding_r, 0.0, attrib_text_padding_l);
    let st = Style::new().with_font_size(main_attrib_font_size);

    if let Some(val) = get_attribute_value(&card.attr, "Offense") {
        if val != 0.0 {
            let h = HorizontalLayout::new()
                .element(resources.images["sword.png"].clone().with_scale((0.20, 0.175)).padded(Margins::trbl(0.38, 0.0, 0.0, 0.0)))
                .element(create_main_attr_par(val, st, attrib_text_margins));
            attribs.push(h);
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Defense") {
        if val != 0.0 {
            let h = HorizontalLayout::new()
                .element(resources.images["shield.png"].clone().with_scale((0.35, 0.31)).padded(Margins::trbl(0.0, 0.0, 0.0, 0.0)))
                .element(create_main_attr_par(val, st, attrib_text_margins));
            attribs.push(h);
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Health") {
        if val != 1.0 {
            let h = HorizontalLayout::new()
                .element(resources.images["heart.jpg"].clone().with_scale((0.11, 0.11)).padded(Margins::trbl(0.22, 0.0, 0.0, 0.0)))
                .element(create_main_attr_par(val, st, attrib_text_margins));
            attribs.push(h);
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Lethality") {
        if val != 1.0 {
            let h = HorizontalLayout::new()
                .element(resources.images["fist.png"].clone().with_scale((0.28, 0.24)).padded(Margins::trbl(0.0, 0.0, 0.0, 0.0)))
                .element(create_main_attr_par(val, st, attrib_text_margins));
            attribs.push(h);
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Power") {
        if val != 0.0 {
            let h = HorizontalLayout::new()
                .element(resources.images["star.png"].clone().with_scale((0.32, 0.32)).padded(Margins::trbl(0.03, 0.0, 0.0, 0.0)))
                .element(create_main_attr_par(val, st, attrib_text_margins));
            attribs.push(h);
        }
    }

    if let Some(val) = get_attribute_value(&card.attr, "Tribute") {
        if val != 0.0 {
            let h = HorizontalLayout::new()
                .element(resources.images["drop.png"].clone().with_scale((0.21, 0.16)).padded(Margins::trbl(0.18, 0.0, 0.0, 0.0)))
                .element(create_main_attr_par(val, st, attrib_text_margins));
            attribs.push(h);
        }
    }

    attribs
}

fn add_attr_to_layout(card: &Card, resources: &Resources, layout: &mut LinearLayout) {
    let attrib_elems = create_main_attr_layout(card, resources);
    let main_attr_count = attrib_elems.len();
    let mut main_attrib_layout = TableLayout::new(vec![1; main_attr_count]);
    let mut row = main_attrib_layout.row();

    for e in attrib_elems {
        row.push_element(e);
    }
    row.push().unwrap();

    layout.push(main_attrib_layout.padded(Margins::trbl(attrib_padding_t, 0.0, attrib_padding_b, attrib_padding_l[main_attr_count])));

    let mut string = String::with_capacity(default_attr_string_alloc);
    for attr in &card.attr {
        match &attr.n[..] {
            "Level" | "Tribute" | "Offense" | "Defense" | "Health" | "Lethality" | "Power" => continue,
            _ => {
                add_attr_to_string(attr, &mut string);
                string.push_str(", ");
            }
        }
    }
    if !string.is_empty() {
        string.pop();
        string.pop();
    }
    let string = process_commands(string);
    layout.push(Paragraph::new(StyledString::new(string, Style::new().italic())).aligned(Alignment::Center));
    layout.push(Break::new(0.9));
}

fn prop_attr_to_par(prop: &Property, resources: &Resources) -> Paragraph {
    let mut string = String::with_capacity(default_attr_string_alloc);
    for attr in &prop.attr {
        add_attr_to_string(attr, &mut string);
        string.push_str(", ");
    }
    string.pop();
    string.pop();

    Paragraph::new(StyledString::new(process_commands(string), resources.styles["HelveticaItalic"]))
}

fn card_to_pdf_text(card: &Card, resources: &Resources) -> PaddedElement<LinearLayout> {
    let name_par = Paragraph::new(style::StyledString::new(&card.name, style::Style::new().with_font_size(name_font_size).bold())).aligned(Alignment::Center);
    let mut linear_layout = LinearLayout::vertical().element(name_par);
    
    add_attr_to_layout(card, resources, &mut linear_layout);

    for prop in &card.acti {
        if !prop.attr.is_empty() {
            linear_layout.push(prop_attr_to_par(prop, resources));
        }
        linear_layout.push(Paragraph::new(process_commands(prop.efct.clone())));
        linear_layout.push(Break::new(0.75));
    }
    for prop in &card.trig {
        if !prop.attr.is_empty() {
            linear_layout.push(prop_attr_to_par(prop, resources));
        }
        linear_layout.push(Paragraph::new(process_commands(prop.efct.clone())));
        linear_layout.push(Break::new(0.75));
    }
    for prop in &card.pass {
        if !prop.attr.is_empty() {
            linear_layout.push(prop_attr_to_par(prop, resources));
        }
        linear_layout.push(Paragraph::new(process_commands(prop.efct.clone())));
        linear_layout.push(Break::new(0.75));
    }

    linear_layout.padded(Margins::trbl(card_padding_t, card_padding, card_padding, card_padding))
}

fn main() -> Maybe {
    let font_family = fonts::from_files("deserialize/fonts/Helvetica", "Helvetica", None)?;
    let mut doc = Document::new(font_family);
    doc.set_font_size(font_size);
    doc.set_line_spacing(line_spacing);
    
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(Margins::trbl(page_padding, page_padding_r, page_padding, page_padding_l));
    doc.set_page_decorator(decorator);
    
    deserialize_all_cards(&mut doc)?;
    //test_deserialize(&mut doc)?;

    cout("Writing to pdf");
    doc.render_to_file("cards.pdf")?;
    println!("Deserialized successfully");
    ok
}

fn deserialize_all_cards(doc: &mut Document) -> Result<usize, Er> {
    let cards = std::fs::read_to_string("cards.json")?;
    let cards: Vec<Card> = serde_json::from_str(&cards)?;
    let num_cards = cards.len();
    let resources = Resources::new(doc);
    for p in 0..if num_cards % cards_per_page == 0 { num_cards / cards_per_page } else { num_cards / cards_per_page + 1 } {
        if p != 0 {
            doc.push(PageBreak::new());
        }
        let mut table = TableLayout::new(vec![1; cards_per_row]);
        table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
        for r in 0..cards_per_column {
            let mut row = table.row();
            for c in 0..cards_per_row {
                let i = p * cards_per_page + r * cards_per_row + c;
                if i < num_cards {
                    let mut layered = LayeredLayout::new();
                    layered.push(resources.images["white.png"].clone().with_scale((card_w, card_h)));
                    layered.push(card_to_pdf_text(&cards[i], &resources));
                    row.push_element(layered);
                }
                else {
                    let image = resources.images["white.png"].clone().with_scale((card_w, card_h));
                    row.push_element(image);
                }
            }
            row.push().expect("EXPECT ERR: Could not push table row");
        }
        doc.push(table);
    }
    Ok(num_cards)
}
