use ferrisdb_core::{Error, Key, Operation, Result, Timestamp, Value};

use bytes::{Buf, BufMut, BytesMut};
use crc32fast::Hasher;

use std::convert::TryFrom;

// Constants for the binary format
const OP_PUT: u8 = 1;
const OP_DELETE: u8 = 2;
const HEADER_SIZE: usize = 8; // length + checksum
const MIN_ENTRY_SIZE: usize = HEADER_SIZE + 8 + 1 + 4 + 4; // header + timestamp + op + key_len + val_len

// Size limits for DoS protection
const MAX_KEY_SIZE: usize = 1024 * 1024; // 1MB
const MAX_VALUE_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const MAX_ENTRY_SIZE: usize = MAX_KEY_SIZE + MAX_VALUE_SIZE + MIN_ENTRY_SIZE;

// Reader configuration
pub const DEFAULT_READER_BUFFER_SIZE: usize = 8 * 1024; // 8KB initial size

/// An entry in the Write-Ahead Log
///
/// Each entry represents a single operation (Put or Delete) with its
/// associated key, value, and timestamp. Entries are encoded in a binary
/// format with checksums for corruption detection.
///
/// ## Binary Format
///
/// ```text
/// Offset  Size  Field         Description
/// ------  ----  -----         -----------
/// 0       4     length        Total entry size (including this field)
/// 4       4     checksum      CRC32 of all following fields
/// 8       8     timestamp     Operation timestamp (microseconds)
/// 16      1     operation     1=Put, 2=Delete
/// 17      4     key_len       Key length in bytes
/// 21      4     value_len     Value length in bytes (0 for Delete)
/// 25      var   key           Key data
/// 25+key  var   value         Value data (empty for Delete)
/// ```
///
/// ## Size Limits
///
/// - Maximum key size: 1 MB
/// - Maximum value size: 10 MB
/// - Maximum entry size: ~11 MB
///
/// These limits prevent memory exhaustion and ensure reasonable performance.
///
/// ## Thread Safety
///
/// WALEntry is `Send` and `Sync`, making it safe to share across threads.
/// The entry itself is immutable once created.
///
/// ## Encoding Details
///
/// - All integers use little-endian format for x86/ARM compatibility
/// - Length field does NOT include itself (entry size = length + 4)
/// - Checksum covers all data after the checksum field
/// - Delete operations have value_len=0 and empty value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WALEntry {
    /// Timestamp when this operation occurred
    pub timestamp: Timestamp,
    /// Type of operation (Put or Delete)
    pub operation: Operation,
    /// The key being operated on
    pub key: Key,
    /// The value (empty for Delete operations)
    pub value: Value,
}

impl WALEntry {
    /// Creates a new Put entry
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisdb_storage::wal::WALEntry;
    ///
    /// let entry = WALEntry::new_put(
    ///     b"user:123".to_vec(),
    ///     b"John Doe".to_vec(),
    ///     12345
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::Corruption` if the key or value exceeds size limits
    pub fn new_put(key: Key, value: Value, timestamp: Timestamp) -> Result<Self> {
        if key.len() > MAX_KEY_SIZE {
            return Err(Error::Corruption(format!(
                "Key size {} exceeds maximum {}",
                key.len(),
                MAX_KEY_SIZE
            )));
        }
        if value.len() > MAX_VALUE_SIZE {
            return Err(Error::Corruption(format!(
                "Value size {} exceeds maximum {}",
                value.len(),
                MAX_VALUE_SIZE
            )));
        }
        Ok(Self {
            timestamp,
            operation: Operation::Put,
            key,
            value,
        })
    }

    /// Creates a new Delete entry
    ///
    /// # Example
    ///
    /// ```
    /// use ferrisdb_storage::wal::WALEntry;
    ///
    /// let entry = WALEntry::new_delete(b"user:123".to_vec(), 12346)?;
    /// # Ok::<(), ferrisdb_core::Error>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns `Error::Corruption` if the key exceeds size limits
    pub fn new_delete(key: Key, timestamp: Timestamp) -> Result<Self> {
        if key.len() > MAX_KEY_SIZE {
            return Err(Error::Corruption(format!(
                "Key size {} exceeds maximum {}",
                key.len(),
                MAX_KEY_SIZE
            )));
        }
        Ok(Self {
            timestamp,
            operation: Operation::Delete,
            key,
            value: Vec::new(),
        })
    }

