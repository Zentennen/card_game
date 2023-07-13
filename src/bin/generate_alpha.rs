#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use png::*;
use extstd::*;
use std::fs::*;
use std::io::*;
use std::path::*;
use card_game::*;

const pixel_width: usize = (pixels_per_mm as f64 * card_outer_width) as usize;

fn make_encoder<'a>(pixel_height: usize, buf_writer: &'a mut BufWriter<File>) -> Encoder<'a, &'a mut BufWriter<File>> {
    let mut encoder = Encoder::new(buf_writer, pixel_width as u32, pixel_height as u32);
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

fn get_gradient_alpha(row: usize) -> u8 {
    let reduction = row / pixels_per_alpha_step;
    let reduction = reduction.clamp(u8::MIN as usize, u8::MAX as usize) as u8;
    max_alpha - reduction
}

fn main() {
    print("Generating upper alphas...");
    par_for(0..5, generate_upper_alpha);
    print("Generating lower alphas...");
    par_for(0..30, generate_lower_alpha);
    print("All alphas generated.");
}

fn generate_lower_alpha(prop_lines: i32) {
    let height = prop_lines as f64 * prop_half_height + card_pad;
    let height = (pixels_per_mm * height) as usize;
        
    let file_name = format!("alpha/lower{}.png", prop_lines);
    let mut buf_writer = make_writer(&file_name);
    let encoder = make_encoder(height + alpha_gradient_pixel_height, &mut buf_writer);
    let mut writer = encoder.write_header().unwrap();
    
    let mut data: Vec<u8> = vec![0u8; pixel_width * (height + alpha_gradient_pixel_height) * 4];
    let offset = pixel_width * alpha_gradient_pixel_height * 4;
    for r in 0..height {
        for c in 0..pixel_width {
            let p = offset + r * pixel_width * 4 + c * 4;
            data[p + 3] = max_alpha;
        }
    }

    for r in 0..alpha_gradient_pixel_height {
        let a = get_gradient_alpha(r);
        for c in 0..pixel_width {
            let p = (alpha_gradient_pixel_height - r - 1) * pixel_width * 4 + c * 4;
            data[p + 3] = a;
        }
    }
    
    writer.write_image_data(&data[..]).unwrap();
}

fn generate_upper_alpha(other_attr_lines: usize) {
    let mut height = upper_alpha_base_height;
    if other_attr_lines > 0 {
        height += main_attr_pad_b;
        height += other_attr_height * other_attr_lines as f64;
    }
    let height = (pixels_per_mm * height) as usize;
        
    let file_name = format!("alpha/upper{}.png", other_attr_lines);
    let mut buf_writer = make_writer(&file_name);
    let encoder = make_encoder(height + alpha_gradient_pixel_height, &mut buf_writer);
    let mut writer = encoder.write_header().unwrap();
    
    let mut data: Vec<u8> = vec![0u8; pixel_width * (height + alpha_gradient_pixel_height) * 4];
    for r in 0..height {
        for c in 0..pixel_width {
            let p = r * pixel_width * 4 + c * 4;
            data[p + 3] = max_alpha;
        }
    }

    let offset = pixel_width * height * 4;
    for r in 0..alpha_gradient_pixel_height {
        let a = get_gradient_alpha(r);
        for c in 0..pixel_width {
            let p = offset + r * pixel_width * 4 + c * 4;
            data[p + 3] = a;
        }
    }
    
    writer.write_image_data(&data[..]).unwrap();
}