/// Metrics for evaluating the performance of any given operation
/// 1. Memory Usage
/// 2. Timing
/// 3. Threads and Locks for performance optimization and learning how to tweak design
/// 4. 
/// -----
/// Saves to a JSON file in ../tmp/report
/// 

// src/metrics.rs

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    start_time: Instant,
    pub execution_time: Option<Duration>,
    pub operations_count: HashMap<String, usize>,
    pub errors_count: usize,
    pub memory_usage: Option<usize>,
    pub threads_spawned: usize,
    pub active_threads: AtomicUsize,
    pub locks_acquired: usize,
}

impl Report {
    // Initialize a new Report
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            execution_time: None,
            operations_count: HashMap::new(),
            errors_count: 0,
            memory_usage: None,
            threads_spawned: 0,
            active_threads: AtomicUsize::new(0),
            locks_acquired: 0,
        }
    }

    // Record the completion of an operation
    pub fn record_operation(&mut self, operation_name: &str) {
        let counter = self.operations_count.entry(operation_name.to_string()).or_insert(0);
        *counter += 1;
    }

    // Record an error occurrence
    pub fn record_error(&mut self) {
        self.errors_count += 1;
    }

    // Record a thread spawn
    pub fn record_thread_spawn(&mut self) {
        self.threads_spawned += 1;
        self.active_threads.fetch_add(1, Ordering::SeqCst);
    }

    // Record a thread termination
    pub fn record_thread_termination(&self) {
        self.active_threads.fetch_sub(1, Ordering::SeqCst);
    }

    // Record a lock acquisition
    pub fn record_lock_acquisition(&mut self) {
        self.locks_acquired += 1;
    }

    // Finalize the report, recording the total execution time
    pub fn finalize(&mut self) {
        self.execution_time = Some(self.start_time.elapsed());
        self.memory_usage = Some(self.calculate_memory_usage());
    }

    // Calculate memory usage (placeholder, would need actual implementation)
    fn calculate_memory_usage(&self) -> usize {
        // Placeholder value. Use actual memory profiling in a real scenario.
        1024 * 1024 // 1 MB
    }

    // Print the report summary to the console
    pub fn print_summary(&self) {
        println!("===== Metrics Report =====");
        if let Some(execution_time) = &self.execution_time {
            println!("Execution Time: {:.2?}", execution_time);
        }
        println!("Operations Count:");
        for (operation, count) in &self.operations_count {
            println!("  {}: {}", operation, count);
        }
        println!("Errors Encountered: {}", self.errors_count);
        if let Some(memory_usage) = &self.memory_usage {
            println!("Memory Usage: {} bytes", memory_usage);
        }
        println!("Threads Spawned: {}", self.threads_spawned);
        println!("Active Threads: {}", self.active_threads.load(Ordering::SeqCst));
        println!("Locks Acquired: {}", self.locks_acquired);
    }

    // Save the report as a JSON file in the ./tmp/report folder
    pub fn save_as_json(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Ensure the directory exists
        let path = Path::new("./tmp/report");
        fs::create_dir_all(&path)?;

        // Serialize the report to a JSON string
        let json = serde_json::to_string_pretty(&self)?;

        // Write the JSON string to a file
        let file_path: PathBuf = path.join("report.json");
        let mut file: File = File::create(file_path)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }
}