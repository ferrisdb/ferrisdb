//! Solution for Challenge 2: Log Rotation

use std::fs::{self, File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use anyhow::Result;

pub struct RotatingWal {
    base_path: PathBuf,
    current_file: Option<BufWriter<File>>,
    max_file_size: u64,
    current_size: u64,
    current_suffix: u32,
}

impl RotatingWal {
    pub fn new<P: AsRef<Path>>(base_path: P, max_file_size: u64) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        
        // Find the latest file
        let current_suffix = Self::find_latest_suffix(&base_path)?;
        
        let mut wal = Self {
            base_path,
            current_file: None,
            max_file_size,
            current_size: 0,
            current_suffix,
        };
        
        wal.open_current_file()?;
        Ok(wal)
    }
    
    fn find_latest_suffix(base_path: &Path) -> Result<u32> {
        if !base_path.exists() {
            return Ok(0);
        }
        
        let parent = base_path.parent().unwrap_or(Path::new("."));
        let base_name = base_path.file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid path"))?;
        
        let mut max_suffix = 0;
        
        // Check base file
        if base_path.exists() {
            max_suffix = 0;
        }
        
        // Check rotated files
        for entry in fs::read_dir(parent)? {
            let entry = entry?;
            let name = entry.file_name();
            let name_str = name.to_str().unwrap_or("");
            
            if name_str.starts_with(base_name) && name_str.contains('.') {
                if let Some(suffix_str) = name_str.split('.').last() {
                    if let Ok(suffix) = suffix_str.parse::<u32>() {
                        max_suffix = max_suffix.max(suffix);
                    }
                }
            }
        }
        
        Ok(max_suffix)
    }
    
    fn current_path(&self) -> PathBuf {
        if self.current_suffix == 0 {
            self.base_path.clone()
        } else {
            let mut path = self.base_path.clone();
            let name = format!("{}.{}", 
                path.file_name().unwrap().to_str().unwrap(),
                self.current_suffix
            );
            path.set_file_name(name);
            path
        }
    }
    
    fn open_current_file(&mut self) -> Result<()> {
        let path = self.current_path();
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)?;
            
        self.current_size = file.metadata()?.len();
        self.current_file = Some(BufWriter::new(file));
        
        Ok(())
    }
    
    fn rotate(&mut self) -> Result<()> {
        // Close current file
        if let Some(mut file) = self.current_file.take() {
            file.flush()?;
            file.get_mut().sync_all()?;
        }
        
        // Move to next suffix
        self.current_suffix += 1;
        self.current_size = 0;
        
        // Open new file
        self.open_current_file()?;
        
        Ok(())
    }
    
    pub fn append(&mut self, data: &[u8]) -> Result<()> {
        // Check if rotation is needed
        if self.current_size + data.len() as u64 > self.max_file_size {
            self.rotate()?;
        }
        
        // Write data
        if let Some(file) = &mut self.current_file {
            file.write_all(data)?;
            file.flush()?;
            self.current_size += data.len() as u64;
        }
        
        Ok(())
    }
    
    pub fn read_all_files(&self) -> Result<Vec<Vec<u8>>> {
        let mut all_data = Vec::new();
        
        // Read base file
        if self.base_path.exists() {
            let data = fs::read(&self.base_path)?;
            if !data.is_empty() {
                all_data.push(data);
            }
        }
        
        // Read rotated files in order
        let mut suffix = 1;
        loop {
            let mut path = self.base_path.clone();
            let name = format!("{}.{}", 
                path.file_name().unwrap().to_str().unwrap(),
                suffix
            );
            path.set_file_name(name);
            
            if !path.exists() {
                break;
            }
            
            let data = fs::read(&path)?;
            if !data.is_empty() {
                all_data.push(data);
            }
            
            suffix += 1;
        }
        
        Ok(all_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_rotation() {
        let dir = tempdir().unwrap();
        let base_path = dir.path().join("test.wal");
        
        // Small max size to force rotation
        let mut wal = RotatingWal::new(&base_path, 100).unwrap();
        
        // Write data that will cause rotation
        for i in 0..10 {
            let data = format!("Entry {}: {}\n", i, "x".repeat(20));
            wal.append(data.as_bytes()).unwrap();
        }
        
        // Check that multiple files were created
        assert!(base_path.exists());
        assert!(dir.path().join("test.wal.1").exists());
        assert!(dir.path().join("test.wal.2").exists());
        
        // Read all data back
        let all_files = wal.read_all_files().unwrap();
        assert!(all_files.len() >= 3);
        
        // Verify data integrity
        let mut all_content = String::new();
        for file_data in all_files {
            all_content.push_str(&String::from_utf8(file_data).unwrap());
        }
        
        for i in 0..10 {
            assert!(all_content.contains(&format!("Entry {}", i)));
        }
    }
}