#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use png::*;
use extstd::*;
use std::fs::*;
use std::io::*;
use std::path::*;
use card_game::*;

const pixels_per_mm: f64 = 10.0;
const gradient_height: usize = (gradient_h * pixels_per_mm as f64) as usize;
const width: usize = (pixels_per_mm * card_outer_w) as usize;
const max_alpha: u8 = 200;
const row_to_alpha_exponent: f64 = 1.16;
const row_to_alpha_factor: f64 = 4.0;

fn make_encoder<'a>(height: usize, buf_writer: &'a mut BufWriter<File>) -> Encoder<'a, &'a mut BufWriter<File>> {
    let mut encoder = Encoder::new(buf_writer, width as u32, height as u32); // Width is 2 pixels and height is 1.
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
    encoder
}

fn make_writer(file_name: &String) -> BufWriter<File> {
    let path = Path::new(&file_name);
    let file = File::create(path).unwrap();
    let buf_writer = BufWriter::new(file);
    buf_writer
}

fn get_alpha(row: usize) -> u8 {
    let row = row as f64;
    let reduction = row_to_alpha_factor * row.powf(row_to_alpha_exponent);
    let alpha = max_alpha as f64 - reduction;
    let alpha = f64::max(alpha, 0.0);
    alpha as u8
}

fn main() {
    for other_attr_lines in 0..10 {
        let mut height = upper_alpha_base_h;
        if other_attr_lines > 0 {
            height += main_attr_pad_b;
            height += other_attr_h * other_attr_lines as f64;
        }
        let height = (pixels_per_mm * height) as usize;
        
        let file_name = format!("deserialize/alpha/upper{}.png", other_attr_lines);
        let mut buf_writer = make_writer(&file_name);
        let encoder = make_encoder(height + gradient_height, &mut buf_writer);
        let mut writer = encoder.write_header().unwrap();
    
        let mut data: Vec<u8> = Vec::with_capacity(width * (height + gradient_height) * 4 );
        data.resize(width * (height + gradient_height) * 4, 0);
        for r in 0..height {
            for c in 0..width {
                let p = r * width * 4 + c * 4;
                data[p] = 0u8;
                data[p + 1] = 0u8;
                data[p + 2] = 0u8;
                data[p + 3] = max_alpha;
            }
        }

        let offset = width * height * 4;
        for r in 0..gradient_height {
            for c in 0..width {
                let p = offset + r * width * 4 + c * 4;
                data[p] = 0u8;
                data[p + 1] = 0u8;
                data[p + 2] = 0u8;
                data[p + 3] = get_alpha(r);
            }
        }
    
        writer.write_image_data(&data[..]).unwrap();
    }
    println!("Upper alphas generated");

    for prop_lines in 0..25 {
        let height = prop_lines as f64 * prop_h + card_pad;
        let height = (pixels_per_mm * height) as usize;
        
        let file_name = format!("deserialize/alpha/lower{}.png", prop_lines);
        let mut buf_writer = make_writer(&file_name);
        let encoder = make_encoder(height + gradient_height, &mut buf_writer);
        let mut writer = encoder.write_header().unwrap();
    
        let mut data: Vec<u8> = Vec::with_capacity(width * (height + gradient_height) * 4 );
        data.resize(width * (height + gradient_height) * 4, 0);
        let offset = width * gradient_height * 4;
        for r in 0..height {
            for c in 0..width {
                let p = offset + r * width * 4 + c * 4;
                data[p] = 0u8;
                data[p + 1] = 0u8;
                data[p + 2] = 0u8;
                data[p + 3] = max_alpha;
            }
        }

        for r in 0..gradient_height {
            for c in 0..width {
                let p = (gradient_height - r - 1) * width * 4 + c * 4;
                data[p] = 0u8;
                data[p + 1] = 0u8;
                data[p + 2] = 0u8;
                data[p + 3] = get_alpha(r);
            }
        }
    
        writer.write_image_data(&data[..]).unwrap();
    }

    println!("Lower alphas generated");
}