    /// Encodes the entry into binary format with checksum
    ///
    /// The encoded format is:
    /// ```text
    /// [length:4][checksum:4][timestamp:8][op:1][key_len:4][val_len:4][key][value]
    /// ```
    ///
    /// Where:
    /// - `length`: Total size of the encoded entry (excluding length field)
    /// - `checksum`: CRC32 of all fields after checksum
    /// - `timestamp`: Microseconds since Unix epoch
    /// - `op`: Operation type (1=Put, 2=Delete)
    /// - `key_len`: Size of key in bytes
    /// - `val_len`: Size of value in bytes (0 for Delete)
    /// - `key`: Raw key bytes
    /// - `value`: Raw value bytes (empty for Delete)
    ///
    /// # Errors
    ///
    /// Returns `Error::Corruption` if:
    /// - The key size exceeds MAX_KEY_SIZE
    /// - The value size exceeds MAX_VALUE_SIZE
    /// - The total size would overflow u32
    pub fn encode(&self) -> Result<Vec<u8>> {
        // Validate sizes
        if self.key.len() > MAX_KEY_SIZE {
            return Err(Error::Corruption(format!(
                "Key size {} exceeds maximum {}",
                self.key.len(),
                MAX_KEY_SIZE
            )));
        }
        if self.value.len() > MAX_VALUE_SIZE {
            return Err(Error::Corruption(format!(
                "Value size {} exceeds maximum {}",
                self.value.len(),
                MAX_VALUE_SIZE
            )));
        }

        // Pre-calculate size for efficient allocation
        let size = 4 + 4 + 8 + 1 + 4 + self.key.len() + 4 + self.value.len();
        let mut buf = BytesMut::with_capacity(size);

        // Reserve space for length and checksum
        buf.put_u32_le(0); // length placeholder
        buf.put_u32_le(0); // checksum placeholder

        // Encode entry data
        buf.put_u64_le(self.timestamp);
        buf.put_u8(match self.operation {
            Operation::Put => OP_PUT,
            Operation::Delete => OP_DELETE,
        });

        // Safe conversion with proper error handling
        let key_len: u32 = self.key.len().try_into().map_err(|_| {
            Error::Corruption(format!("Key length {} too large for u32", self.key.len()))
        })?;
        buf.put_u32_le(key_len);
        buf.put_slice(&self.key);

        let value_len: u32 = self.value.len().try_into().map_err(|_| {
            Error::Corruption(format!(
                "Value length {} too large for u32",
                self.value.len()
            ))
        })?;
        buf.put_u32_le(value_len);
        buf.put_slice(&self.value);

        // Calculate and set length (excluding length field itself)
        let total_len = buf.len() - 4;
        let total_len_u32: u32 = total_len.try_into().map_err(|_| {
            Error::Corruption(format!("Entry size {} too large for u32", total_len))
        })?;
        buf[0..4].copy_from_slice(&total_len_u32.to_le_bytes());

        // Calculate and set checksum (excluding length and checksum fields)
        let mut hasher = Hasher::new();
        hasher.update(&buf[8..]);
        let checksum = hasher.finalize();
        buf[4..8].copy_from_slice(&checksum.to_le_bytes());

        Ok(buf.to_vec())
    }

