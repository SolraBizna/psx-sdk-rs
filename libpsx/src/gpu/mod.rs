use core::marker::PhantomData;
use crate::bios;
use crate::util::zero_extend;

pub mod color;
pub mod position;
pub mod polygon;
pub mod line;
pub mod vram;

// TODO: Think about whether it makes sense to keep a copy of the GPUSTAT settings in a field
// or whether this should only provide an interface to read/write the GPU settings
// TODO: Think about whether this should be must_use or not
//#[must_use]
pub struct Ctxt<S: Screen> {
    gpustat: u32,
    display: PhantomData<S>,
}

pub struct Disabled;
pub struct Enabled;

pub trait Screen {}
impl Screen for Disabled {}
impl Screen for Enabled {}

impl<S: Screen> Ctxt<S> {
    pub fn reset_gpu(self) -> Ctxt<Disabled> {
        bios::gpu_gp1_command_word(0x0000_0000);
        Ctxt::new()
    }
    pub fn reset_buffer(self) -> Self {
        bios::gpu_gp1_command_word(0x0100_0000);
        self
    }
    pub fn start_display(self, pos: (u16, u16)) -> Self {
        let cmd = 0x05 << 24;
        let mut pos = zero_extend(pos);
        pos.0 &= 0b11_1111_1111;
        pos.1 &= 0b01_1111_1111;
        bios::gpu_gp1_command_word(cmd | pos.0 | pos.1 << 10);
        self
    }
}

impl Ctxt<Disabled> {
    pub fn new() -> Self {
        const DEFAULT_GPUSTAT: u32 = 0x1480_2000;
        Ctxt::<Disabled> {
            gpustat: DEFAULT_GPUSTAT,
            display: PhantomData::<Disabled>,
        }
    }
    pub fn display_on(self) -> Ctxt<Enabled> {
        bios::gpu_gp1_command_word(0x0300_0000);
        Ctxt::<Enabled> {
            gpustat: self.gpustat,
            display: PhantomData::<Enabled>,
        }
    }
    pub fn toggle_display(self) -> Ctxt<Enabled> {
        self.display_on()
    }
}

impl Ctxt<Enabled> {
    pub fn display_off(self) -> Ctxt<Disabled> {
        bios::gpu_gp1_command_word(0x0300_0001);
        Ctxt::<Disabled> {
            gpustat: self.gpustat,
            display: PhantomData::<Disabled>,
        }
    }
    pub fn toggle_display(self) -> Ctxt<Disabled> {
        self.display_off()
    }
}
