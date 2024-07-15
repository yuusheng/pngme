use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PngError {
    #[error("Invalid chunkType param error")]
    ChunkTypeError,

    #[error("Invalid chunk error")]
    ChunkError,

    #[error("CRC error")]
    CRCError,

    #[error("Chunk header error")]
    ChunkHeaderError,

    #[error("IO error")]
    IOError(#[from] io::Error),
}
