use std::path::PathBuf;
use std::result;

use clap::{Parser, Subcommand, ValueEnum};
use thiserror::Error;

use identicon::color;

const DEFAULT_SIZE: u32 = 60;
const DEFAULT_BACKGROUND: color::RGB = color::RGB::new(240, 240, 240);
const BACKGROUND_DELIMITER: &'static str = ",";

#[derive(Parser)]
/// This is an identicon generator.
///
/// Input your name and a file path to save your identicon image.
/// More customizing options are available. Use -h or --help for details.
#[clap(author, version)]
struct Cli {
    #[clap(value_parser, value_name = "STRING")]
    /// Your name, or any random string
    name: String,

    #[clap(short, long, default_value_t = DEFAULT_SIZE, value_parser = clap::value_parser!(u32).range(..613566757), value_name = "U32")]
    /// The number of pixels of each square in the generated identicon; must be less than 613566757 (image size in pixels must fit in u32)
    size: u32,

    #[clap(short, long, default_value_t = DEFAULT_BACKGROUND, value_parser = parse_color, value_name="RGB")]
    /// The background color in RGB format separated by ","; e.g. 255,0,0 (red)
    background: color::RGB,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ImageFormat {
    PNG,
    JPEG,
    GIF,
}

#[derive(Subcommand)]
enum Command {
    Render {
        #[clap(value_parser, value_name = "FILE")]
        /// The output file path; the extension determins the image format
        path: PathBuf,
    },
    Encode {
        #[clap(arg_enum, value_parser)]
        /// image format
        format: ImageFormat,
    },
}

#[derive(Error, Debug)]
enum Error {
    #[error("invalid color=[{val}], expect format=[<u8>,<u8>,<u8>]")]
    InvalidColor { val: String },
    #[error(transparent)]
    InvalidIdenticon(#[from] identicon::Error),
}

type Result<T> = result::Result<T, Error>;

fn parse_color(s: &str) -> Result<color::RGB> {
    let mut iter = s.split(BACKGROUND_DELIMITER);
    let r = iter
        .next()
        .and_then(|s| s.parse::<u8>().ok())
        .ok_or(Error::InvalidColor { val: s.to_string() })?;
    let g = iter
        .next()
        .and_then(|s| s.parse::<u8>().ok())
        .ok_or(Error::InvalidColor { val: s.to_string() })?;
    let b = iter
        .next()
        .and_then(|s| s.parse::<u8>().ok())
        .ok_or(Error::InvalidColor { val: s.to_string() })?;
    Ok(color::RGB::new(r, g, b))
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let identicon = identicon::Identicon::new(&cli.name, cli.size, cli.background)?;
    match &cli.command {
        Command::Render { path } => Ok(identicon.render(&path)?),
        Command::Encode {
            format: ImageFormat::PNG,
        } => Ok(println!("base64 encoded: {}", identicon.png()?)),
        Command::Encode {
            format: ImageFormat::JPEG,
        } => Ok(println!("base64 encoded: {}", identicon.jpeg()?)),
        Command::Encode {
            format: ImageFormat::GIF,
        } => Ok(println!("base64 encoded: {}", identicon.gif()?)),
    }
}
