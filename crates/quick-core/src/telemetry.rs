use std::fs;
use std::time::Instant;

/// Diagnostic telemetry and performance metrics helper for Quick applications.
#[derive(Debug, Clone, Default)]
pub struct ProcessMetrics {
    pub rss_kb: u64,
    pub virtual_kb: u64,
}

impl ProcessMetrics {
    /// Reads resident set size (RSS) and virtual memory from Linux /proc/self/status.
    pub fn current() -> Self {
        let mut metrics = Self::default();
        if let Ok(status) = fs::read_to_string("/proc/self/status") {
            for line in status.lines() {
                if let Some(rss_str) = line.strip_prefix("VmRSS:") {
                    if let Some(kb) = rss_str.trim().strip_suffix("kB").and_then(|s| s.trim().parse::<u64>().ok()) {
                        metrics.rss_kb = kb;
                    }
                } else if let Some(vsize_str) = line.strip_prefix("VmSize:") {
                    if let Some(kb) = vsize_str.trim().strip_suffix("kB").and_then(|s| s.trim().parse::<u64>().ok()) {
                        metrics.virtual_kb = kb;
                    }
                }
            }
        }
        metrics
    }

    pub fn rss_mb(&self) -> f64 {
        self.rss_kb as f64 / 1024.0
    }
}

/// High-resolution benchmark timer
pub struct BenchmarkTimer {
    start: Instant,
    name: &'static str,
}

impl BenchmarkTimer {
    pub fn start(name: &'static str) -> Self {
        Self {
            start: Instant::now(),
            name,
        }
    }

    pub fn elapsed_ns(&self) -> u128 {
        self.start.elapsed().as_nanos()
    }

    pub fn elapsed_us(&self) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1000.0
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_nanos() as f64 / 1_000_000.0
    }

    pub fn report(&self) {
        println!("⏱️ [Benchmark] {}: {:.3} ms ({:.1} µs)", self.name, self.elapsed_ms(), self.elapsed_us());
    }
}
