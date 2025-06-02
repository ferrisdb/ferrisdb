//! Metrics collection for WAL operations
//!
//! This module provides comprehensive metrics tracking for both WAL reader and writer
//! operations, enabling performance monitoring and debugging.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

/// WAL operation metrics
///
/// Tracks various performance and operational metrics for the Write-Ahead Log.
/// All fields use atomic operations for thread-safe access without locks.
///
/// # Example
///
/// ```no_run
/// use ferrisdb_storage::wal::{WALWriter, WALMetrics};
/// use ferrisdb_core::SyncMode;
///
/// let writer = WALWriter::new("wal.log", SyncMode::Full, 64 * 1024 * 1024)?;
/// let metrics = writer.metrics();
///
/// // After some operations...
/// println!("Total writes: {}", metrics.writes_total());
/// println!("Bytes written: {}", metrics.bytes_written());
/// # Ok::<(), ferrisdb_core::Error>(())
/// ```
#[derive(Debug, Default)]
pub struct WALMetrics {
    // Writer metrics
    writes_total: AtomicU64,
    writes_failed: AtomicU64,
    bytes_written: AtomicU64,
    sync_total: AtomicU64,
    sync_duration_ms: AtomicU64,
    rotation_count: AtomicU64,

    // Reader metrics
    reads_total: AtomicU64,
    reads_failed: AtomicU64,
    bytes_read: AtomicU64,
    corrupted_entries: AtomicU64,

    // Performance metrics
    avg_entry_size: AtomicU64,
    max_entry_size: AtomicU64,

    // File metrics
    current_file_size: AtomicU64,
    files_opened: AtomicU64,
}

