//! Solution for Challenge 1: Compression

use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write, Seek};
use std::path::Path;
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;

pub struct CompressedWal {
    file: BufWriter<File>,
    compression_enabled: bool,
    next_sequence: u64,
}

const COMPRESSED_WAL_MAGIC: u32 = 0x43574C21; // "CWL!"
const VERSION_COMPRESSED: u32 = 2;

impl CompressedWal {
    pub fn new<P: AsRef<Path>>(path: P, compression: bool) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;
            
        let mut wal = Self {
            file: BufWriter::new(file),
            compression_enabled: compression,
            next_sequence: 0,
        };
        
        wal.write_header()?;
        Ok(wal)
    }
    
    fn write_header(&mut self) -> Result<()> {
        self.file.write_u32::<LittleEndian>(COMPRESSED_WAL_MAGIC)?;
        self.file.write_u32::<LittleEndian>(VERSION_COMPRESSED)?;
        self.file.write_u8(if self.compression_enabled { 1 } else { 0 })?;
        self.file.sync_all()?;
        Ok(())
    }
    
    pub fn append(&mut self, data: &[u8]) -> Result<u64> {
        let sequence = self.next_sequence;
        
        let encoded = if self.compression_enabled {
            // Compress the data
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(data)?;
            encoder.finish()?
        } else {
            data.to_vec()
        };
        
        // Write length and compressed data
        self.file.write_u32::<LittleEndian>(encoded.len() as u32)?;
        self.file.write_all(&encoded)?;
        self.file.sync_data()?;
        
        self.next_sequence += 1;
        Ok(sequence)
    }
    
    pub fn read_all(&self, path: &Path) -> Result<Vec<Vec<u8>>> {
        let mut file = File::open(path)?;
        let mut reader = BufReader::new(&mut file);
        
        // Read header
        let magic = reader.read_u32::<LittleEndian>()?;
        if magic != COMPRESSED_WAL_MAGIC {
            return Err(anyhow::anyhow!("Invalid magic number"));
        }
        
        let _version = reader.read_u32::<LittleEndian>()?;
        let compression = reader.read_u8()? == 1;
        
        let mut entries = Vec::new();
        
        loop {
            // Read entry length
            let length = match reader.read_u32::<LittleEndian>() {
                Ok(len) => len as usize,
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            };
            
            // Read entry data
            let mut compressed_data = vec![0u8; length];
            reader.read_exact(&mut compressed_data)?;
            
            // Decompress if needed
            let data = if compression {
                let mut decoder = GzDecoder::new(&compressed_data[..]);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                decompressed
            } else {
                compressed_data
            };
            
            entries.push(data);
        }
        
        Ok(entries)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_compression_reduces_size() {
        let dir = tempdir().unwrap();
        let uncompressed_path = dir.path().join("uncompressed.wal");
        let compressed_path = dir.path().join("compressed.wal");
        
        // Create repetitive data that compresses well
        let data = "Hello World! ".repeat(1000).into_bytes();
        
        // Write uncompressed
        {
            let mut wal = CompressedWal::new(&uncompressed_path, false).unwrap();
            wal.append(&data).unwrap();
        }
        
        // Write compressed
        {
            let mut wal = CompressedWal::new(&compressed_path, true).unwrap();
            wal.append(&data).unwrap();
        }
        
        // Compare sizes
        let uncompressed_size = std::fs::metadata(&uncompressed_path).unwrap().len();
        let compressed_size = std::fs::metadata(&compressed_path).unwrap().len();
        
        println!("Uncompressed: {} bytes", uncompressed_size);
        println!("Compressed: {} bytes", compressed_size);
        println!("Compression ratio: {:.2}%", 
            (compressed_size as f64 / uncompressed_size as f64) * 100.0);
        
        assert!(compressed_size < uncompressed_size / 2); // Should compress well
        
        // Verify data integrity
        let wal = CompressedWal::new(&compressed_path, true).unwrap();
        let entries = wal.read_all(&compressed_path).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0], data);
    }
}