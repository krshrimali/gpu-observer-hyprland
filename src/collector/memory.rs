use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use parking_lot::Mutex;
use sysinfo::{MemoryRefreshKind, System};

use crate::metrics::{RamMetrics, Snapshot};

pub async fn memory_collector(shared: Arc<Mutex<Snapshot>>, interval: Duration) -> Result<()> {
    let mut sys = System::new();

    loop {
        sys.refresh_memory_specifics(MemoryRefreshKind::everything());

        let total_bytes = sys.total_memory();
        let used_bytes = sys.used_memory();
        let available_bytes = sys.available_memory();
        let free_bytes = sys.free_memory();
        // cached ≈ total – used – free  (page cache + reclaimable slabs)
        let cached_approx_bytes = total_bytes
            .saturating_sub(used_bytes)
            .saturating_sub(free_bytes);

        {
            let mut snap = shared.lock();
            snap.ram = RamMetrics {
                total_bytes,
                used_bytes,
                available_bytes,
                free_bytes,
                cached_approx_bytes,
            };
        }

        tokio::time::sleep(interval).await;
    }
}
