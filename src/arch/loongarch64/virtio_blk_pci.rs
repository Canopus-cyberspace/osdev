#![allow(dead_code)]

use core::sync::atomic::{compiler_fence, Ordering};

const PCI_ECAM_BASE: usize = 0x2000_0000;
const PCI_IO_CPU_BASE: usize = 0x1800_4000;
const PCI_IO_BASE: u32 = 0x4000;

const PCI_VENDOR_VIRTIO: u16 = 0x1af4;
const PCI_DEVICE_VIRTIO_BLK_LEGACY: u16 = 0x1001;

const PCI_VENDOR_ID: usize = 0x00;
const PCI_DEVICE_ID: usize = 0x02;
const PCI_COMMAND: usize = 0x04;
const PCI_STATUS: usize = 0x06;
const PCI_BAR0: usize = 0x10;
const PCI_BAR1: usize = 0x14;
const PCI_CAP_PTR: usize = 0x34;
const PCI_BAR4: usize = 0x20;
const PCI_BAR5: usize = 0x24;

const PCI_COMMAND_IO: u16 = 1 << 0;
const PCI_COMMAND_MEMORY: u16 = 1 << 1;
const PCI_COMMAND_BUS_MASTER: u16 = 1 << 2;
const PCI_STATUS_CAP_LIST: u16 = 1 << 4;
const PCI_CAP_ID_VENDOR: u8 = 0x09;

const PCI_MEM_BAR1_BASE: u32 = 0x4000_0000;
const PCI_MEM_BAR4_BASE: u32 = 0x4001_0000;

const VIRTIO_PCI_DEVICE_FEATURES: usize = 0x00;
const VIRTIO_PCI_DRIVER_FEATURES: usize = 0x04;
const VIRTIO_PCI_QUEUE_PFN: usize = 0x08;
const VIRTIO_PCI_QUEUE_NUM: usize = 0x0c;
const VIRTIO_PCI_QUEUE_SEL: usize = 0x0e;
const VIRTIO_PCI_QUEUE_NOTIFY: usize = 0x10;
const VIRTIO_PCI_STATUS: usize = 0x12;
const VIRTIO_PCI_ISR: usize = 0x13;

const STATUS_ACKNOWLEDGE: u8 = 1;
const STATUS_DRIVER: u8 = 2;
const STATUS_DRIVER_OK: u8 = 4;
const STATUS_FEATURES_OK: u8 = 8;

const VIRTIO_PCI_CAP_COMMON_CFG: u8 = 1;
const VIRTIO_PCI_CAP_NOTIFY_CFG: u8 = 2;
const VIRTIO_PCI_CAP_ISR_CFG: u8 = 3;
const VIRTIO_F_VERSION_1_HIGH: u32 = 1;

const COMMON_DEVICE_FEATURE_SELECT: usize = 0;
const COMMON_DEVICE_FEATURE: usize = 4;
const COMMON_DRIVER_FEATURE_SELECT: usize = 8;
const COMMON_DRIVER_FEATURE: usize = 12;
const COMMON_DEVICE_STATUS: usize = 20;
const COMMON_QUEUE_SELECT: usize = 22;
const COMMON_QUEUE_SIZE: usize = 24;
const COMMON_QUEUE_ENABLE: usize = 28;
const COMMON_QUEUE_NOTIFY_OFF: usize = 30;
const COMMON_QUEUE_DESC: usize = 32;
const COMMON_QUEUE_DRIVER: usize = 40;
const COMMON_QUEUE_DEVICE: usize = 48;

pub const SECTOR_SIZE: usize = 512;
const QUEUE_SIZE: usize = 8;
const LEGACY_QUEUE_BYTES: usize = 8192;
const LEGACY_USED_OFFSET: usize = 4096;
const VRING_DESC_F_NEXT: u16 = 1;
const VRING_DESC_F_WRITE: u16 = 2;
const VIRTIO_BLK_T_IN: u32 = 0;

