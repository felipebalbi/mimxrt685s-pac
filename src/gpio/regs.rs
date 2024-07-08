#[doc = "Byte pin registers for all port 0 and 1 GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct B(pub u8);
impl B {
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn pbyte(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read: state of the pin PIOm_n, regardless of direction, masking, or alternate function, except that pins configured as analog I/O always read as 0. One register for each port pin. Supported pins depends on the specific device and package. Write: loads the pin's output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn set_pbyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
    }
}
impl Default for B {
    #[inline(always)]
    fn default() -> B {
        B(0)
    }
}
#[doc = "Clear port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clr(pub u32);
impl Clr {
    #[doc = "Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub const fn clrp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Clear output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear output bit."]
    #[inline(always)]
    pub fn set_clrp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Clr {
    #[inline(always)]
    fn default() -> Clr {
        Clr(0)
    }
}
#[doc = "Direction registers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dir(pub u32);
impl Dir {
    #[doc = "Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub const fn dirp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Selects pin direction for pin PIOm_n (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = input. 1 = output."]
    #[inline(always)]
    pub fn set_dirp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dir {
    #[inline(always)]
    fn default() -> Dir {
        Dir(0)
    }
}
#[doc = "Clear pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirclr(pub u32);
impl Dirclr {
    #[doc = "Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[inline(always)]
    pub const fn dirclrp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Clear direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Clear direction bit."]
    #[inline(always)]
    pub fn set_dirclrp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dirclr {
    #[inline(always)]
    fn default() -> Dirclr {
        Dirclr(0)
    }
}
#[doc = "Toggle pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirnot(pub u32);
impl Dirnot {
    #[doc = "Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[inline(always)]
    pub const fn dirnotp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Toggle direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle direction bit."]
    #[inline(always)]
    pub fn set_dirnotp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dirnot {
    #[inline(always)]
    fn default() -> Dirnot {
        Dirnot(0)
    }
}
#[doc = "Set pin direction bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dirset(pub u32);
impl Dirset {
    #[doc = "Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    pub const fn dirsetp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Set direction bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = No operation. 1 = Set direction bit."]
    #[inline(always)]
    pub fn set_dirsetp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dirset {
    #[inline(always)]
    fn default() -> Dirset {
        Dirset(0)
    }
}
#[doc = "choose edge or level for interrupt"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intedg(pub u32);
impl Intedg {
    #[doc = "choose level or edge based detection for each pin(bit0 for pion_0, bit1 for pion_1, etc)"]
    #[inline(always)]
    pub const fn edge(&self) -> super::vals::Edge {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Edge::from_bits(val as u32)
    }
    #[doc = "choose level or edge based detection for each pin(bit0 for pion_0, bit1 for pion_1, etc)"]
    #[inline(always)]
    pub fn set_edge(&mut self, val: super::vals::Edge) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intedg {
    #[inline(always)]
    fn default() -> Intedg {
        Intedg(0)
    }
}
#[doc = "interrupt A enable control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intena(pub u32);
impl Intena {
    #[doc = "interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
    #[inline(always)]
    pub const fn int_en(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
    #[inline(always)]
    pub fn set_int_en(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intena {
    #[inline(always)]
    fn default() -> Intena {
        Intena(0)
    }
}
#[doc = "interrupt B enable control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenb(pub u32);
impl Intenb {
    #[doc = "interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
    #[inline(always)]
    pub const fn int_en(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "interrupt enable control for each pin(bit 0 for pion_0, bin 1 for pion_1, etc)"]
    #[inline(always)]
    pub fn set_int_en(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intenb {
    #[inline(always)]
    fn default() -> Intenb {
        Intenb(0)
    }
}
#[doc = "interupt polarity control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intpol(pub u32);
impl Intpol {
    #[doc = "polarity control for each pin(bit 0 for pion_0, bit 1 for pion_1, etc.)"]
    #[inline(always)]
    pub const fn pol_ctl(&self) -> super::vals::PolCtl {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::PolCtl::from_bits(val as u32)
    }
    #[doc = "polarity control for each pin(bit 0 for pion_0, bit 1 for pion_1, etc.)"]
    #[inline(always)]
    pub fn set_pol_ctl(&mut self, val: super::vals::PolCtl) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intpol {
    #[inline(always)]
    fn default() -> Intpol {
        Intpol(0)
    }
}
#[doc = "interrupt status for interrupt A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstata(pub u32);
impl Intstata {
    #[doc = "interrupt status"]
    #[inline(always)]
    pub const fn status(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "interrupt status"]
    #[inline(always)]
    pub fn set_status(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intstata {
    #[inline(always)]
    fn default() -> Intstata {
        Intstata(0)
    }
}
#[doc = "interrupt status for interrupt B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstatb(pub u32);
impl Intstatb {
    #[doc = "interrupt status"]
    #[inline(always)]
    pub const fn status(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "interrupt status"]
    #[inline(always)]
    pub fn set_status(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Intstatb {
    #[inline(always)]
    fn default() -> Intstatb {
        Intstatb(0)
    }
}
#[doc = "Mask register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask(pub u32);
impl Mask {
    #[doc = "Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub const fn maskp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Controls which bits corresponding to PIOm_n are active in the MPORT register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read MPORT: pin state; write MPORT: load output bit. 1 = Read MPORT: 0; write MPORT: output bit not affected."]
    #[inline(always)]
    pub fn set_maskp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mask {
    #[inline(always)]
    fn default() -> Mask {
        Mask(0)
    }
}
#[doc = "Masked port register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpin(pub u32);
impl Mpin {
    #[doc = "Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub const fn mportp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Masked port register (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is LOW and/or the corresponding bit in the MASK register is 1; write: clear output bit if the corresponding bit in the MASK register is 0. 1 = Read: pin is HIGH and the corresponding bit in the MASK register is 0; write: set output bit if the corresponding bit in the MASK register is 0."]
    #[inline(always)]
    pub fn set_mportp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Mpin {
    #[inline(always)]
    fn default() -> Mpin {
        Mpin(0)
    }
}
#[doc = "Toggle port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Not(pub u32);
impl Not {
    #[doc = "Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub const fn notp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Toggle output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = no operation. 1 = Toggle output bit."]
    #[inline(always)]
    pub fn set_notp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Not {
    #[inline(always)]
    fn default() -> Not {
        Not(0)
    }
}
#[doc = "Port pin register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pin(pub u32);
impl Pin {
    #[doc = "Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub const fn port(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Reads pin states or loads output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: pin is low; write: clear output bit. 1 = Read: pin is high; write: set output bit."]
    #[inline(always)]
    pub fn set_port(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Pin {
    #[inline(always)]
    fn default() -> Pin {
        Pin(0)
    }
}
#[doc = "Write: Set register for port Read: output bits for port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Set(pub u32);
impl Set {
    #[doc = "Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub const fn setp(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read or set output bits (bit 0 = PIOn_0, bit 1 = PIOn_1, etc.). Supported pins depends on the specific device and package. 0 = Read: output bit: write: no operation. 1 = Read: output bit; write: set output bit."]
    #[inline(always)]
    pub fn set_setp(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Set {
    #[inline(always)]
    fn default() -> Set {
        Set(0)
    }
}
#[doc = "Word pin registers for all port 0 and 1 GPIO pins"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct W(pub u32);
impl W {
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub const fn pword(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Read 0: pin PIOm_n is LOW. Write 0: clear output bit. Read 0xFFFF FFFF: pin PIOm_n is HIGH. Write any value 0x0000 0001 to 0xFFFF FFFF: set output bit. Only 0 or 0xFFFF FFFF can be read. Writing any value other than 0 will set the output bit. One register for each port pin. Supported pins depends on the specific device and package."]
    #[inline(always)]
    pub fn set_pword(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for W {
    #[inline(always)]
    fn default() -> W {
        W(0)
    }
}