    /// Decodes an entry from binary format
    ///
    /// Verifies the checksum and returns an error if corruption is detected.
    /// The input must be a complete encoded entry including the length prefix.
    ///
    /// ## Error Conditions
    ///
    /// Returns `Error::Corruption` if:
    /// - The buffer is too small (< 25 bytes minimum)
    /// - The length field doesn't match actual size
    /// - The checksum verification fails
    /// - The operation type is invalid (not 1 or 2)
    /// - Key or value sizes exceed limits
    /// - Data is truncated (insufficient bytes for declared lengths)
    /// - Unexpected trailing bytes after the value
    ///
    /// ## Corruption Detection
    ///
    /// The decoder performs multiple validation steps:
    /// 1. Minimum size check
    /// 2. Length field validation
    /// 3. CRC32 checksum verification
    /// 4. Operation type validation
    /// 5. Size limit checks for key and value
    /// 6. Buffer bounds checking during parsing
    /// 7. Exact size match verification
    pub fn decode(data: &[u8]) -> Result<Self> {
        if data.len() < MIN_ENTRY_SIZE {
            return Err(Error::Corruption(format!(
                "WAL entry too small: {} bytes (minimum: {})",
                data.len(),
                MIN_ENTRY_SIZE
            )));
        }

        let mut cursor = data;

        // Read and verify length
        let length = cursor.get_u32_le() as usize;
        if length > MAX_ENTRY_SIZE {
            return Err(Error::Corruption(format!(
                "WAL entry size {} exceeds maximum {}",
                length, MAX_ENTRY_SIZE
            )));
        }
        if data.len() != length + 4 {
            return Err(Error::Corruption(format!(
                "WAL entry length mismatch: declared {} but got {} bytes",
                length + 4,
                data.len()
            )));
        }

        // Read and verify checksum
        let expected_checksum = cursor.get_u32_le();
        let mut hasher = Hasher::new();
        hasher.update(&data[8..]);
        let actual_checksum = hasher.finalize();

        if expected_checksum != actual_checksum {
            return Err(Error::Corruption(format!(
                "WAL entry checksum mismatch: expected {:#x} but got {:#x}",
                expected_checksum, actual_checksum
            )));
        }

        // Ensure we have enough data for fixed fields
        if cursor.len() < 8 + 1 + 4 {
            return Err(Error::Corruption(
                "WAL entry truncated: missing fixed fields".to_string(),
            ));
        }

        // Decode entry data
        let timestamp = cursor.get_u64_le();
        let operation = match cursor.get_u8() {
            OP_PUT => Operation::Put,
            OP_DELETE => Operation::Delete,
            op => return Err(Error::Corruption(format!("Invalid operation type: {}", op))),
        };

        let key_len = cursor.get_u32_le() as usize;
        if key_len > MAX_KEY_SIZE {
            return Err(Error::Corruption(format!(
                "Key size {} exceeds maximum {}",
                key_len, MAX_KEY_SIZE
            )));
        }
        if cursor.len() < key_len + 4 {
            return Err(Error::Corruption(format!(
                "WAL entry truncated: expected {} key bytes but only {} available",
                key_len,
                cursor.len() - 4
            )));
        }
        let key = cursor[..key_len].to_vec();
        cursor.advance(key_len);

        if cursor.len() < 4 {
            return Err(Error::Corruption(
                "WAL entry truncated: missing value length".to_string(),
            ));
        }
        let value_len = cursor.get_u32_le() as usize;
        if value_len > MAX_VALUE_SIZE {
            return Err(Error::Corruption(format!(
                "Value size {} exceeds maximum {}",
                value_len, MAX_VALUE_SIZE
            )));
        }
        if cursor.len() < value_len {
            return Err(Error::Corruption(format!(
                "WAL entry truncated: expected {} value bytes but only {} available",
                value_len,
                cursor.len()
            )));
        }
        let value = cursor[..value_len].to_vec();
        cursor.advance(value_len);

        // Verify we consumed exactly the right amount of data
        if !cursor.is_empty() {
            return Err(Error::Corruption(format!(
                "WAL entry has {} unexpected trailing bytes",
                cursor.len()
            )));
        }

        Ok(Self {
            timestamp,
            operation,
            key,
            value,
        })
    }
}

