use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};
use nvml_wrapper::enums::device::UsedGpuMemory;
use nvml_wrapper::Nvml;
use parking_lot::Mutex;

use crate::metrics::{GpuMetrics, GpuProcess, Snapshot};

pub async fn gpu_collector(shared: Arc<Mutex<Snapshot>>, interval: Duration) -> Result<()> {
    let nvml = Nvml::init()?;
    let device = nvml.device_by_index(0)?;

    loop {
        let name = device
            .name()
            .unwrap_or_else(|_| "Unknown GPU".to_string());

        let temperature_c = device
            .temperature(TemperatureSensor::Gpu)
            .unwrap_or(0);

        let fan_speed_pct = device.fan_speed(0).unwrap_or(0);

        let utilization = device.utilization_rates().ok();

        let clock_graphics_mhz = device.clock_info(Clock::Graphics).unwrap_or(0);
        let clock_sm_mhz = device.clock_info(Clock::SM).unwrap_or(0);
        let clock_memory_mhz = device.clock_info(Clock::Memory).unwrap_or(0);

        let power_draw_mw = device.power_usage().unwrap_or(0);
        let power_limit_mw = device.enforced_power_limit().unwrap_or(0);

        let (vram_used_bytes, vram_free_bytes, vram_total_bytes) =
            match device.memory_info() {
                Ok(m) => (m.used, m.free, m.total),
                Err(_) => (0, 0, 0),
            };

        let processes = device
            .running_graphics_processes()
            .unwrap_or_default()
            .into_iter()
            .map(|p| {
                let vram_bytes = match p.used_gpu_memory {
                    UsedGpuMemory::Used(b) => b,
                    UsedGpuMemory::Unavailable => 0,
                };
                let proc_name = std::fs::read_to_string(format!("/proc/{}/comm", p.pid))
                    .unwrap_or_else(|_| format!("pid:{}", p.pid))
                    .trim()
                    .to_string();
                GpuProcess {
                    pid: p.pid,
                    name: proc_name,
                    vram_bytes,
                }
            })
            .collect();

        let metrics = GpuMetrics {
            name,
            temperature_c,
            fan_speed_pct,
            utilization_gpu_pct: utilization.as_ref().map(|u| u.gpu).unwrap_or(0),
            utilization_mem_pct: utilization.as_ref().map(|u| u.memory).unwrap_or(0),
            clock_graphics_mhz,
            clock_sm_mhz,
            clock_memory_mhz,
            power_draw_mw,
            power_limit_mw,
            vram_used_bytes,
            vram_free_bytes,
            vram_total_bytes,
            processes,
        };

        {
            let mut snap = shared.lock();
            snap.gpu = metrics;
        }

        tokio::time::sleep(interval).await;
    }
}
