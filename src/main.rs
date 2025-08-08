use clap::Parser;
use crate::args::{Cli, PngMeArgs};

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Result<T> = std::result::Result<T, &'static str>;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        PngMeArgs::Encode(args) => {
            println!("Encoding: file={}, type={}, message={}", args.file_path, args.chunk_type, args.message);
            commands::encode(args)?;
        }
        PngMeArgs::Decode(args) => {
            println!("Decoding: file={}, type={}", args.file_path, args.chunk_type);
            commands::decode(args)?;
        }
        PngMeArgs::Remove(args) => {
            println!("Removing: file={}, type={}", args.file_path, args.chunk_type);
            commands::remove(args)?;
        }
        PngMeArgs::Print(args) => {
            println!("Printing all chunks in: {}", args.file_path);
            commands::print_chunks(args)?;
        }
    }

    Ok(())
}
