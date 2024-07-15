#![allow(dead_code)]
use std::{fmt::Display, str::FromStr};

use crate::error::PngError;

#[derive(Debug, PartialEq, Clone)]
pub struct ChunkType {
    value: [u8; 4],
}

impl ChunkType {
    pub fn new(value: [u8; 4]) -> Self {
        Self { value }
    }

    pub fn bytes(self) -> [u8; 4] {
        self.value
    }

    fn is_critical(&self) -> bool {
        bit5_is_zero(&self.value[0])
    }
    fn is_public(&self) -> bool {
        bit5_is_zero(&self.value[1])
    }
    fn is_reserved_bit_valid(&self) -> bool {
        bit5_is_zero(&self.value[2])
    }
    fn is_safe_to_copy(&self) -> bool {
        !bit5_is_zero(&self.value[3])
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = PngError;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self { value })
    }
}

impl FromStr for ChunkType {
    type Err = PngError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr = s.as_bytes();
        for byte in arr.iter() {
            if !is_lower_case_letter(byte) && !is_upper_case_letter(byte) {
                return Err(PngError::ChunkTypeError);
            }
        }

        Ok(Self::try_from([arr[0], arr[1], arr[2], arr[3]])?)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.value).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", s)
    }
}

fn is_upper_case_letter(byte: &u8) -> bool {
    (&65..&90).contains(&byte)
}

fn is_lower_case_letter(byte: &u8) -> bool {
    (&97..&122).contains(&byte)
}

fn bit5_is_zero(bit5: &u8) -> bool {
    (bit5 & 0x20) != 0x20
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