#[repr(C)]
#[derive(Copy, Clone)]
struct VirtqDesc {
    addr: u64,
    len: u32,
    flags: u16,
    next: u16,
}

impl VirtqDesc {
    const fn empty() -> Self {
        Self {
            addr: 0,
            len: 0,
            flags: 0,
            next: 0,
        }
    }
}

#[repr(C)]
struct VirtqAvail {
    flags: u16,
    idx: u16,
    ring: [u16; QUEUE_SIZE],
    used_event: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VirtqUsedElem {
    id: u32,
    len: u32,
}

impl VirtqUsedElem {
    const fn empty() -> Self {
        Self { id: 0, len: 0 }
    }
}

#[repr(C)]
struct VirtqUsed {
    flags: u16,
    idx: u16,
    ring: [VirtqUsedElem; QUEUE_SIZE],
    avail_event: u16,
}

#[repr(C, align(16))]
struct VirtioBlkReq {
    req_type: u32,
    reserved: u32,
    sector: u64,
}

#[repr(C, align(4096))]
struct LegacyQueue {
    bytes: [u8; LEGACY_QUEUE_BYTES],
}

static mut QUEUE: LegacyQueue = LegacyQueue {
    bytes: [0; LEGACY_QUEUE_BYTES],
};
static mut REQUEST: VirtioBlkReq = VirtioBlkReq {
    req_type: 0,
    reserved: 0,
    sector: 0,
};
static mut DATA: [u8; SECTOR_SIZE] = [0; SECTOR_SIZE];
static mut REQ_STATUS: u8 = 0xff;
static mut IO_BASE: usize = 0;
static mut MODERN_COMMON: usize = 0;
static mut MODERN_NOTIFY: usize = 0;
static mut MODERN_ISR: usize = 0;
static mut MODERN_MODE: bool = false;
static mut READY: bool = false;
static mut LAST_USED_IDX: u16 = 0;

pub fn init() -> Result<(), &'static str> {
    unsafe {
        if READY {
            return Ok(());
        }
        let cfg = find_legacy_block_device().ok_or("pci_virtio_blk_not_found")?;
        if init_modern(cfg).is_ok() {
            READY = true;
            MODERN_MODE = true;
            return Ok(());
        }
        MODERN_MODE = false;
        config_write32(cfg, PCI_BAR0, PCI_IO_BASE | 1);
        config_write16(
            cfg,
            PCI_COMMAND,
            PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_BUS_MASTER,
        );
        let bar0 = config_read32(cfg, PCI_BAR0) & !0x3;
        if bar0 < PCI_IO_BASE {
            return Err("pci_bar0_io_base");
        }
        IO_BASE = PCI_IO_CPU_BASE + (bar0 - PCI_IO_BASE) as usize;

        io_write8(VIRTIO_PCI_STATUS, 0);
        io_write8(VIRTIO_PCI_STATUS, STATUS_ACKNOWLEDGE);
        io_write8(VIRTIO_PCI_STATUS, STATUS_ACKNOWLEDGE | STATUS_DRIVER);
        let _features = io_read32(VIRTIO_PCI_DEVICE_FEATURES);
        io_write32(VIRTIO_PCI_DRIVER_FEATURES, 0);

        io_write16(VIRTIO_PCI_QUEUE_SEL, 0);
        let queue_num = io_read16(VIRTIO_PCI_QUEUE_NUM) as usize;
        if queue_num < QUEUE_SIZE || queue_num == 0 {
            return Err("virtio_pci_queue_size");
        }
        clear_queue();
        LAST_USED_IDX = 0;
        let queue_pfn = (core::ptr::addr_of!(QUEUE) as usize >> 12) as u32;
        io_write32(VIRTIO_PCI_QUEUE_PFN, queue_pfn);
        compiler_fence(Ordering::SeqCst);
        io_write8(
            VIRTIO_PCI_STATUS,
            STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_DRIVER_OK,
        );
        READY = true;
    }
    Ok(())
}

