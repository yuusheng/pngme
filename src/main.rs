mod chunk;
mod chunk_type;
mod commands;
mod error;
mod png;

use anyhow::Result;
use clap::Parser;
use commands::{decode, encode, print, remove};

fn main() -> Result<()> {
    let args = commands::PngArgs::parse();
    match args.command {
        commands::PngCommand::Encode(encode_args) => encode(encode_args),
        commands::PngCommand::Decode(decode_args) => {
            if let Some(decode_content) = decode(decode_args) {
                println!("{}", decode_content)
            }
        }
        commands::PngCommand::Remove(remove_args) => remove(remove_args),
        commands::PngCommand::Print(print_args) => print(print_args),
    };
    Ok(())
}
