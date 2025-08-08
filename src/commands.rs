use std::str::FromStr;
use crate::Result;
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<Png> {
    let mut png = Png::from_file(&args.file_path)?;

    let chunk_type = ChunkType::from_str(&*args.chunk_type)?;
    let chunk = Chunk::new(chunk_type, args.message.into_bytes());

    png.append_chunk(chunk);

    png.save(&args.file_path)?;

    Ok(png)
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<Png> {
    let png = Png::from_file(args.file_path)?;
    let chunk_type = ChunkType::from_str(&*args.chunk_type)?;

    if !chunk_type.is_valid() {
        return Err("Chunk type no valido")
    }

    let chunk = png.chunk_by_type(&*args.chunk_type)
        .ok_or("No se ha encontrado ningún chunk")?;


    println!("{}", chunk);

    Ok(png)
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<Png> {
    let mut png = Png::from_file(&args.file_path)?;

    png.remove_first_chunk(&*args.chunk_type)?;

    png.save(&args.file_path)?;

    Ok(png)
}

/// Prints all the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<Png> {
    let png = Png::from_file(args.file_path)?;

    for chunk in png.chunks() {
        println!("{}", chunk);
    }

    Ok(png)
}
