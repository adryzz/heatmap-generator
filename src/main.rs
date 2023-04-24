use std::{io::stdin, slice};

use image::{GenericImageView, Rgba};
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

    image::save_buffer(
        "image.png",
        convert(&out_buf[..]),
        w as u32,
        h as u32,
        image::ColorType::Rgba8,
    )
    .unwrap();
}

pub fn convert<'a>(data: &'a [u32]) -> &'a [u8] {
    unsafe { &mut slice::from_raw_parts(data.as_ptr() as *const u8, data.len() * 4) }
}

pub fn try_convert<'a>(data: &'a [u8]) -> Option<&'a [u32]> {
    if data.len() % 4 != 0 {
        return None;
    }

    Some(unsafe { &mut slice::from_raw_parts(data.as_ptr() as *const u32, data.len() / 4) })
}

fn get_color(value: u16) -> u32 {
    let blocks = MAP_COLORS.len() - 1;
    let block_of_color = value as u32 * blocks as u32;
    let block_idx = (block_of_color >> u16::BITS) as usize;
    let perc_of_color = block_of_color as u16;

    let target = MAP_COLORS[block_idx];
    let next = MAP_COLORS[block_idx + 1];

    let delta_r = red(next) as i32 - red(target) as i32;
    let delta_g = green(next) as i32 - green(target) as i32;
    let delta_b = blue(next) as i32 - blue(target) as i32;

    let r = red(target) + ((delta_r * perc_of_color as i32) >> u16::BITS) as u32;
    let g = green(target) + ((delta_g * perc_of_color as i32) >> u16::BITS) as u32;
    let b = blue(target) + ((delta_b * perc_of_color as i32) >> u16::BITS) as u32;

    return pix(r, g, b);
}

fn get_color_block(values: [u16; 8]) -> [u32; 8] {
    let mut ret = [0u32; 8];

    for (i, value) in values.iter().enumerate() {
        let blocks = MAP_COLORS.len() - 1;
        let block_of_color = *value as u32 * blocks as u32;
        let block_idx = (block_of_color >> u16::BITS) as usize;
        let perc_of_color = block_of_color as u16;

        let target = MAP_COLORS[block_idx];
        let next = MAP_COLORS[block_idx + 1];

        let delta_r = red(next) as i32 - red(target) as i32;
        let delta_g = green(next) as i32 - green(target) as i32;
        let delta_b = blue(next) as i32 - blue(target) as i32;

        let r = red(target) + ((delta_r * perc_of_color as i32) >> u16::BITS) as u32;
        let g = green(target) + ((delta_g * perc_of_color as i32) >> u16::BITS) as u32;
        let b = blue(target) + ((delta_b * perc_of_color as i32) >> u16::BITS) as u32;

        ret[i] = pix(r, g, b);
    }

    ret
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

const MAP_COLORS_LENGTH: usize = 7;
const MAP_COLORS_LENGTH_I32: i32 = MAP_COLORS_LENGTH as i32;
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
