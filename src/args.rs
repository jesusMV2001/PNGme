use clap::{Parser, Subcommand};
use crate::chunk_type::ChunkType;

#[derive(Parser)]
#[command(name = "pngme", version, about = "Inserta mensajes secretos en imágenes PNG")]
pub struct Cli {
    #[command(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Subcommand)]
pub enum PngMeArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Parser, Debug)]
pub struct EncodeArgs {
    /// Ruta del archivo PNG
    pub file_path: String,
    /// Tipo de chunk (ej: "ruST")
    pub chunk_type: ChunkType,
    /// Mensaje a insertar
    pub message: String,
}

#[derive(Parser, Debug)]
pub struct DecodeArgs {
    pub file_path: String,
    pub chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
pub struct RemoveArgs {
    pub file_path: String,
    pub chunk_type: ChunkType,
}

#[derive(Parser, Debug)]
pub struct PrintArgs {
    pub file_path: String,
}