pub fn read_sector(sector: u64, out: &mut [u8; SECTOR_SIZE]) -> Result<(), &'static str> {
    init()?;
    unsafe {
        REQUEST = VirtioBlkReq {
            req_type: VIRTIO_BLK_T_IN,
            reserved: 0,
            sector,
        };
        clear_data();
        REQ_STATUS = 0xff;

        let desc = desc_ptr();
        let avail = avail_ptr();
        let used = used_ptr();

        core::ptr::write(
            desc.add(0),
            VirtqDesc {
                addr: core::ptr::addr_of!(REQUEST) as u64,
                len: core::mem::size_of::<VirtioBlkReq>() as u32,
                flags: VRING_DESC_F_NEXT,
                next: 1,
            },
        );
        core::ptr::write(
            desc.add(1),
            VirtqDesc {
                addr: core::ptr::addr_of!(DATA) as u64,
                len: SECTOR_SIZE as u32,
                flags: VRING_DESC_F_NEXT | VRING_DESC_F_WRITE,
                next: 2,
            },
        );
        core::ptr::write(
            desc.add(2),
            VirtqDesc {
                addr: core::ptr::addr_of!(REQ_STATUS) as u64,
                len: 1,
                flags: VRING_DESC_F_WRITE,
                next: 0,
            },
        );

        let avail_idx = core::ptr::read_volatile(core::ptr::addr_of!((*avail).idx));
        (*avail).ring[avail_idx as usize % QUEUE_SIZE] = 0;
        compiler_fence(Ordering::SeqCst);
        core::ptr::write_volatile(
            core::ptr::addr_of_mut!((*avail).idx),
            avail_idx.wrapping_add(1),
        );
        compiler_fence(Ordering::SeqCst);
        if MODERN_MODE {
            modern_write16(MODERN_NOTIFY, 0);
        } else {
            io_write16(VIRTIO_PCI_QUEUE_NOTIFY, 0);
        }

        let start = LAST_USED_IDX;
        let mut spin = 0usize;
        while core::ptr::read_volatile(core::ptr::addr_of!((*used).idx)) == start
            && spin < 50_000_000
        {
            spin += 1;
        }
        let used_idx = core::ptr::read_volatile(core::ptr::addr_of!((*used).idx));
        if used_idx == start {
            return Err("virtio_pci_timeout");
        }
        LAST_USED_IDX = used_idx;
        let isr = if MODERN_MODE {
            modern_read8(MODERN_ISR)
        } else {
            io_read8(VIRTIO_PCI_ISR)
        };
        let _ = isr;
        if core::ptr::read_volatile(core::ptr::addr_of!(REQ_STATUS)) != 0 {
            return Err("virtio_pci_status");
        }

        let mut i = 0usize;
        while i < SECTOR_SIZE {
            out[i] = core::ptr::read_volatile(core::ptr::addr_of!(DATA[i]));
            i += 1;
        }
    }
    Ok(())
}

