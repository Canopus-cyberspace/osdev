use core::sync::atomic::{compiler_fence, Ordering};

const VIRTIO0: usize = 0x1000_1000;

const REG_MAGIC: usize = 0x000;
const REG_VERSION: usize = 0x004;
const REG_DEVICE_ID: usize = 0x008;
const REG_DEVICE_FEATURES: usize = 0x010;
const REG_DEVICE_FEATURES_SEL: usize = 0x014;
const REG_DRIVER_FEATURES: usize = 0x020;
const REG_DRIVER_FEATURES_SEL: usize = 0x024;
const REG_GUEST_PAGE_SIZE: usize = 0x028;
const REG_QUEUE_SEL: usize = 0x030;
const REG_QUEUE_NUM_MAX: usize = 0x034;
const REG_QUEUE_NUM: usize = 0x038;
const REG_QUEUE_ALIGN: usize = 0x03c;
const REG_QUEUE_PFN: usize = 0x040;
const REG_QUEUE_READY: usize = 0x044;
const REG_QUEUE_NOTIFY: usize = 0x050;
const REG_INTERRUPT_STATUS: usize = 0x060;
const REG_INTERRUPT_ACK: usize = 0x064;
const REG_STATUS: usize = 0x070;
const REG_QUEUE_DESC_LOW: usize = 0x080;
const REG_QUEUE_DESC_HIGH: usize = 0x084;
const REG_QUEUE_DRIVER_LOW: usize = 0x090;
const REG_QUEUE_DRIVER_HIGH: usize = 0x094;
const REG_QUEUE_DEVICE_LOW: usize = 0x0a0;
const REG_QUEUE_DEVICE_HIGH: usize = 0x0a4;

const STATUS_ACKNOWLEDGE: u32 = 1;
const STATUS_DRIVER: u32 = 2;
const STATUS_DRIVER_OK: u32 = 4;
const STATUS_FEATURES_OK: u32 = 8;
const STATUS_FAILED: u32 = 128;

const VIRTIO_MAGIC: u32 = 0x7472_6976;
const VIRTIO_VERSION_LEGACY: u32 = 1;
const VIRTIO_VERSION_MODERN: u32 = 2;
const VIRTIO_DEVICE_BLOCK: u32 = 2;
const VIRTIO_F_VERSION_1_HIGH: u32 = 1;

const QUEUE_SIZE: usize = 8;
const SECTOR_SIZE: usize = 512;
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
        Self { addr: 0, len: 0, flags: 0, next: 0 }
    }
}

#[repr(C, align(16))]
struct VirtqDescTable {
    desc: [VirtqDesc; QUEUE_SIZE],
}

#[repr(C, align(2))]
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

#[repr(C, align(4))]
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
struct LegacyQueue([u8; LEGACY_QUEUE_BYTES]);

static mut DESC_TABLE: VirtqDescTable = VirtqDescTable { desc: [VirtqDesc::empty(); QUEUE_SIZE] };
static mut AVAIL_RING: VirtqAvail = VirtqAvail { flags: 0, idx: 0, ring: [0; QUEUE_SIZE], used_event: 0 };
static mut USED_RING: VirtqUsed = VirtqUsed { flags: 0, idx: 0, ring: [VirtqUsedElem::empty(); QUEUE_SIZE], avail_event: 0 };
static mut LEGACY_QUEUE: LegacyQueue = LegacyQueue([0; LEGACY_QUEUE_BYTES]);
static mut REQUEST: VirtioBlkReq = VirtioBlkReq { req_type: 0, reserved: 0, sector: 0 };
static mut DATA: [u8; SECTOR_SIZE] = [0; SECTOR_SIZE];
static mut REQ_STATUS: u8 = 0xff;
static mut READY: bool = false;
static mut LEGACY_MODE: bool = false;
static mut LAST_USED_IDX: u16 = 0;

pub fn init() {
    crate::println!("[drivers::virtio_blk] stub init");
}

