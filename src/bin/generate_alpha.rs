#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use png::*;
use extrust::*;
use std::fs::*;
use std::io::*;
use std::path::*;
use card_game::*;

const pixels_per_mm: usize = 5;
const width: usize = pixels_per_mm * card_outer_w as usize;
const max_alpha: u8 = 128;

fn main() {
    for other_attr_lines in 0..10 {
        let mut height = card_upper_alpha_h_short;
        if other_attr_lines > 0 {
            height += main_attr_pad_b;
            height += other_attr_h * other_attr_lines as f64;
        }
        let height = pixels_per_mm * height as usize;
        
        let file_name = format!("deserialize/images/upper{}.png", other_attr_lines);
        let path = Path::new(&file_name);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        cout(height);

        let mut encoder = Encoder::new(w, width as u32, height as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_source_gamma(ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = SourceChromaticities::new(     // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000)
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();
    
        let mut data: Vec<u8> = Vec::with_capacity(width * height * 4);
        data.resize(width * height * 4, 0);
        for r in 0..height {
            for c in 0..width {
                let p = r * width * 4 + c * 4;
                data[p] = 0u8;
                data[p + 1] = 0u8;
                data[p + 2] = 0u8;
                data[p + 3] = if r as u8 > max_alpha { 0 } else { max_alpha - r as u8 };
            }
        }
    
        writer.write_image_data(&data[..]).unwrap();
    }

    println!("Alphas generated");
}