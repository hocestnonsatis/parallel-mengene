//! Memory usage monitoring and optimization utilities

use parallel_mengene_core::error::Result;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Memory usage statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_memory: u64,
    pub available_memory: u64,
    pub used_memory: u64,
    pub memory_percentage: f64,
    pub timestamp: Instant,
}

impl MemoryStats {
    pub fn new(total_memory: u64, available_memory: u64, used_memory: u64) -> Self {
        let memory_percentage = if total_memory > 0 {
            (used_memory as f64 / total_memory as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total_memory,
            available_memory,
            used_memory,
            memory_percentage,
            timestamp: Instant::now(),
        }
    }
}

/// Memory monitor for tracking system memory usage
pub struct MemoryMonitor {
    stats: Arc<Mutex<Option<MemoryStats>>>,
    update_interval: Duration,
    last_update: Arc<Mutex<Instant>>,
}

impl MemoryMonitor {
    /// Create a new memory monitor
    pub fn new() -> Self {
        Self {
            stats: Arc::new(Mutex::new(None)),
            update_interval: Duration::from_secs(1), // Update every second
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Create a new memory monitor with custom update interval
    pub fn with_update_interval(update_interval: Duration) -> Self {
        Self {
            stats: Arc::new(Mutex::new(None)),
            update_interval,
            last_update: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Get current memory statistics
    pub fn get_memory_stats(&self) -> Result<MemoryStats> {
        let mut last_update_guard = self.last_update.lock().unwrap();
        let now = Instant::now();

        // Check if we need to update the stats
        if now.duration_since(*last_update_guard) >= self.update_interval {
            let stats = self.read_memory_info()?;
            let mut stats_guard = self.stats.lock().unwrap();
            *stats_guard = Some(stats.clone());
            *last_update_guard = now;
            Ok(stats)
        } else {
            // Return cached stats
            let stats_guard = self.stats.lock().unwrap();
            match stats_guard.as_ref() {
                Some(stats) => Ok(stats.clone()),
                None => {
                    // No cached stats, read fresh
                    drop(stats_guard);
                    let stats = self.read_memory_info()?;
                    let mut stats_guard = self.stats.lock().unwrap();
                    *stats_guard = Some(stats.clone());
                    *last_update_guard = now;
                    Ok(stats)
                }
            }
        }
    }

    /// Read memory information from /proc/meminfo (Linux)
    fn read_memory_info(&self) -> Result<MemoryStats> {
        let meminfo = std::fs::read_to_string("/proc/meminfo")?;
        let mut total_memory = 0u64;
        let mut available_memory = 0u64;

        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                total_memory = self.parse_meminfo_line(line)?;
            } else if line.starts_with("MemAvailable:") {
                available_memory = self.parse_meminfo_line(line)?;
            }
        }

        let used_memory = total_memory.saturating_sub(available_memory);
        Ok(MemoryStats::new(
            total_memory,
            available_memory,
            used_memory,
        ))
    }

    /// Parse a line from /proc/meminfo
    fn parse_meminfo_line(&self, line: &str) -> Result<u64> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            parts[1]
                .parse::<u64>()
                .map(|kb| kb * 1024) // Convert KB to bytes
                .map_err(|e| {
                    parallel_mengene_core::error::Error::InvalidInput(format!(
                        "Failed to parse memory info: {}",
                        e
                    ))
                })
        } else {
            Err(parallel_mengene_core::error::Error::InvalidInput(
                "Invalid meminfo line format".to_string(),
            ))
        }
    }

    /// Check if memory usage is above a threshold
    pub fn is_memory_usage_high(&self, threshold_percentage: f64) -> Result<bool> {
        let stats = self.get_memory_stats()?;
        Ok(stats.memory_percentage > threshold_percentage)
    }

    /// Get recommended chunk size based on available memory
    pub fn get_recommended_chunk_size(&self, base_chunk_size: usize) -> Result<usize> {
        let stats = self.get_memory_stats()?;

        // If memory usage is high (>80%), reduce chunk size
        if stats.memory_percentage > 80.0 {
            Ok(base_chunk_size / 2)
        }
        // If memory usage is low (<50%), we can use larger chunks
        else if stats.memory_percentage < 50.0 && stats.available_memory > 2 * 1024 * 1024 * 1024
        {
            // Only increase if we have at least 2GB available
            Ok(base_chunk_size * 2)
        } else {
            Ok(base_chunk_size)
        }
    }

    /// Log current memory usage
    pub fn log_memory_usage(&self) -> Result<()> {
        let stats = self.get_memory_stats()?;
        tracing::info!(
            "Memory usage: {:.1}% ({:.1} GB / {:.1} GB used, {:.1} GB available)",
            stats.memory_percentage,
            stats.used_memory as f64 / 1_073_741_824.0,
            stats.total_memory as f64 / 1_073_741_824.0,
            stats.available_memory as f64 / 1_073_741_824.0
        );
        Ok(())
    }
}

impl Default for MemoryMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Memory usage warning thresholds
#[derive(Debug, Clone)]
pub struct MemoryThresholds {
    pub warning: f64,   // 70%
    pub critical: f64,  // 85%
    pub emergency: f64, // 95%
}

impl Default for MemoryThresholds {
    fn default() -> Self {
        Self {
            warning: 70.0,
            critical: 85.0,
            emergency: 95.0,
        }
    }
}

/// Memory usage tracker with automatic threshold monitoring
pub struct MemoryUsageTracker {
    monitor: MemoryMonitor,
    thresholds: MemoryThresholds,
    last_warning: Arc<Mutex<Option<Instant>>>,
}

impl MemoryUsageTracker {
    /// Create a new memory usage tracker
    pub fn new() -> Self {
        Self {
            monitor: MemoryMonitor::new(),
            thresholds: MemoryThresholds::default(),
            last_warning: Arc::new(Mutex::new(None)),
        }
    }

    /// Create a new memory usage tracker with custom thresholds
    pub fn with_thresholds(thresholds: MemoryThresholds) -> Self {
        Self {
            monitor: MemoryMonitor::new(),
            thresholds,
            last_warning: Arc::new(Mutex::new(None)),
        }
    }

    /// Check memory usage and log warnings if needed
    pub fn check_and_warn(&self) -> Result<()> {
        let stats = self.monitor.get_memory_stats()?;
        let now = Instant::now();

        // Check if we should issue a warning (avoid spam)
        let mut last_warning_guard = self.last_warning.lock().unwrap();
        let should_warn = match *last_warning_guard {
            Some(last) => now.duration_since(last) > Duration::from_secs(30),
            None => true,
        };

        if should_warn {
            if stats.memory_percentage >= self.thresholds.emergency {
                tracing::error!(
                    "EMERGENCY: Memory usage is at {:.1}%! Consider reducing chunk size or stopping other processes.",
                    stats.memory_percentage
                );
                *last_warning_guard = Some(now);
            } else if stats.memory_percentage >= self.thresholds.critical {
                tracing::warn!(
                    "CRITICAL: Memory usage is at {:.1}%! Performance may be affected.",
                    stats.memory_percentage
                );
                *last_warning_guard = Some(now);
            } else if stats.memory_percentage >= self.thresholds.warning {
                tracing::warn!(
                    "WARNING: Memory usage is at {:.1}%.",
                    stats.memory_percentage
                );
                *last_warning_guard = Some(now);
            }
        }

        Ok(())
    }

    /// Get memory monitor reference
    pub fn monitor(&self) -> &MemoryMonitor {
        &self.monitor
    }
}

impl Default for MemoryUsageTracker {
    fn default() -> Self {
        Self::new()
    }
}