unsafe fn init_modern(cfg: usize) -> Result<(), &'static str> {
    config_write32(cfg, PCI_BAR1, PCI_MEM_BAR1_BASE);
    config_write32(cfg, PCI_BAR4, PCI_MEM_BAR4_BASE);
    config_write32(cfg, PCI_BAR5, 0);
    config_write16(
        cfg,
        PCI_COMMAND,
        PCI_COMMAND_IO | PCI_COMMAND_MEMORY | PCI_COMMAND_BUS_MASTER,
    );

    let bar1 = (config_read32(cfg, PCI_BAR1) & !0xf) as usize;
    let bar4 = (config_read32(cfg, PCI_BAR4) & !0xf) as usize;
    let mut common = 0usize;
    let mut notify = 0usize;
    let mut notify_mult = 0u32;
    let mut isr = 0usize;
    if (config_read16(cfg, PCI_STATUS) & PCI_STATUS_CAP_LIST) == 0 {
        return Err("virtio_modern_no_caps");
    }
    let mut cap = (config_read8(cfg, PCI_CAP_PTR) & !0x3) as usize;
    let mut guard = 0usize;
    while cap != 0 && guard < 32 {
        if config_read8(cfg, cap) == PCI_CAP_ID_VENDOR {
            let cfg_type = config_read8(cfg, cap + 3);
            let bar = config_read8(cfg, cap + 4);
            let offset = config_read32(cfg, cap + 8) as usize;
            let base = match bar {
                1 => bar1,
                4 => bar4,
                _ => 0,
            };
            if base != 0 {
                match cfg_type {
                    VIRTIO_PCI_CAP_COMMON_CFG => common = base + offset,
                    VIRTIO_PCI_CAP_NOTIFY_CFG => {
                        notify = base + offset;
                        notify_mult = config_read32(cfg, cap + 16);
                    }
                    VIRTIO_PCI_CAP_ISR_CFG => isr = base + offset,
                    _ => {}
                }
            }
        }
        cap = (config_read8(cfg, cap + 1) & !0x3) as usize;
        guard += 1;
    }
    if common == 0 || notify == 0 || isr == 0 {
        return Err("virtio_modern_caps");
    }

    modern_write8(common + COMMON_DEVICE_STATUS, 0);
    modern_write8(common + COMMON_DEVICE_STATUS, STATUS_ACKNOWLEDGE);
    modern_write8(
        common + COMMON_DEVICE_STATUS,
        STATUS_ACKNOWLEDGE | STATUS_DRIVER,
    );
    modern_write32(common + COMMON_DEVICE_FEATURE_SELECT, 1);
    if (modern_read32(common + COMMON_DEVICE_FEATURE) & VIRTIO_F_VERSION_1_HIGH) == 0 {
        return Err("virtio_modern_version");
    }
    modern_write32(common + COMMON_DRIVER_FEATURE_SELECT, 0);
    modern_write32(common + COMMON_DRIVER_FEATURE, 0);
    modern_write32(common + COMMON_DRIVER_FEATURE_SELECT, 1);
    modern_write32(common + COMMON_DRIVER_FEATURE, VIRTIO_F_VERSION_1_HIGH);
    modern_write8(
        common + COMMON_DEVICE_STATUS,
        STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_FEATURES_OK,
    );
    if (modern_read8(common + COMMON_DEVICE_STATUS) & STATUS_FEATURES_OK) == 0 {
        return Err("virtio_modern_features_ok");
    }

    modern_write16(common + COMMON_QUEUE_SELECT, 0);
    let queue_num = modern_read16(common + COMMON_QUEUE_SIZE) as usize;
    if queue_num < QUEUE_SIZE || queue_num == 0 {
        return Err("virtio_modern_queue_size");
    }
    clear_queue();
    LAST_USED_IDX = 0;
    modern_write16(common + COMMON_QUEUE_SIZE, QUEUE_SIZE as u16);
    modern_write64(common + COMMON_QUEUE_DESC, desc_ptr() as u64);
    modern_write64(common + COMMON_QUEUE_DRIVER, avail_ptr() as u64);
    modern_write64(common + COMMON_QUEUE_DEVICE, used_ptr() as u64);
    let notify_off = modern_read16(common + COMMON_QUEUE_NOTIFY_OFF) as u32;
    MODERN_COMMON = common;
    MODERN_NOTIFY = notify + notify_off.wrapping_mul(notify_mult) as usize;
    MODERN_ISR = isr;
    modern_write16(common + COMMON_QUEUE_ENABLE, 1);
    compiler_fence(Ordering::SeqCst);
    modern_write8(
        common + COMMON_DEVICE_STATUS,
        STATUS_ACKNOWLEDGE | STATUS_DRIVER | STATUS_FEATURES_OK | STATUS_DRIVER_OK,
    );
    Ok(())
}

unsafe fn find_legacy_block_device() -> Option<usize> {
    let mut dev = 0usize;
    while dev < 32 {
        let cfg = PCI_ECAM_BASE + (dev << 15);
        if config_read16(cfg, PCI_VENDOR_ID) == PCI_VENDOR_VIRTIO
            && config_read16(cfg, PCI_DEVICE_ID) == PCI_DEVICE_VIRTIO_BLK_LEGACY
        {
            return Some(cfg);
        }
        dev += 1;
    }
    None
}

