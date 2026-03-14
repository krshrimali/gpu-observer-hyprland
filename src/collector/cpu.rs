use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use parking_lot::Mutex;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

use crate::metrics::{CpuMetrics, Snapshot};

pub async fn cpu_collector(shared: Arc<Mutex<Snapshot>>, interval: Duration) -> Result<()> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
    );

    // Warm-up: first sample initialises the usage delta baseline.
    sys.refresh_cpu_specifics(CpuRefreshKind::new().with_cpu_usage());
    tokio::time::sleep(Duration::from_millis(500)).await;

    loop {
        sys.refresh_cpu_specifics(CpuRefreshKind::everything());

        let brand = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_default();

        let per_core_utilization: Vec<f32> =
            sys.cpus().iter().map(|c| c.cpu_usage()).collect();

        let per_core_frequency_mhz: Vec<u64> =
            sys.cpus().iter().map(|c| c.frequency()).collect();

        let global_utilization_pct = sys.global_cpu_usage();

        {
            let mut snap = shared.lock();
            snap.cpu = CpuMetrics {
                global_utilization_pct,
                per_core_utilization,
                per_core_frequency_mhz,
                brand,
            };
        }

        tokio::time::sleep(interval).await;
    }
}
