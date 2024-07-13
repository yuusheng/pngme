mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let saner = u32::from_be_bytes([1, 2, 3, 4]);
    println!("{saner}");
    todo!()
}