pub fn read_sector(sector: u64, out: &mut [u8; SECTOR_SIZE]) -> Result<(), &'static str> {
    ensure_ready()?;
    unsafe {
        REQUEST = VirtioBlkReq { req_type: VIRTIO_BLK_T_IN, reserved: 0, sector };
        DATA = [0; SECTOR_SIZE];
        REQ_STATUS = 0xff;

        let desc = desc_ptr();
        let avail = avail_ptr();
        let used = used_ptr();

        core::ptr::write(desc.add(0), VirtqDesc {
            addr: core::ptr::addr_of!(REQUEST) as u64,
            len: core::mem::size_of::<VirtioBlkReq>() as u32,
            flags: VRING_DESC_F_NEXT,
            next: 1,
        });
        core::ptr::write(desc.add(1), VirtqDesc {
            addr: core::ptr::addr_of!(DATA) as u64,
            len: SECTOR_SIZE as u32,
            flags: VRING_DESC_F_NEXT | VRING_DESC_F_WRITE,
            next: 2,
        });
        core::ptr::write(desc.add(2), VirtqDesc {
            addr: core::ptr::addr_of!(REQ_STATUS) as u64,
            len: 1,
            flags: VRING_DESC_F_WRITE,
            next: 0,
        });

        let avail_idx = core::ptr::read_volatile(core::ptr::addr_of!((*avail).idx));
        let slot = avail_idx as usize % QUEUE_SIZE;
        (*avail).ring[slot] = 0;
        compiler_fence(Ordering::SeqCst);
        core::ptr::write_volatile(core::ptr::addr_of_mut!((*avail).idx), avail_idx.wrapping_add(1));
        compiler_fence(Ordering::SeqCst);
        mmio_write(REG_QUEUE_NOTIFY, 0);

        let start = LAST_USED_IDX;
        let mut spin = 0usize;
        while core::ptr::read_volatile(core::ptr::addr_of!((*used).idx)) == start && spin < 10_000_000 {
            spin += 1;
        }
        let used_idx = core::ptr::read_volatile(core::ptr::addr_of!((*used).idx));
        if used_idx == start {
            return Err("virtio_timeout");
        }
        LAST_USED_IDX = used_idx;
        let interrupt = mmio_read(REG_INTERRUPT_STATUS);
        if interrupt != 0 {
            mmio_write(REG_INTERRUPT_ACK, interrupt);
        }
        if core::ptr::read_volatile(core::ptr::addr_of!(REQ_STATUS)) != 0 {
            return Err("virtio_status");
        }
        let mut i = 0usize;
        while i < SECTOR_SIZE {
            out[i] = core::ptr::read_volatile(core::ptr::addr_of!(DATA[i]));
            i += 1;
        }
    }
    Ok(())
}

fn ensure_ready() -> Result<(), &'static str> {
    unsafe {
        if READY {
            return Ok(());
        }
        if mmio_read(REG_MAGIC) != VIRTIO_MAGIC {
            return Err("virtio_magic");
        }
        if mmio_read(REG_VERSION) != VIRTIO_VERSION_MODERN {
            if mmio_read(REG_VERSION) == VIRTIO_VERSION_LEGACY {
                return ensure_ready_legacy();
            }
            return Err("virtio_version");
        }
        if mmio_read(REG_DEVICE_ID) != VIRTIO_DEVICE_BLOCK {
            return Err("virtio_device");
        }
        LEGACY_MODE = false;

        mmio_write(REG_STATUS, 0);
        mmio_write(REG_STATUS, STATUS_ACKNOWLEDGE);
        mmio_write(REG_STATUS, STATUS_ACKNOWLEDGE | STATUS_DRIVER);

        mmio_write(REG_DEVICE_FEATURES_SEL, 1);
        let high_features = mmio_read(REG_DEVICE_FEATURES);
        if (high_features & VIRTIO_F_VERSION_1_HIGH) == 0 {
            mmio_write(REG_STATUS, mmio_read(REG_STATUS) | STATUS_FAILED);
            return Err("virtio_feature_version");
        }
        mmio_write(REG_DRIVER_FEATURES_SEL, 0);
        mmio_write(REG_DRIVER_FEATURES, 0);
        mmio_write(REG_DRIVER_FEATURES_SEL, 1);
        mmio_write(REG_DRIVER_FEATURES, VIRTIO_F_VERSION_1_HIGH);

        mmio_write(REG_STATUS, mmio_read(REG_STATUS) | STATUS_FEATURES_OK);
        if (mmio_read(REG_STATUS) & STATUS_FEATURES_OK) == 0 {
            return Err("virtio_features_ok");
        }

        mmio_write(REG_QUEUE_SEL, 0);
        let max = mmio_read(REG_QUEUE_NUM_MAX) as usize;
        if max < QUEUE_SIZE || max == 0 {
            return Err("virtio_queue_size");
        }
        mmio_write(REG_QUEUE_READY, 0);
        mmio_write(REG_QUEUE_NUM, QUEUE_SIZE as u32);
        AVAIL_RING = VirtqAvail { flags: 0, idx: 0, ring: [0; QUEUE_SIZE], used_event: 0 };
        USED_RING = VirtqUsed { flags: 0, idx: 0, ring: [VirtqUsedElem::empty(); QUEUE_SIZE], avail_event: 0 };
        LAST_USED_IDX = 0;

        let desc = core::ptr::addr_of!(DESC_TABLE) as u64;
        let avail = core::ptr::addr_of!(AVAIL_RING) as u64;
        let used = core::ptr::addr_of!(USED_RING) as u64;
        mmio_write(REG_QUEUE_DESC_LOW, desc as u32);
        mmio_write(REG_QUEUE_DESC_HIGH, (desc >> 32) as u32);
        mmio_write(REG_QUEUE_DRIVER_LOW, avail as u32);
        mmio_write(REG_QUEUE_DRIVER_HIGH, (avail >> 32) as u32);
        mmio_write(REG_QUEUE_DEVICE_LOW, used as u32);
        mmio_write(REG_QUEUE_DEVICE_HIGH, (used >> 32) as u32);
        mmio_write(REG_QUEUE_READY, 1);
        mmio_write(REG_STATUS, mmio_read(REG_STATUS) | STATUS_DRIVER_OK);
        compiler_fence(Ordering::SeqCst);
        READY = true;
    }
    Ok(())
}

