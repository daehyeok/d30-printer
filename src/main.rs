use crate::btl::D30;
use crate::config::Config;
use anyhow::Result;
use clap::Parser;
#[cfg(not(debug_assertions))]
use env_logger::fmt::{Color, Style, StyledValue};
#[cfg(not(debug_assertions))]
use log::{Level, LevelFilter};

mod btl;
mod config;
mod image_helper;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    pretty_env_logger::init();

    //Remove tracing info when run release mode.
    #[cfg(not(debug_assertions))]
    pretty_env_logger::formatted_builder()
        .filter(None, LevelFilter::Info)
        .format(|buf, record| {
            use std::io::Write;

            let mut style = buf.style();
            let level = colored_level(&mut style, record.level());
            writeln!(buf, "[{}] {}", level, record.args())
        })
        .init();

    let args = Config::parse();
    if let Err(e) = print(&args).await {
        error!("Failed to print: {:?}", e);
    }
}

#[cfg(not(debug_assertions))]
fn colored_level<'a>(style: &'a mut Style, level: Level) -> StyledValue<'a, &'a str> {
    match level {
        Level::Trace => style.set_color(Color::Magenta).value("Trace"),
        Level::Debug => style.set_color(Color::Blue).value("Debug"),
        Level::Info => style.set_color(Color::Green).value("Info"),
        Level::Warn => style.set_color(Color::Yellow).value("Warn"),
        Level::Error => style.set_color(Color::Red).value("Error"),
    }
}

async fn print(config: &Config) -> Result<()> {
    //TODO: add option to connect with addr.
    let d30 = D30::new(config).await?;

    let image = image_helper::generate_image(config)?;
    let mut output = image_helper::IMG_PRECURSOR.to_vec();

    for idx in 0..=(image.height() / 255) {
        let chunk = image.clone().crop(0, idx * 255, image.width(), 255);
        output.extend(image_helper::pack_image(&chunk));

        d30.write(output.as_slice()).await?;

        output.clear();
    }

    Ok(())
}
