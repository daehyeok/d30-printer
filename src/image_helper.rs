/// This module provides functionality to create image data for the Phomemo D30 label maker.
/// The original code for this file is derived from [crabdancing/phomemo-d30](https://github.com/crabdancing/phomemo-d30).
use std::path::Path;

use crate::config::Config;
use anyhow::{Context, Result, anyhow};
use image::{self, DynamicImage, ImageBuffer, Rgb};
use log::trace;
use rusttype::{Font, Scale};

use dimensions::*;

pub const IMG_PRECURSOR: &[u8] = &[31, 17, 36, 0, 27, 64, 29, 118, 48, 0, 12, 0, 64, 1]; // 1f1124001b401d7630000c004001

const COLOR_BLACK: image::Rgb<u8> = Rgb([255u8, 255u8, 255u8]);

pub fn generate_image(config: &Config) -> Result<DynamicImage> {
    let label_dimensions = Dimensions::new(320, 96);
    trace!("{:#?}", &label_dimensions);

    let font = load_font(&config.font)?;
    let font = Font::try_from_vec(font).context("Could not init font.")?;
    let text = &config.text;

    //TODO - calcuate scale.
    let default_margins = 15.;
    let default_minus = 1.;
    let scale = {
        // let scale = 100.0;
        let actual_size: Dimensions =
            imageproc::drawing::text_size(Scale::uniform(100.0), &font, text).into();
        let scale_by_x = (label_dimensions.x - 2.0 * default_margins) / actual_size.x;
        let scale_by_y = (label_dimensions.y - 2.0 * default_margins) / actual_size.y;
        100.0
            * if scale_by_y > scale_by_x {
                scale_by_x
            } else {
                scale_by_y
            }
            - default_minus
    };

    let actual_size: Dimensions =
        imageproc::drawing::text_size(Scale::uniform(scale), &font, text).into();
    let txt_pos = (actual_size - label_dimensions) / -2.;

    let mut canvas: ImageBuffer<Rgb<u8>, _> = ImageBuffer::new(
        label_dimensions.width() as u32,
        label_dimensions.height() as u32,
    );

    imageproc::drawing::draw_text_mut(
        &mut canvas,
        COLOR_BLACK,
        txt_pos.x as i32,
        txt_pos.y as i32,
        Scale::uniform(scale),
        &font,
        text,
    );

    let canvas = DynamicImage::from(canvas).rotate270();

    Ok(canvas)
}

pub fn pack_image(image: &DynamicImage) -> Vec<u8> {
    // This section of code is heavily based on logic from polskafan's phomemo_d30 code on Github
    // See here: https://github.com/polskafan/phomemo_d30
    let threshold: u8 = 127;
    let width = image.width() as usize;
    let height = image.height() as usize;

    let mut bit_grid = vec![vec![0u8; width]; height];

    let image = image.to_rgb8();

    let mut output = Vec::new();
    for (x, y, pixel) in image.enumerate_pixels() {
        let (x, y) = (x as usize, y as usize);

        if pixel[0] > threshold {
            bit_grid[y][x] = 1;
        } else {
            bit_grid[y][x] = 0;
        }
    }

    for bit_row in bit_grid {
        for byte_num in 0..(image.width() / 8) {
            let mut byte: u8 = 0;
            for bit_offset in 0..8 {
                let pixel: u8 = bit_row[(byte_num * 8 + bit_offset) as usize];
                // Raw bit manipulation iterates through 0 through 7, and bitshifts the micro-pixels onto a byte 'sandwich',
                // before it gets shipped off to the D30 printer
                byte |= (pixel & 0x01) << (7 - bit_offset);
                // For instance, instead of storing, e.g. 00000001 00000000 00000000 00000001 00000000 00000000 00000000 00000000
                // We can instead send: 10010000
                // This considerably cuts down on the number of bytes needed to send an image,
                // but of course only works if we don't need to send any gradations of pixel color or intensity.
            }
            output.push(byte);
        }
    }
    output
}

fn read_font(font_name: &str) -> Result<Vec<u8>> {
    let system_font = findfont::find(font_name);
    if system_font.is_some() {
        let res = std::fs::read(system_font.unwrap())?;
        return Ok(res);
    }

    info!("Failed to find the font in the system");
    info!("Try to read file: {}", font_name);

    let path = Path::new(font_name);
    if !path.exists() {
        return Err(anyhow!("Font does not exist: {:?}.", font_name));
    }

    let res = std::fs::read(system_font.unwrap())?;
    Ok(res)
}

fn load_font(font_name: &Option<String>) -> Result<Vec<u8>> {
    match font_name {
        Some(name) => read_font(name),
        None => Ok(include_bytes!("Hack.ttf").to_vec()),
    }
}