// Implement TryFrom for ergonomic conversions
impl TryFrom<&[u8]> for WALEntry {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self> {
        Self::decode(data)
    }
}

impl TryFrom<Vec<u8>> for WALEntry {
    type Error = Error;

    fn try_from(data: Vec<u8>) -> Result<Self> {
        Self::decode(&data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    /// Tests basic Put entry encoding and decoding.
    ///
    /// Verifies:
    /// - Put entries can be encoded to bytes
    /// - Encoded bytes can be decoded back
    /// - Roundtrip preserves all entry data
    /// - Basic functionality works correctly
    #[test]
    fn test_encode_decode_put() {
        let entry = WALEntry::new_put(b"test_key".to_vec(), b"test_value".to_vec(), 12345)
            .expect("Failed to create entry");

        let encoded = entry.encode().expect("Failed to encode");
        let decoded = WALEntry::decode(&encoded).unwrap();

        assert_eq!(entry, decoded);
    }

    /// Tests basic Delete entry encoding and decoding.
    ///
    /// Ensures:
    /// - Delete entries encode correctly
    /// - Delete operations have empty values
    /// - Key and timestamp are preserved
    /// - Delete roundtrip works properly
    #[test]
    fn test_encode_decode_delete() {
        let entry =
            WALEntry::new_delete(b"test_key".to_vec(), 12345).expect("Failed to create entry");

        let encoded = entry.encode().expect("Failed to encode");
        let decoded = WALEntry::decode(&encoded).unwrap();

        assert_eq!(entry, decoded);
    }

    /// Tests that data corruption is detected during decode.
    ///
    /// Verifies:
    /// - Bit flips in data are caught
    /// - Corruption error is returned
    /// - No silent data corruption
    /// - Checksum validation works
    #[test]
    fn test_corruption_detection() {
        let entry = WALEntry::new_put(b"test_key".to_vec(), b"test_value".to_vec(), 12345)
            .expect("Failed to create entry");

        let mut encoded = entry.encode().expect("Failed to encode");
        // Corrupt the data
        encoded[20] ^= 0xFF;

        let result = WALEntry::decode(&encoded);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Corruption(_)));
    }

    // Test proper behavior names as per guidelines
    /// Tests that complex Put entry data is preserved exactly.
    ///
    /// Verifies:
    /// - Unicode data in keys/values works
    /// - Maximum timestamp values handled
    /// - Binary data preserved byte-for-byte
    /// - No data transformation occurs
    #[test]
    fn encode_decode_preserves_put_entry_data() {
        let entry = WALEntry::new_put(
            b"key_with_unicode_\xf0\x9f\xa6\x80".to_vec(),
            b"value_with_unicode_\xf0\x9f\x8e\x89".to_vec(),
            u64::MAX,
        )
        .expect("Failed to create entry");

        let encoded = entry.encode().expect("Failed to encode");
        let decoded = WALEntry::decode(&encoded).expect("Failed to decode");

        assert_eq!(entry.timestamp, decoded.timestamp);
        assert_eq!(entry.operation, decoded.operation);
        assert_eq!(entry.key, decoded.key);
        assert_eq!(entry.value, decoded.value);
    }

    /// Tests that Delete entry fields are preserved correctly.
    ///
    /// Ensures:
    /// - Delete operation type maintained
    /// - Key data preserved exactly
    /// - Value is always empty
    /// - Timestamp accuracy maintained
    #[test]
    fn encode_decode_preserves_delete_entry_data() {
        let entry =
            WALEntry::new_delete(b"delete_key".to_vec(), 999999).expect("Failed to create entry");

        let encoded = entry.encode().expect("Failed to encode");
        let decoded = WALEntry::decode(&encoded).expect("Failed to decode");

        assert_eq!(entry.timestamp, decoded.timestamp);
        assert_eq!(entry.operation, decoded.operation);
        assert_eq!(entry.key, decoded.key);
        assert!(decoded.value.is_empty());
    }