unsafe fn ensure_ready_legacy() -> Result<(), &'static str> {
    if mmio_read(REG_DEVICE_ID) != VIRTIO_DEVICE_BLOCK {
        return Err("virtio_device");
    }
    LEGACY_MODE = true;
    mmio_write(REG_STATUS, 0);
    mmio_write(REG_STATUS, STATUS_ACKNOWLEDGE);
    mmio_write(REG_STATUS, STATUS_ACKNOWLEDGE | STATUS_DRIVER);

    mmio_write(REG_DRIVER_FEATURES_SEL, 0);
    mmio_write(REG_DRIVER_FEATURES, 0);
    mmio_write(REG_GUEST_PAGE_SIZE, 4096);
    mmio_write(REG_QUEUE_SEL, 0);
    let max = mmio_read(REG_QUEUE_NUM_MAX) as usize;
    if max < QUEUE_SIZE || max == 0 {
        return Err("virtio_queue_size");
    }
    let base = core::ptr::addr_of_mut!(LEGACY_QUEUE) as *mut u8;
    let mut i = 0usize;
    while i < LEGACY_QUEUE_BYTES {
        core::ptr::write_volatile(base.add(i), 0);
        i += 1;
    }
    LAST_USED_IDX = 0;
    mmio_write(REG_QUEUE_NUM, QUEUE_SIZE as u32);
    mmio_write(REG_QUEUE_ALIGN, 4096);
    mmio_write(REG_QUEUE_PFN, (base as usize >> 12) as u32);
    mmio_write(REG_STATUS, mmio_read(REG_STATUS) | STATUS_DRIVER_OK);
    compiler_fence(Ordering::SeqCst);
    READY = true;
    Ok(())
}

unsafe fn desc_ptr() -> *mut VirtqDesc {
    if LEGACY_MODE {
        core::ptr::addr_of_mut!(LEGACY_QUEUE) as *mut VirtqDesc
    } else {
        core::ptr::addr_of_mut!(DESC_TABLE.desc[0])
    }
}

unsafe fn avail_ptr() -> *mut VirtqAvail {
    if LEGACY_MODE {
        (core::ptr::addr_of_mut!(LEGACY_QUEUE) as *mut u8).add(QUEUE_SIZE * core::mem::size_of::<VirtqDesc>()) as *mut VirtqAvail
    } else {
        core::ptr::addr_of_mut!(AVAIL_RING)
    }
}

unsafe fn used_ptr() -> *mut VirtqUsed {
    if LEGACY_MODE {
        (core::ptr::addr_of_mut!(LEGACY_QUEUE) as *mut u8).add(LEGACY_USED_OFFSET) as *mut VirtqUsed
    } else {
        core::ptr::addr_of_mut!(USED_RING)
    }
}

unsafe fn mmio_read(offset: usize) -> u32 {
    core::ptr::read_volatile((VIRTIO0 + offset) as *const u32)
}

unsafe fn mmio_write(offset: usize, value: u32) {
    core::ptr::write_volatile((VIRTIO0 + offset) as *mut u32, value);
}
