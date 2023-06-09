use crate::{chunk_type::{ChunkType}, Error};
use crc::{CRC_32_ISO_HDLC, Crc};
use core::fmt;
use crate::Result;
use std::fmt::Error as E;


#[derive(Debug, PartialEq, Eq)]
pub struct Chunk{
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk{
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk{
        let length:u32 = data.len().try_into().unwrap() ;
        let crc = Self::calaculate_crc(&chunk_type, data.clone());

        return Chunk { length, chunk_type, data, crc };
    }

    pub fn length(&self) -> u32{
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8]{
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String>{
        Ok(String::from_utf8(self.data.clone())?)
    }

    pub fn as_bytes(&self) -> Vec<u8>{
        self.length.to_be_bytes().iter().chain(self.chunk_type.bytes().iter())
        .chain(self.data.iter()).chain(self.crc.to_be_bytes().iter()).copied().collect()
    }

    fn calaculate_crc(chunk_type: &ChunkType, chunk_data: Vec<u8>) -> u32 {
        pub const ISO: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        return ISO.checksum(&chunk_type.bytes().iter().chain(chunk_data.iter()).copied().collect::<Vec<u8>>());
    }
}

impl TryFrom<&[u8]> for Chunk{
    type Error = Error;

     fn try_from(data: &[u8]) -> Result<Self>{
        let length = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let chunk_type = ChunkType{bytes: <[u8;4]>::try_from(&data[4..8]).unwrap()};
        let value = &data[8..data.len() - 4];
        let crc = u32::from_be_bytes(data[data.len()-4..].try_into().unwrap());

        if crc != Self::calaculate_crc(&chunk_type, value.to_vec()){
            return Err(Box::new(E));
        }

        return Ok(Chunk {length, chunk_type, data:value.to_vec() , crc});
    }

    
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk{{", )?;
        writeln!(f, " Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())

    }
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();
        
        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        
        let _chunk_string = format!("{}", chunk);
    }
}

