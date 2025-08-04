use crate::chunk_type::ChunkType;
use crc32fast::Hasher;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Self::create_crc(&chunk_type, &data);
        Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }
    fn create_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let mut hasher = Hasher::new();

        hasher.update(&chunk_type.bytes());
        hasher.update(data);

        hasher.finalize()
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
    pub fn data_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.data.clone())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(12 + self.data.len()); // 4 + 4 + data + 4

        result.extend_from_slice(&self.length.to_be_bytes());
        result.extend_from_slice(&self.chunk_type.bytes());
        result.extend_from_slice(&self.data);
        result.extend_from_slice(&self.crc.to_be_bytes());

        result
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Chunk Type: {}\nLength: {}\nCRC: {}\nData: {:?}",
            self.chunk_type, // usa el Display de ChunkType
            self.length,
            self.crc,
            String::from_utf8_lossy(&self.data) // muestra el data como texto si es UTF-8
        )
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        // LENGTH
        if value.len() < 8 {
            return Err("Vector demasiado pequeño");
        }
        let length = u32::from_be_bytes(
            value[..4]
                .try_into()
                .map_err(|_| "No se pudo leer la longitud")?,
        );

        // CHUNK TYPE
        let v: [u8; 4] = value[4..8]
            .try_into()
            .map_err(|_| "No se pudo leer el Chunk Type")?;
        let chunk_type = ChunkType::try_from(v)?;

        // DATA
        let n = 8 + length as usize;
        if value.len() < n + 4 {
            return Err("Vector demasiado pequeño, no tiene los datos indicados en la longitud");
        }
        let data = value[8..n].to_vec();

        // CRC
        let crc = u32::from_be_bytes(
            value[n..n + 4]
                .try_into()
                .map_err(|_| "No se pudo leer el crc")?,
        );
        let check_crc = Self::create_crc(&chunk_type, &data);
        if crc != check_crc {
            return Err("crc invalido");
        }

        Ok(Chunk {
            length,
            chunk_type,
            data,
            crc,
        })
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
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
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
