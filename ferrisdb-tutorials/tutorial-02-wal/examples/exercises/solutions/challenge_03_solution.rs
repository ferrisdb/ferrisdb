//! Solution for Challenge 3: Concurrent Access

use std::sync::{Arc, RwLock};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write, Read, Seek, SeekFrom};
use std::path::Path;
use anyhow::Result;

/// A WAL that supports concurrent reads and exclusive writes
pub struct ConcurrentWal {
    inner: Arc<RwLock<WalInner>>,
}

struct WalInner {
    file: BufWriter<File>,
    next_sequence: u64,
}

impl ConcurrentWal {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .open(path)?;
            
        let inner = WalInner {
            file: BufWriter::new(file),
            next_sequence: 0,
        };
        
        Ok(Self {
            inner: Arc::new(RwLock::new(inner)),
        })
    }
    
    /// Append data (requires exclusive write access)
    pub fn append(&self, data: &[u8]) -> Result<u64> {
        let mut inner = self.inner.write()
            .map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
            
        let sequence = inner.next_sequence;
        
        // Write length + data
        inner.file.write_all(&(data.len() as u32).to_le_bytes())?;
        inner.file.write_all(data)?;
        inner.file.flush()?;
        
        inner.next_sequence += 1;
        Ok(sequence)
    }
    
    /// Read current sequence number (shared read access)
    pub fn current_sequence(&self) -> Result<u64> {
        let inner = self.inner.read()
            .map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
        Ok(inner.next_sequence)
    }
    
    /// Create a snapshot reader that won't block writers
    pub fn snapshot_reader(&self, path: &Path) -> Result<SnapshotReader> {
        // Take a read lock briefly to get current position
        let current_seq = {
            let inner = self.inner.read()
                .map_err(|_| anyhow::anyhow!("Lock poisoned"))?;
            inner.next_sequence
        };
        
        // Open a separate file handle for reading
        let file = File::open(path)?;
        
        Ok(SnapshotReader {
            file,
            max_sequence: current_seq,
        })
    }
}

/// A reader that reads up to a specific sequence number
/// and doesn't interfere with writers
pub struct SnapshotReader {
    file: File,
    max_sequence: u64,
}

impl SnapshotReader {
    pub fn read_entries(&mut self) -> Result<Vec<Vec<u8>>> {
        let mut entries = Vec::new();
        let mut sequence = 0;
        
        self.file.seek(SeekFrom::Start(0))?;
        
        while sequence < self.max_sequence {
            // Read entry length
            let mut len_bytes = [0u8; 4];
            match self.file.read_exact(&mut len_bytes) {
                Ok(_) => {},
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e.into()),
            }
            
            let length = u32::from_le_bytes(len_bytes) as usize;
            
            // Read entry data
            let mut data = vec![0u8; length];
            self.file.read_exact(&mut data)?;
            
            entries.push(data);
            sequence += 1;
        }
        
        Ok(entries)
    }
}

// Thread-safe wrapper for easy cloning
impl Clone for ConcurrentWal {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

unsafe impl Send for ConcurrentWal {}
unsafe impl Sync for ConcurrentWal {}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_concurrent_reads() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("concurrent.wal");
        
        let wal = ConcurrentWal::new(&path).unwrap();
        
        // Write some initial data
        for i in 0..5 {
            wal.append(format!("entry {}", i).as_bytes()).unwrap();
        }
        
        let wal_clone1 = wal.clone();
        let wal_clone2 = wal.clone();
        let path_clone1 = path.clone();
        let path_clone2 = path.clone();
        
        // Start multiple readers
        let reader1 = thread::spawn(move || {
            let mut snapshot = wal_clone1.snapshot_reader(&path_clone1).unwrap();
            snapshot.read_entries().unwrap()
        });
        
        let reader2 = thread::spawn(move || {
            let mut snapshot = wal_clone2.snapshot_reader(&path_clone2).unwrap();
            snapshot.read_entries().unwrap()
        });
        
        // Both readers should succeed
        let entries1 = reader1.join().unwrap();
        let entries2 = reader2.join().unwrap();
        
        assert_eq!(entries1.len(), 5);
        assert_eq!(entries2.len(), 5);
        assert_eq!(entries1, entries2);
    }
    
    #[test]
    fn test_read_during_write() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("read_write.wal");
        
        let wal = ConcurrentWal::new(&path).unwrap();
        
        // Write initial data
        wal.append(b"initial").unwrap();
        
        let wal_writer = wal.clone();
        let wal_reader = wal.clone();
        let path_reader = path.clone();
        
        // Start a writer thread
        let writer = thread::spawn(move || {
            for i in 0..10 {
                wal_writer.append(format!("write {}", i).as_bytes()).unwrap();
                thread::sleep(Duration::from_millis(10));
            }
        });
        
        // Start a reader thread
        let reader = thread::spawn(move || {
            // Take snapshot after some writes
            thread::sleep(Duration::from_millis(50));
            let mut snapshot = wal_reader.snapshot_reader(&path_reader).unwrap();
            snapshot.read_entries().unwrap()
        });
        
        writer.join().unwrap();
        let entries = reader.join().unwrap();
        
        // Reader should see some entries (at least initial + some writes)
        assert!(entries.len() >= 1);
        assert_eq!(entries[0], b"initial");
    }
}