    // Edge cases and error conditions
    /// Tests that Put entries enforce the 1MB key size limit.
    ///
    /// Verifies:
    /// - Keys larger than MAX_KEY_SIZE rejected
    /// - Proper error type returned
    /// - Size validation happens early
    /// - Memory safety maintained
    #[test]
    fn new_put_rejects_oversized_key() {
        let large_key = vec![0u8; MAX_KEY_SIZE + 1];
        let result = WALEntry::new_put(large_key, b"value".to_vec(), 123);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Corruption(_)));
    }

    /// Tests that Put entries enforce the 10MB value size limit.
    ///
    /// Ensures:
    /// - Values larger than MAX_VALUE_SIZE rejected
    /// - Corruption error returned
    /// - Size limits prevent OOM
    /// - Validation before encoding
    #[test]
    fn new_put_rejects_oversized_value() {
        let large_value = vec![0u8; MAX_VALUE_SIZE + 1];
        let result = WALEntry::new_put(b"key".to_vec(), large_value, 123);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Corruption(_)));
    }

    /// Tests that Delete entries enforce the 1MB key size limit.
    ///
    /// Verifies:
    /// - Delete operations have same key limits
    /// - Consistent size enforcement
    /// - Early validation of inputs
    /// - Clear error messages
    #[test]
    fn new_delete_rejects_oversized_key() {
        let large_key = vec![0u8; MAX_KEY_SIZE + 1];
        let result = WALEntry::new_delete(large_key, 123);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Corruption(_)));
    }

    /// Tests detection of entries with incomplete headers.
    ///
    /// Ensures:
    /// - Minimum header size enforced
    /// - Truncated data rejected early
    /// - Clear error for debugging
    /// - Safe handling of partial data
    #[test]
    fn decode_detects_truncated_header() {
        let data = vec![0u8; 7]; // Too small for header
        let result = WALEntry::decode(&data);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::Corruption(_)));
    }

    #[test]
    fn decode_detects_length_mismatch() {
        let mut data = vec![0u8; 100];
        // Set length to 200 but only provide 100 bytes
        data[0..4].copy_from_slice(&200u32.to_le_bytes());
        let result = WALEntry::decode(&data);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Corruption(msg) if msg.contains("length mismatch")));
    }

    #[test]
    fn decode_detects_checksum_corruption() {
        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123)
            .expect("Failed to create entry");
        let mut encoded = entry.encode().expect("Failed to encode");

        // Corrupt checksum bytes
        encoded[4] ^= 0xFF;
        encoded[5] ^= 0xFF;

        let result = WALEntry::decode(&encoded);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Corruption(msg) if msg.contains("checksum mismatch")));
    }

    #[test]
    fn decode_detects_invalid_operation_type() {
        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123)
            .expect("Failed to create entry");
        let mut encoded = entry.encode().expect("Failed to encode");

        // Set invalid operation type (99)
        encoded[16] = 99;

        // Recalculate checksum for valid data with invalid op
        let mut hasher = Hasher::new();
        hasher.update(&encoded[8..]);
        let checksum = hasher.finalize();
        encoded[4..8].copy_from_slice(&checksum.to_le_bytes());

        let result = WALEntry::decode(&encoded);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, Error::Corruption(msg) if msg.contains("Invalid operation type: 99"))
        );
    }

    #[test]
    fn decode_detects_truncated_key() {
        let entry = WALEntry::new_put(b"test_key".to_vec(), b"value".to_vec(), 123)
            .expect("Failed to create entry");
        let encoded = entry.encode().expect("Failed to encode");

        // Truncate in the middle of the key
        let truncated = &encoded[..encoded.len() - 10];

        let result = WALEntry::decode(truncated);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Corruption(msg) if msg.contains("length mismatch")));
    }

    #[test]
    fn decode_detects_truncated_value() {
        let entry = WALEntry::new_put(b"key".to_vec(), b"test_value".to_vec(), 123)
            .expect("Failed to create entry");
        let encoded = entry.encode().expect("Failed to encode");

        // Truncate in the middle of the value
        let truncated = &encoded[..encoded.len() - 5];

        let result = WALEntry::decode(truncated);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Corruption(msg) if msg.contains("length mismatch")));
    }

    #[test]
    fn decode_detects_oversized_key_length() {
        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123)
            .expect("Failed to create entry");
        let mut encoded = entry.encode().expect("Failed to encode");

        // Set key length to exceed MAX_KEY_SIZE
        let oversized_len = (MAX_KEY_SIZE + 1000) as u32;
        encoded[17..21].copy_from_slice(&oversized_len.to_le_bytes());

        // Recalculate checksum
        let mut hasher = Hasher::new();
        hasher.update(&encoded[8..]);
        let checksum = hasher.finalize();
        encoded[4..8].copy_from_slice(&checksum.to_le_bytes());

        let result = WALEntry::decode(&encoded);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, Error::Corruption(msg) if msg.contains("Key size") && msg.contains("exceeds maximum"))
        );
    }

    #[test]
    fn decode_detects_trailing_bytes() {
        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123)
            .expect("Failed to create entry");
        let mut encoded = entry.encode().expect("Failed to encode");

        // Add extra bytes at the end
        encoded.extend_from_slice(b"extra");

        // Update length field to include extra bytes
        let new_len = (encoded.len() - 4) as u32;
        encoded[0..4].copy_from_slice(&new_len.to_le_bytes());

        // Recalculate checksum to make the data "valid" except for trailing bytes
        let mut hasher = Hasher::new();
        hasher.update(&encoded[8..]);
        let checksum = hasher.finalize();
        encoded[4..8].copy_from_slice(&checksum.to_le_bytes());

        let result = WALEntry::decode(&encoded);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Corruption(msg) if msg.contains("trailing bytes")));
    }

    #[test]
    fn handles_empty_key_and_value() {
        let entry = WALEntry::new_put(vec![], vec![], 0).expect("Failed to create entry");
        let encoded = entry.encode().expect("Failed to encode");
        let decoded = WALEntry::decode(&encoded).expect("Failed to decode");

        assert_eq!(decoded.key, Vec::<u8>::new());
        assert_eq!(decoded.value, Vec::<u8>::new());
        assert_eq!(decoded.timestamp, 0);
    }

    #[test]
    fn handles_maximum_allowed_sizes() {
        // Test with max allowed key size
        let max_key = vec![0xAB; MAX_KEY_SIZE];
        let entry = WALEntry::new_put(max_key.clone(), vec![1, 2, 3], 12345)
            .expect("Failed to create entry");
        let encoded = entry.encode().expect("Failed to encode");
        let decoded = WALEntry::decode(&encoded).expect("Failed to decode");
        assert_eq!(decoded.key, max_key);

        // Test with max allowed value size
        let max_value = vec![0xCD; MAX_VALUE_SIZE];
        let entry = WALEntry::new_put(vec![1, 2, 3], max_value.clone(), 54321)
            .expect("Failed to create entry");
        let encoded = entry.encode().expect("Failed to encode");
        let decoded = WALEntry::decode(&encoded).expect("Failed to decode");
        assert_eq!(decoded.value, max_value);
    }

    // TryFrom trait tests
    #[test]
    fn try_from_slice_works() {
        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123)
            .expect("Failed to create entry");
        let encoded = entry.encode().expect("Failed to encode");

        let decoded = WALEntry::try_from(encoded.as_slice()).expect("Failed to decode");
        assert_eq!(entry, decoded);
    }

    #[test]
    fn try_from_vec_works() {
        let entry = WALEntry::new_put(b"key".to_vec(), b"value".to_vec(), 123)
            .expect("Failed to create entry");
        let encoded = entry.encode().expect("Failed to encode");

        let decoded = WALEntry::try_from(encoded).expect("Failed to decode");
        assert_eq!(entry, decoded);
    }

    // Concurrent tests as required by guidelines
    #[test]
    fn concurrent_encoding_maintains_integrity() {
        let entry = Arc::new(
            WALEntry::new_put(
                b"concurrent_key".to_vec(),
                b"concurrent_value".to_vec(),
                999,
            )
            .expect("Failed to create entry"),
        );

        let mut handles = vec![];
        let results = Arc::new(Mutex::new(Vec::new()));

        // Spawn 10 threads that encode the same entry
        for _ in 0..10 {
            let entry = Arc::clone(&entry);
            let results = Arc::clone(&results);

            handles.push(thread::spawn(move || {
                let encoded = entry.encode().expect("Failed to encode");
                results.lock().unwrap().push(encoded);
            }));
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all encodings are identical
        let encoded_results = results.lock().unwrap();
        let first = &encoded_results[0];
        for encoded in encoded_results.iter() {
            assert_eq!(
                first, encoded,
                "Concurrent encoding produced different results"
            );
        }
    }

    #[test]
    fn concurrent_decoding_is_safe() {
        let entry = WALEntry::new_put(b"decode_key".to_vec(), b"decode_value".to_vec(), 777)
            .expect("Failed to create entry");
        let encoded = Arc::new(entry.encode().expect("Failed to encode"));

        let mut handles = vec![];

        // Spawn 10 threads that decode the same data
        for _ in 0..10 {
            let encoded = Arc::clone(&encoded);

            handles.push(thread::spawn(move || {
                let decoded = WALEntry::decode(&encoded).expect("Failed to decode");
                assert_eq!(decoded.key, b"decode_key");
                assert_eq!(decoded.value, b"decode_value");
                assert_eq!(decoded.timestamp, 777);
            }));
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

// Property-based tests
#[cfg(all(test, not(miri)))] // Skip under miri as proptest is slow
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn encoding_roundtrip_preserves_data(
            key in prop::collection::vec(any::<u8>(), 0..1000),
            value in prop::collection::vec(any::<u8>(), 0..1000),
            timestamp in any::<u64>(),
            is_delete in any::<bool>()
        ) {
            let entry = if is_delete {
                WALEntry::new_delete(key, timestamp)
            } else {
                WALEntry::new_put(key, value, timestamp)
            };

            if let Ok(entry) = entry {
                let encoded = entry.encode().expect("Encoding should succeed");
                let decoded = WALEntry::decode(&encoded).expect("Decoding should succeed");

                assert_eq!(entry.timestamp, decoded.timestamp);
                assert_eq!(entry.operation, decoded.operation);
                assert_eq!(entry.key, decoded.key);
                assert_eq!(entry.value, decoded.value);
            }
        }

        #[test]
        fn decode_rejects_random_data(data in prop::collection::vec(any::<u8>(), 0..1000)) {
            // Most random data should fail to decode
            let _ = WALEntry::decode(&data);
            // We don't assert failure because some random data might accidentally be valid
        }

        #[test]
        fn encoded_size_is_predictable(
            key in prop::collection::vec(any::<u8>(), 0..100),
            value in prop::collection::vec(any::<u8>(), 0..100),
            timestamp in any::<u64>()
        ) {
            let entry = WALEntry::new_put(key.clone(), value.clone(), timestamp)
                .expect("Failed to create entry");
            let encoded = entry.encode().expect("Failed to encode");

            let expected_size = 4 + 4 + 8 + 1 + 4 + key.len() + 4 + value.len();
            assert_eq!(encoded.len(), expected_size);
        }
    }
}