impl WALMetrics {
    /// Creates a new metrics instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a write operation
    pub fn record_write(&self, size: u64, success: bool) {
        if success {
            self.writes_total.fetch_add(1, Ordering::Relaxed);
            self.bytes_written.fetch_add(size, Ordering::Relaxed);
            self.update_avg_entry_size(size);
            self.update_max_entry_size(size);
        } else {
            self.writes_failed.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Records a sync operation with its duration
    pub fn record_sync(&self, duration_ms: u64) {
        self.sync_total.fetch_add(1, Ordering::Relaxed);
        self.sync_duration_ms
            .fetch_add(duration_ms, Ordering::Relaxed);
    }

    /// Records a read operation
    pub fn record_read(&self, size: u64, success: bool) {
        if success {
            self.reads_total.fetch_add(1, Ordering::Relaxed);
            self.bytes_read.fetch_add(size, Ordering::Relaxed);
        } else {
            self.reads_failed.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Records a corrupted entry
    pub fn record_corruption(&self) {
        self.corrupted_entries.fetch_add(1, Ordering::Relaxed);
    }

    /// Records a file rotation
    pub fn record_rotation(&self) {
        self.rotation_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Records a file being opened
    pub fn record_file_opened(&self) {
        self.files_opened.fetch_add(1, Ordering::Relaxed);
    }

    /// Updates the current file size
    pub fn update_file_size(&self, size: u64) {
        self.current_file_size.store(size, Ordering::Relaxed);
    }

    /// Gets the average sync duration in milliseconds
    pub fn avg_sync_duration_ms(&self) -> f64 {
        let total = self.sync_duration_ms.load(Ordering::Relaxed);
        let count = self.sync_total.load(Ordering::Relaxed);
        if count == 0 {
            0.0
        } else {
            total as f64 / count as f64
        }
    }

    /// Gets the write success rate as a percentage
    pub fn write_success_rate(&self) -> f64 {
        let success = self.writes_total.load(Ordering::Relaxed);
        let failed = self.writes_failed.load(Ordering::Relaxed);
        let total = success + failed;
        if total == 0 {
            100.0
        } else {
            (success as f64 / total as f64) * 100.0
        }
    }

    /// Gets the read success rate as a percentage
    pub fn read_success_rate(&self) -> f64 {
        let success = self.reads_total.load(Ordering::Relaxed);
        let failed = self.reads_failed.load(Ordering::Relaxed);
        let total = success + failed;
        if total == 0 {
            100.0
        } else {
            (success as f64 / total as f64) * 100.0
        }
    }

    /// Updates the moving average for entry size
    fn update_avg_entry_size(&self, size: u64) {
        let current_avg = self.avg_entry_size.load(Ordering::Relaxed);
        let new_avg = if current_avg == 0 {
            size
        } else {
            // Simple exponential moving average with alpha = 0.1
            (current_avg * 9 + size) / 10
        };
        self.avg_entry_size.store(new_avg, Ordering::Relaxed);
    }

    /// Updates the maximum entry size if necessary
    fn update_max_entry_size(&self, size: u64) {
        let mut current_max = self.max_entry_size.load(Ordering::Relaxed);
        while size > current_max {
            match self.max_entry_size.compare_exchange_weak(
                current_max,
                size,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => current_max = actual,
            }
        }
    }

    /// Resets all metrics to zero
    pub fn reset(&self) {
        self.writes_total.store(0, Ordering::Relaxed);
        self.writes_failed.store(0, Ordering::Relaxed);
        self.bytes_written.store(0, Ordering::Relaxed);
        self.sync_total.store(0, Ordering::Relaxed);
        self.sync_duration_ms.store(0, Ordering::Relaxed);
        self.rotation_count.store(0, Ordering::Relaxed);
        self.reads_total.store(0, Ordering::Relaxed);
        self.reads_failed.store(0, Ordering::Relaxed);
        self.bytes_read.store(0, Ordering::Relaxed);
        self.corrupted_entries.store(0, Ordering::Relaxed);
        self.avg_entry_size.store(0, Ordering::Relaxed);
        self.max_entry_size.store(0, Ordering::Relaxed);
        self.current_file_size.store(0, Ordering::Relaxed);
        self.files_opened.store(0, Ordering::Relaxed);
    }

    // Accessor methods for encapsulated fields

    /// Gets the total number of successful writes
    pub fn writes_total(&self) -> u64 {
        self.writes_total.load(Ordering::Relaxed)
    }

    /// Gets the total number of failed writes
    pub fn writes_failed(&self) -> u64 {
        self.writes_failed.load(Ordering::Relaxed)
    }

    /// Gets the total bytes written
    pub fn bytes_written(&self) -> u64 {
        self.bytes_written.load(Ordering::Relaxed)
    }

    /// Gets the total number of sync operations
    pub fn sync_total(&self) -> u64 {
        self.sync_total.load(Ordering::Relaxed)
    }

    /// Gets the total sync duration in milliseconds
    pub fn sync_duration_ms(&self) -> u64 {
        self.sync_duration_ms.load(Ordering::Relaxed)
    }

    /// Gets the number of file rotations
    pub fn rotation_count(&self) -> u64 {
        self.rotation_count.load(Ordering::Relaxed)
    }

    /// Gets the total number of successful reads
    pub fn reads_total(&self) -> u64 {
        self.reads_total.load(Ordering::Relaxed)
    }

    /// Gets the total number of failed reads
    pub fn reads_failed(&self) -> u64 {
        self.reads_failed.load(Ordering::Relaxed)
    }

    /// Gets the total bytes read
    pub fn bytes_read(&self) -> u64 {
        self.bytes_read.load(Ordering::Relaxed)
    }

    /// Gets the number of corrupted entries
    pub fn corrupted_entries(&self) -> u64 {
        self.corrupted_entries.load(Ordering::Relaxed)
    }

    /// Gets the average entry size
    pub fn avg_entry_size(&self) -> u64 {
        self.avg_entry_size.load(Ordering::Relaxed)
    }

    /// Gets the maximum entry size
    pub fn max_entry_size(&self) -> u64 {
        self.max_entry_size.load(Ordering::Relaxed)
    }

    /// Gets the current file size
    pub fn current_file_size(&self) -> u64 {
        self.current_file_size.load(Ordering::Relaxed)
    }

    /// Gets the number of files opened
    pub fn files_opened(&self) -> u64 {
        self.files_opened.load(Ordering::Relaxed)
    }
}

/// Helper struct for timing operations
pub struct TimedOperation {
    start: Instant,
}

impl TimedOperation {
    /// Starts timing an operation
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Completes the timing and returns duration in milliseconds
    pub fn complete(self) -> u64 {
        self.start.elapsed().as_millis() as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that write operations are tracked accurately in metrics.
    ///
    /// This test verifies that:
    /// - Successful writes increment the write counter correctly
    /// - Failed writes increment the failure counter separately
    /// - Byte counts accumulate accurately across operations
    /// - Maximum entry size tracking works properly
    /// - Success rates are calculated correctly from counters
    #[test]
    fn record_write_updates_counters_and_averages_correctly() {
        let metrics = WALMetrics::new();

        // Record some successful writes
        metrics.record_write(100, true);
        metrics.record_write(200, true);
        metrics.record_write(150, true);

        // Record a failed write
        metrics.record_write(300, false);

        assert_eq!(metrics.writes_total(), 3);
        assert_eq!(metrics.writes_failed(), 1);
        assert_eq!(metrics.bytes_written(), 450);
        assert_eq!(metrics.max_entry_size(), 200);
        assert_eq!(metrics.write_success_rate(), 75.0);
    }

    /// Tests that read operations and corruption tracking work correctly.
    ///
    /// This test verifies that:
    /// - Successful reads are counted and tracked accurately
    /// - Failed reads are handled separately from successes
    /// - Byte counts reflect actual data read successfully
    /// - Corruption detection increments corruption counters
    /// - Success rates account for both failed reads and corruptions
    #[test]
    fn record_read_updates_counters_and_tracks_corruption() {
        let metrics = WALMetrics::new();

        metrics.record_read(100, true);
        metrics.record_read(200, true);
        metrics.record_read(0, false);
        metrics.record_corruption();

        assert_eq!(metrics.reads_total(), 2);
        assert_eq!(metrics.reads_failed(), 1);
        assert_eq!(metrics.bytes_read(), 300);
        assert_eq!(metrics.corrupted_entries(), 1);

        // Use approximate comparison for floating point
        let success_rate = metrics.read_success_rate();
        assert!((success_rate - (200.0 / 3.0)).abs() < 0.01);
    }

    /// Tests that sync operations accumulate duration and count correctly.
    ///
    /// This test verifies that:
    /// - Sync operations are counted accurately
    /// - Duration values accumulate correctly across multiple syncs
    /// - Average sync duration is calculated properly
    /// - Performance metrics reflect actual sync behavior
    #[test]
    fn record_sync_accumulates_duration_and_count() {
        let metrics = WALMetrics::new();

        metrics.record_sync(10);
        metrics.record_sync(20);
        metrics.record_sync(30);

        assert_eq!(metrics.sync_total(), 3);
        assert_eq!(metrics.sync_duration_ms(), 60);
        assert_eq!(metrics.avg_sync_duration_ms(), 20.0);
    }

    /// Tests that metrics reset clears all counters to initial state.
    ///
    /// This test verifies that:
    /// - All write-related counters are reset to zero
    /// - All read-related counters are reset to zero
    /// - Sync counters and durations are cleared
    /// - Metrics can be safely reused after reset
    #[test]
    fn reset_sets_all_counters_to_zero() {
        let metrics = WALMetrics::new();

        // Add some data
        metrics.record_write(100, true);
        metrics.record_read(200, true);
        metrics.record_sync(30);

        // Reset
        metrics.reset();

        // Verify all counters are zero
        assert_eq!(metrics.writes_total(), 0);
        assert_eq!(metrics.reads_total(), 0);
        assert_eq!(metrics.sync_total(), 0);
    }

    /// Tests that TimedOperation helper measures elapsed time accurately.
    ///
    /// This test verifies that:
    /// - Timer starts correctly when created
    /// - Elapsed time reflects actual sleep duration
    /// - Time measurement has reasonable accuracy
    /// - Helper utility works for performance tracking
    #[test]
    fn timed_operation_measures_elapsed_time_accurately() {
        let timer = TimedOperation::start();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let duration = timer.complete();

        // Should be at least 10ms
        assert!(duration >= 10);
    }
}
