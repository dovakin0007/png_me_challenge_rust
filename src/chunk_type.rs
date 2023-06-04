use std::{str::FromStr, fmt};

// use crate::Error;
use std::fmt::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType{
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.bytes;
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        let shifted_byte = self.bytes[0].clone() >> 5;

        let fifth_byte = shifted_byte & 0x01;

        if let 0 = fifth_byte{
            true
        }else{
            false
        }
    }
    pub fn is_public(&self) -> bool {
        let shifted_byte = self.bytes[1].clone() >> 5;

        let fifth_byte = shifted_byte & 0x01;

        if let 0 = fifth_byte{
            true
        }else{
            false
        }
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        let shifted_byte = self.bytes[2].clone() >> 5;

        let fifth_byte = shifted_byte & 0x01;

        if let 0 = fifth_byte{
            true
        }else{
            false
        }
    }
    pub fn is_safe_to_copy(&self)-> bool {
        let shifted_byte = self.bytes[3].clone() >> 5;

        let fifth_byte = shifted_byte & 0x01;

        if let 1 = fifth_byte{
            true
        }else{
            false
        }

    }
}


impl TryFrom<[u8; 4]> for ChunkType{
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for byte in value.iter(){
            if !byte.is_ascii_alphabetic(){
                return Err(Error);
            }
        }
        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType{
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value:[u8;4] = [0;4];
        let mut location: usize = 0;
        for byte in s.as_bytes(){
            if !byte.is_ascii_alphabetic() {
                return Err(Error);
            }else{
                value[location] = *byte;
                location +=1;
            }
        }
      Self::try_from(value)
    }
}

impl fmt::Display for ChunkType{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", String::from_utf8(self.bytes.to_vec()).unwrap())
    }
}



#[allow(unused_variables)]

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::System;
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
        println!("\n{}", &chunk.to_string());
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

