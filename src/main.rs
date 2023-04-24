use std::slice;

use image::{Rgba, GenericImageView};
use rayon::prelude::*;

fn main() {
    //println!("{:x}", pix(0x00, 0x02, 0xFF));
    let pic = image::open("pic.jpg").unwrap();
    let w = pic.width() as usize;
    let h = pic.height() as usize;

    let mut out_buf: Vec<u32> = vec![0u32; w * h];

    for (x, y, color) in pic.pixels() {
        let value = (color.0[0] as u16 + color.0[1] as u16 + color.0[2] as u16) * 85;
        out_buf[(y * w as u32 + x) as usize] = get_color(value);
    }

    image::save_buffer("image.png", convert(&out_buf[..]), w as u32, h as u32, image::ColorType::Rgba8).unwrap()
}

pub fn convert<'a>(data: &'a[u32]) -> &'a[u8] {
    unsafe { &mut slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4) }
}

fn get_color_slow(value: u16) -> u32 {
    let val_perc = (value - u16::MIN) as f64 / (u16::MAX - u16::MIN) as f64;
    let color_perc = 1.0 / (MAP_COLORS.len() - 1) as f64;
    let block_of_color = val_perc / color_perc;
    let block_idx = block_of_color.trunc() as usize;
    let val_perc_resudual = val_perc - (block_of_color.trunc() * color_perc);
    let perc_of_color = val_perc_resudual / color_perc;

    let target = MAP_COLORS[block_idx];
    let next = if value == u16::MAX {MAP_COLORS[block_idx]} else {MAP_COLORS[block_idx + 1]};

    let delta_r = red(next) as i32 - red(target) as i32;
    let delta_g = green(next) as i32 - green(target) as i32;
    let delta_b = blue(next) as i32 - blue(target) as i32;

    let r = red(target) + (delta_r as f64 * perc_of_color) as u32;
    let g = green(target) + (delta_g as f64 * perc_of_color) as u32;
    let b = blue(target) + (delta_b as f64 * perc_of_color) as u32;


    return pix(r, g, b);
}

fn get_color(value: u16) -> u32 {
    let val_perc = value as i32; // scale 0-u16
    let color_perc = U16_MAX / (MAP_COLORS.len() - 1) as i32; // scale 0-u16

    let block_of_color = (val_perc  / color_perc) * U16_MAX; // scale 0-u16

    let block_idx = (block_of_color / U16_MAX) as usize; // correct
    let val_perc_resudual = val_perc - (block_idx as i32 * color_perc); // scale 0-u16
    let perc_of_color = val_perc_resudual * U16_MAX / color_perc; // scale 0-u16

    let target = MAP_COLORS[block_idx];
    let next = if block_idx == MAP_COLORS_LENGTH - 1 {MAP_COLORS[block_idx]} else {MAP_COLORS[block_idx + 1]};

    let delta_r = red(next) as i32 - red(target) as i32;
    let delta_g = green(next) as i32 - green(target) as i32;
    let delta_b = blue(next) as i32 - blue(target) as i32;

    let r = red(target) + (delta_r * perc_of_color / U16_MAX) as u32;
    let g = green(target) + (delta_g * perc_of_color / U16_MAX) as u32;
    let b = blue(target) + (delta_b * perc_of_color / U16_MAX) as u32;


    return pix(r, g, b);
}

#[inline(always)]
fn red(pix: u32) -> u32 {
    (pix & 0xFF000000) >> 24
}

#[inline(always)]
fn green(pix: u32) -> u32 {
    (pix & 0x00FF0000) >> 16
}

#[inline(always)]
fn blue(pix: u32) -> u32 {
    (pix & 0x0000FF00) >> 8
}

#[inline(always)]
fn pix(r: u32, g: u32, b: u32) -> u32 {
    (0xFF << 24) | (b << 16) | (g << 8) | r
}

const MAP_COLORS_LENGTH: usize = MAP_COLORS_LENGTH_u16 as usize;
const MAP_COLORS_LENGTH_u32: u32 = MAP_COLORS_LENGTH_u16 as u32;
const MAP_COLORS_LENGTH_i32: i32 = MAP_COLORS_LENGTH_u16 as i32;
const MAP_COLORS_LENGTH_u16: u16 = 7;
const U16_MAX: i32 = u16::MAX as i32;

const MAP_COLORS: [u32; MAP_COLORS_LENGTH] = [
    0x000000FF, // Black
    0x0000FFFF, // Blue,
    0x00FFFFFF, // Cyan,
    0x00FF00FF, // Green,
    0xFFFF00FF, // Yellow,
    0xFF0000FF, // Red,
    0xFFFFFFFF, // White
];