unsafe fn clear_queue() {
    let base = core::ptr::addr_of_mut!(QUEUE.bytes) as *mut u8;
    let mut i = 0usize;
    while i < LEGACY_QUEUE_BYTES {
        core::ptr::write_volatile(base.add(i), 0);
        i += 1;
    }
}

unsafe fn clear_data() {
    let base = core::ptr::addr_of_mut!(DATA) as *mut u8;
    let mut i = 0usize;
    while i < SECTOR_SIZE {
        core::ptr::write_volatile(base.add(i), 0);
        i += 1;
    }
}

unsafe fn desc_ptr() -> *mut VirtqDesc {
    core::ptr::addr_of_mut!(QUEUE.bytes) as *mut VirtqDesc
}

unsafe fn avail_ptr() -> *mut VirtqAvail {
    (core::ptr::addr_of_mut!(QUEUE.bytes) as *mut u8)
        .add(QUEUE_SIZE * core::mem::size_of::<VirtqDesc>()) as *mut VirtqAvail
}

unsafe fn used_ptr() -> *mut VirtqUsed {
    (core::ptr::addr_of_mut!(QUEUE.bytes) as *mut u8).add(LEGACY_USED_OFFSET) as *mut VirtqUsed
}

unsafe fn config_read16(cfg: usize, off: usize) -> u16 {
    core::ptr::read_volatile((cfg + off) as *const u16)
}

unsafe fn config_read8(cfg: usize, off: usize) -> u8 {
    core::ptr::read_volatile((cfg + off) as *const u8)
}

unsafe fn config_read32(cfg: usize, off: usize) -> u32 {
    core::ptr::read_volatile((cfg + off) as *const u32)
}

unsafe fn config_write16(cfg: usize, off: usize, value: u16) {
    core::ptr::write_volatile((cfg + off) as *mut u16, value);
}

unsafe fn config_write32(cfg: usize, off: usize, value: u32) {
    core::ptr::write_volatile((cfg + off) as *mut u32, value);
}

unsafe fn io_read8(off: usize) -> u8 {
    core::ptr::read_volatile((IO_BASE + off) as *const u8)
}

unsafe fn io_read16(off: usize) -> u16 {
    core::ptr::read_volatile((IO_BASE + off) as *const u16)
}

unsafe fn io_read32(off: usize) -> u32 {
    core::ptr::read_volatile((IO_BASE + off) as *const u32)
}

unsafe fn io_write8(off: usize, value: u8) {
    core::ptr::write_volatile((IO_BASE + off) as *mut u8, value);
}

unsafe fn io_write16(off: usize, value: u16) {
    core::ptr::write_volatile((IO_BASE + off) as *mut u16, value);
}

unsafe fn io_write32(off: usize, value: u32) {
    core::ptr::write_volatile((IO_BASE + off) as *mut u32, value);
}

unsafe fn modern_read8(addr: usize) -> u8 {
    core::ptr::read_volatile(addr as *const u8)
}

unsafe fn modern_read16(addr: usize) -> u16 {
    core::ptr::read_volatile(addr as *const u16)
}

unsafe fn modern_read32(addr: usize) -> u32 {
    core::ptr::read_volatile(addr as *const u32)
}

unsafe fn modern_write8(addr: usize, value: u8) {
    core::ptr::write_volatile(addr as *mut u8, value);
}

unsafe fn modern_write16(addr: usize, value: u16) {
    core::ptr::write_volatile(addr as *mut u16, value);
}

unsafe fn modern_write32(addr: usize, value: u32) {
    core::ptr::write_volatile(addr as *mut u32, value);
}

unsafe fn modern_write64(addr: usize, value: u64) {
    core::ptr::write_volatile(addr as *mut u64, value);
}
