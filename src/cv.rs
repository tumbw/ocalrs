use crate::CONFIG;
use image::{ImageBuffer, Rgba};

// need struct ?

fn is_pixel_start(pixel: &Rgba<u8>) -> bool {
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];

    r == CONFIG.start_pixel_red && g == CONFIG.start_pixel_green && b == CONFIG.start_pixel_blue
}


fn what_color(
    pixel: &[u8],
    thr_r_max: u8,
    thr_r_min: u8,
    thr_g_max: u8,
    thr_g_min: u8,
    thr_b_max: u8,
    thr_b_min: u8
) -> bool {
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];

    thr_r_max >= r && r >= thr_r_min &&
    thr_g_max >= g && g >= thr_g_min &&
    thr_b_max >= b && b >= thr_b_min
}


fn is_white(pixel: &[u8], thr: u8) -> bool {
    let r = pixel[0];
    let g = pixel[1];
    let b = pixel[2];

    r >= thr && g >= thr && b >= thr
}


pub fn is_start(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> bool {
    let pixel = img.get_pixel(CONFIG.start_pixel_x, CONFIG.start_pixel_y);
    !is_pixel_start(pixel)
}


pub fn get_beginning_of_word(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Option<(u32, u32)> {
    let letter_w = CONFIG.char_width;
    let letter_h = CONFIG.char_height;

    let min_x = CONFIG.roi_x;
    let min_y = CONFIG.roy_y;
    let max_x = min_x + CONFIG.roi_width - letter_w;
    let max_y = min_y + CONFIG.roi_height - letter_h;

    let scan_stride = CONFIG.scan_stride;

    let mut y = min_y;
    while y <= max_y {
        let mut x = min_x;
        while x <= max_x {
            match detect_word_start(img, x, y) {
                Some((found_x, found_y)) => return Some((found_x, found_y)),
                None => x += scan_stride,
            }
        }
        y += scan_stride;
    }
    None
}


fn detect_word_start(img: &ImageBuffer<Rgba<u8>, Vec<u8>>, start_x: u32, start_y: u32) -> Option<(u32, u32)> {
    let mut ans_x = 0;
    let mut ans_y = 0;

    let raw_img = img.as_raw();

    let width = img.width() as usize;

    let end_x = (start_x + CONFIG.char_width) as usize;
    let end_y = (start_y + CONFIG.char_height) as usize;
    let mut color_count: u32 = 0;

    for y in start_y as usize..end_y {
        let row_start = (y * width + start_x as usize) * 4;
        let row_end = (y * width + end_x) * 4;

        for (i, chunk) in raw_img[row_start..row_end].chunks_exact(4).enumerate() {
            if what_color(
                chunk,
                CONFIG.start_char_r_max,
                CONFIG.start_char_r_min,
                CONFIG.start_char_g_max,
                CONFIG.start_char_g_min,
                CONFIG.start_char_b_max,
                CONFIG.start_char_b_min
            ) {
                color_count += 1;

                if ans_x == 0 {
                    ans_x = start_x + (i as u32);
                    ans_y = y as u32;
                }

                if color_count >= CONFIG.required_pixels_for_start {
                    return Some((ans_x, ans_y));
                }
            }
        }
    }
    None
}


pub fn get_word_bounds (x: u32, y: u32, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> (u32, u32, u32, u32) {
    let approx_x = x.saturating_sub(15);
    let approx_y = y.saturating_sub(7);

    let (img_w, img_h) = img.dimensions();
    let height = 43.min(img_h.saturating_sub(approx_y));

    let max_scan_x = img_w.saturating_sub(CONFIG.scan_stride);

    let width_cut = calc_width(approx_x, approx_y, max_scan_x, height, img);

    (approx_x, approx_y, width_cut, height)
}


fn calc_width(start_x: u32, start_y: u32, max_x: u32, height: u32, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> u32 {
    let mut allowed_false_strip = 0;

    let mut scan_x = start_x;
    while scan_x <= max_x {
        if is_char_square(scan_x, start_y, height, img) {
            allowed_false_strip = 0;
        } else {
            allowed_false_strip += 1;
            if allowed_false_strip >= 3 {
                let width = scan_x.saturating_sub(start_x);
                return width;
            }
        }
        scan_x += CONFIG.scan_stride;
    }

    max_x
}


fn is_char_square(x: u32, start_y: u32, height: u32, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> bool {
    let (width, _) = img.dimensions();
    let raw_img = img.as_raw();

    let mut count = 0;

    for y in start_y..(start_y + height) {
        let row_start = (y * width + x) as usize * 4;
        let end_x = (x + CONFIG.scan_stride).min(width);
        let row_end = (y * width + end_x) as usize * 4;

        for chunk in raw_img[row_start..row_end].chunks_exact(4) {
            if is_white(chunk, CONFIG.char_white) ||
                what_color(
                chunk,
                CONFIG.start_char_r_max,
                CONFIG.start_char_r_min,
                CONFIG.start_char_g_max,
                CONFIG.start_char_g_min,
                CONFIG.start_char_b_max,
                CONFIG.start_char_b_min
            ) {
                count += 1;

                if count >= CONFIG.required_pixels_for_borders {
                    return true;
                }
            }
        }
    }
    false
}


pub fn make_bin(x: u32, y: u32, width: u32, height: u32, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Option<ImageBuffer<Rgba<u8>, Vec<u8>>> {

    let origin_width = img.width();
    let raw_img = img.as_raw();
    let mut binary_data: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

    for y in y..(y + height) {
        let row_start = (y * origin_width + x) as usize * 4;
        let row_end = (y * origin_width + x + width) as usize * 4;

        for chunk in raw_img[row_start..row_end].chunks_exact(4) {
            let is_pass_pixel = what_color(
                chunk,
                CONFIG.bin_start_char_r_max,
                CONFIG.bin_start_char_r_min,
                CONFIG.bin_start_char_g_max,
                CONFIG.bin_start_char_g_min,
                CONFIG.bin_start_char_b_max,
                CONFIG.bin_start_char_b_min
            );

            let is_white_pixel = is_white(chunk, CONFIG.bin_char_white);

            match is_pass_pixel || is_white_pixel {
                true => binary_data.extend_from_slice(&[0, 0, 0, 255]),
                false  => binary_data.extend_from_slice(&[255, 255, 255, 255]),
            }
        }
    }


    ImageBuffer::from_raw(width, height, binary_data)
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn testcl_get_word_start() {
        let img = image::open(format!("debug\\f1.png")).unwrap();
        let rgba_img = img.to_rgba8();
        let (x, y) = get_beginning_of_word(&rgba_img).unwrap();

        println!("{} - {}", x, y);

        let bounds = get_word_bounds(x, y, &rgba_img);

        let gray = make_bin(bounds.0, bounds.1, bounds.2, bounds.3, &rgba_img).unwrap();

        gray.save(format!("img\\gray.png")).ok();
        println!("save gray");
    }
}