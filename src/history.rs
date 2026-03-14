pub const HISTORY_LEN: usize = 120;

/// Fixed-capacity ring buffer storing u64 samples (oldest to newest).
pub struct RingBuffer {
    data: Vec<u64>,
    head: usize,
    filled: bool,
}

impl RingBuffer {
    pub fn new() -> Self {
        Self {
            data: vec![0; HISTORY_LEN],
            head: 0,
            filled: false,
        }
    }

    pub fn push(&mut self, value: u64) {
        self.data[self.head] = value;
        self.head = (self.head + 1) % HISTORY_LEN;
        if self.head == 0 {
            self.filled = true;
        }
    }

    /// Returns samples in chronological order (oldest → newest).
    pub fn as_ordered_vec(&self) -> Vec<u64> {
        if !self.filled {
            self.data[..self.head].to_vec()
        } else {
            let mut v = Vec::with_capacity(HISTORY_LEN);
            v.extend_from_slice(&self.data[self.head..]);
            v.extend_from_slice(&self.data[..self.head]);
            v
        }
    }
}

pub struct History {
    pub gpu_util: RingBuffer,
    pub gpu_power_w: RingBuffer,
    pub cpu_util: RingBuffer,
    pub vram_used_mib: RingBuffer,
    pub ram_used_gib_x10: RingBuffer, // GiB × 10 to keep u64 precision
}

impl History {
    pub fn new() -> Self {
        Self {
            gpu_util: RingBuffer::new(),
            gpu_power_w: RingBuffer::new(),
            cpu_util: RingBuffer::new(),
            vram_used_mib: RingBuffer::new(),
            ram_used_gib_x10: RingBuffer::new(),
        }
    }

    pub fn update(&mut self, snap: &crate::metrics::Snapshot) {
        self.gpu_util.push(snap.gpu.utilization_gpu_pct as u64);
        self.gpu_power_w
            .push((snap.gpu.power_draw_mw / 1000) as u64);
        self.cpu_util.push(snap.cpu.global_utilization_pct as u64);
        self.vram_used_mib
            .push(snap.gpu.vram_used_bytes / (1024 * 1024));
        self.ram_used_gib_x10
            .push(snap.ram.used_bytes * 10 / (1024 * 1024 * 1024));
    }
}
