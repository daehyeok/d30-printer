use anyhow::Result;
use btleplug::api::BDAddr;
use clap::{Parser, command};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The text to be printed on the label.
    pub text: String,

    /// The MAC address of the D30 label maker. (Optional)
    #[arg(short, long)]
    addr: Option<String>,

    /// The font name or the path to the font file. If not set, 'Hack' font will be used by default. (Optional)
    #[arg(short, long)]
    pub font: Option<String>,
}

impl Config {
    pub fn get_addr(&self) -> Result<Option<BDAddr>> {
        match &self.addr {
            None => Ok(None),
            Some(addr) => {
                let bd_addr = BDAddr::from_str_delim(addr)?;
                Ok(Some(bd_addr))
            }
        }
    }
}
