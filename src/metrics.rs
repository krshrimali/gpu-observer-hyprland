/// All raw metrics collected from hardware.
/// Units: bytes for memory, milliwatts for power, MHz for clocks.

#[derive(Clone, Debug, Default)]
pub struct GpuMetrics {
    pub name: String,
    pub temperature_c: u32,
    pub fan_speed_pct: u32,
    pub utilization_gpu_pct: u32,
    pub utilization_mem_pct: u32,
    pub clock_graphics_mhz: u32,
    #[allow(dead_code)]
    pub clock_sm_mhz: u32,
    pub clock_memory_mhz: u32,
    /// Raw milliwatts from NVML
    pub power_draw_mw: u32,
    /// Raw milliwatts from NVML
    pub power_limit_mw: u32,
    pub vram_used_bytes: u64,
    pub vram_free_bytes: u64,
    pub vram_total_bytes: u64,
    pub processes: Vec<GpuProcess>,
}

#[derive(Clone, Debug)]
pub struct GpuProcess {
    pub pid: u32,
    pub name: String,
    pub vram_bytes: u64,
}

#[derive(Clone, Debug, Default)]
pub struct CpuMetrics {
    pub global_utilization_pct: f32,
    pub per_core_utilization: Vec<f32>,
    pub per_core_frequency_mhz: Vec<u64>,
    pub brand: String,
}

#[derive(Clone, Debug, Default)]
pub struct RamMetrics {
    /// All values in bytes (as returned by sysinfo)
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub free_bytes: u64,
    /// Approximation: total - used - free (page cache + reclaimable)
    pub cached_approx_bytes: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub gpu: GpuMetrics,
    pub cpu: CpuMetrics,
    pub ram: RamMetrics,
}
