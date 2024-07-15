use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};
use clap::{Args, Parser, Subcommand};
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct PngArgs {
    #[command(subcommand)]
    pub command: PngCommand,
}

#[derive(Subcommand, Debug)]
pub enum PngCommand {
    /// Encode your png files with some secret content
    Encode(EncodeArgs),

    /// Decode your secret content with your chunk type
    Decode(DecodeArgs),

    /// remove some secret content with chunk type
    Remove(RemoveArgs),

    /// Print all secret content
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    pub png_path: PathBuf,
    pub chunk_type: ChunkType,
    pub message: String,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    pub png_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    pub png_path: PathBuf,
    pub chunk_type: ChunkType,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    pub png_path: PathBuf,
}

pub fn read_file(path: &PathBuf) -> Vec<u8> {
    let mut f = File::open(&path).expect("File not found");
    let metadata = fs::metadata(&path).expect("Unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("Buffer overflow");
    buffer
}

pub fn encode(args: EncodeArgs) {
    let file_buffer = read_file(&args.png_path);
    let mut png = Png::try_from(file_buffer.as_ref()).unwrap();
    let chunk_type = ChunkType::from(args.chunk_type);
    let chunk = Chunk::new(chunk_type, args.message.into_bytes());
    png.append_chunk(chunk);
    if let Some(out_path) = args.output {
        fs::write(out_path, png.as_bytes()).unwrap();
    }
}

pub fn decode(decode_args: DecodeArgs) -> Option<String> {
    let file_buffer = read_file(&decode_args.png_path);
    let png = Png::try_from(file_buffer.as_ref()).unwrap();
    for chunk in png.chunks().iter() {
        if chunk.chunk_type().to_string() == decode_args.chunk_type.to_string() {
            return Some(chunk.data_as_string().unwrap());
        }
    }
    None
}

pub fn remove(remove_args: RemoveArgs) {
    let file_buffer = read_file(&remove_args.png_path);
    let mut png = Png::try_from(file_buffer.as_ref()).unwrap();
    png.remove_first_chunk(remove_args.chunk_type.to_string().as_str());
    fs::write(&remove_args.png_path, png.as_bytes()).unwrap();
    println!("Successfully removed!")
}

pub fn print(print_args: PrintArgs) {
    let file_buffer = read_file(&print_args.png_path);
    let png = Png::try_from(file_buffer.as_ref()).unwrap();
    println!("All messages:");
    for (i, chunk) in png.chunks().iter().enumerate() {
        println!("{} {:?}", i + 1, chunk.data_as_string())
    }
}
