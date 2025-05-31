use anyhow::Result;
use tutorial_02_wal::{Operation, SyncMode, WalBuilder};

fn main() -> Result<()> {
    println!("🚀 Tutorial 2: Write-Ahead Log Demo\n");
    
    // Create a WAL with different sync modes
    let wal_path = "demo.wal";
    
    println!("Creating WAL at: {}", wal_path);
    let mut wal = WalBuilder::new(wal_path)
        .sync_mode(SyncMode::DataOnly)
        .max_file_size(10 * 1024 * 1024) // 10MB
        .build()?;
    
    // Demonstrate basic operations
    println!("\n📝 Writing operations to WAL:");
    
    let seq1 = wal.append(Operation::Set {
        key: "user:1".to_string(),
        value: "Alice".to_string(),
    })?;
    println!("  - Set user:1 = Alice (sequence: {})", seq1);
    
    let seq2 = wal.append(Operation::Set {
        key: "user:2".to_string(),
        value: "Bob".to_string(),
    })?;
    println!("  - Set user:2 = Bob (sequence: {})", seq2);
    
    let seq3 = wal.append(Operation::Delete {
        key: "temp:session".to_string(),
    })?;
    println!("  - Delete temp:session (sequence: {})", seq3);
    
    // Simulate crash and recovery
    println!("\n💥 Simulating crash...");
    drop(wal);
    
    println!("\n🔄 Recovering from WAL:");
    let recovered_wal = WalBuilder::new(wal_path).build()?;
    let entries = recovered_wal.recover_entries()?;
    
    println!("\nRecovered {} entries:", entries.len());
    for entry in &entries {
        match &entry.operation {
            Operation::Set { key, value } => {
                println!("  [{}] Set {} = {}", entry.sequence, key, value);
            }
            Operation::Delete { key } => {
                println!("  [{}] Delete {}", entry.sequence, key);
            }
        }
    }
    
    println!("\n✅ Recovery successful! Your data survived the crash!");
    
    // Clean up
    std::fs::remove_file(wal_path).ok();
    
    Ok(())
}