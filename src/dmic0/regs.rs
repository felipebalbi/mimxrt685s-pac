#[doc = "Channel Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chanen(pub u32);
impl Chanen {
    #[doc = "Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub const fn en_ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel 0. When 1, PDM channel 0 is enabled."]
    #[inline(always)]
    pub fn set_en_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub const fn en_ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel 1. When 1, PDM channel 1 is enabled."]
    #[inline(always)]
    pub fn set_en_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable channel 2. When 1, PDM channel 2 is enabled."]
    #[inline(always)]
    pub const fn en_ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel 2. When 1, PDM channel 2 is enabled."]
    #[inline(always)]
    pub fn set_en_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable channel 3. When 1, PDM channel 3 is enabled."]
    #[inline(always)]
    pub const fn en_ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel 3. When 1, PDM channel 3 is enabled."]
    #[inline(always)]
    pub fn set_en_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable channel 4. When 1, PDM channel 4 is enabled."]
    #[inline(always)]
    pub const fn en_ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel 4. When 1, PDM channel 4 is enabled."]
    #[inline(always)]
    pub fn set_en_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable channel 5. When 1, PDM channel 5 is enabled."]
    #[inline(always)]
    pub const fn en_ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel 5. When 1, PDM channel 5 is enabled."]
    #[inline(always)]
    pub fn set_en_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable channel 6. When 1, PDM channel 6 is enabled."]
    #[inline(always)]
    pub const fn en_ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel 6. When 1, PDM channel 6 is enabled."]
    #[inline(always)]
    pub fn set_en_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable channel 7. When 1, PDM channel 7 is enabled."]
    #[inline(always)]
    pub const fn en_ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable channel 7. When 1, PDM channel 7 is enabled."]
    #[inline(always)]
    pub fn set_en_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Chanen {
    #[inline(always)]
    fn default() -> Chanen {
        Chanen(0)
    }
}
#[doc = "DC Filter Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DcCtrl(pub u32);
impl DcCtrl {
    #[doc = "DC block filter"]
    #[inline(always)]
    pub const fn dcpole(&self) -> super::vals::Dcpole {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Dcpole::from_bits(val as u8)
    }
    #[doc = "DC block filter"]
    #[inline(always)]
    pub fn set_dcpole(&mut self, val: super::vals::Dcpole) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Fine gain adjustment in the form of a number of bits to downshift."]
    #[inline(always)]
    pub const fn dcgain(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Fine gain adjustment in the form of a number of bits to downshift."]
    #[inline(always)]
    pub fn set_dcgain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Selects 16-bit saturation."]
    #[inline(always)]
    pub const fn saturateat16bit(&self) -> super::vals::Saturateat16bit {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Saturateat16bit::from_bits(val as u8)
    }
    #[doc = "Selects 16-bit saturation."]
    #[inline(always)]
    pub fn set_saturateat16bit(&mut self, val: super::vals::Saturateat16bit) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Sign extend."]
    #[inline(always)]
    pub const fn signextend(&self) -> super::vals::Signextend {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Signextend::from_bits(val as u8)
    }
    #[doc = "Sign extend."]
    #[inline(always)]
    pub fn set_signextend(&mut self, val: super::vals::Signextend) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for DcCtrl {
    #[inline(always)]
    fn default() -> DcCtrl {
        DcCtrl(0)
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Decreset(pub u32);
impl Decreset {
    #[doc = "no description available"]
    #[inline(always)]
    pub const fn decreset(&self) -> super::vals::Decreset {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Decreset::from_bits(val as u8)
    }
    #[doc = "no description available"]
    #[inline(always)]
    pub fn set_decreset(&mut self, val: super::vals::Decreset) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Decreset {
    #[inline(always)]
    fn default() -> Decreset {
        Decreset(0)
    }
}
#[doc = "Divider for generating PDM clock from DMIC clock input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Divhfclk(pub u32);
impl Divhfclk {
    #[doc = "Divide by factor to create PDM Clock (enumerated type)"]
    #[inline(always)]
    pub const fn pdmdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Divide by factor to create PDM Clock (enumerated type)"]
    #[inline(always)]
    pub fn set_pdmdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Divhfclk {
    #[inline(always)]
    fn default() -> Divhfclk {
        Divhfclk(0)
    }
}
#[doc = "FIFO Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoCtrl(pub u32);
impl FifoCtrl {
    #[doc = "FIFO enable."]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "FIFO enable."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FIFO reset."]
    #[inline(always)]
    pub const fn resetn(&self) -> super::vals::Resetn {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Resetn::from_bits(val as u8)
    }
    #[doc = "FIFO reset."]
    #[inline(always)]
    pub fn set_resetn(&mut self, val: super::vals::Resetn) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt enable."]
    #[inline(always)]
    pub const fn inten(&self) -> super::vals::Inten {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Inten::from_bits(val as u8)
    }
    #[doc = "Interrupt enable."]
    #[inline(always)]
    pub fn set_inten(&mut self, val: super::vals::Inten) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMA enable"]
    #[inline(always)]
    pub const fn dmaen(&self) -> super::vals::Dmaen {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmaen::from_bits(val as u8)
    }
    #[doc = "DMA enable"]
    #[inline(always)]
    pub fn set_dmaen(&mut self, val: super::vals::Dmaen) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Trigger level for interrupt"]
    #[inline(always)]
    pub const fn triglvl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Trigger level for interrupt"]
    #[inline(always)]
    pub fn set_triglvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for FifoCtrl {
    #[inline(always)]
    fn default() -> FifoCtrl {
        FifoCtrl(0)
    }
}
#[doc = "FIFO Data"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoData(pub u32);
impl FifoData {
    #[doc = "PCM Data"]
    #[inline(always)]
    pub const fn data(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "PCM Data"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for FifoData {
    #[inline(always)]
    fn default() -> FifoData {
        FifoData(0)
    }
}
#[doc = "FIFO Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoStatus(pub u32);
impl FifoStatus {
    #[doc = "Status of Interrupt (write 1 to clear)"]
    #[inline(always)]
    pub const fn int(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Status of Interrupt (write 1 to clear)"]
    #[inline(always)]
    pub fn set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Overrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub const fn overrun(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub fn set_overrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Underrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub const fn underrun(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun Detected (write 1 to clear)"]
    #[inline(always)]
    pub fn set_underrun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for FifoStatus {
    #[inline(always)]
    fn default() -> FifoStatus {
        FifoStatus(0)
    }
}
#[doc = "Decimator output gain adjustment"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gainshift(pub u32);
impl Gainshift {
    #[doc = "Gain shift for decimator output (can be positive or negative number)"]
    #[inline(always)]
    pub const fn gain(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Gain shift for decimator output (can be positive or negative number)"]
    #[inline(always)]
    pub fn set_gain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Gainshift {
    #[inline(always)]
    fn default() -> Gainshift {
        Gainshift(0)
    }
}
#[doc = "no description available"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlobalCountVal(pub u32);
impl GlobalCountVal {
    #[doc = "32bit value, global sync counter will trigger a pulse whenever count reaches GCOUNTVAL"]
    #[inline(always)]
    pub const fn ccountval(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "32bit value, global sync counter will trigger a pulse whenever count reaches GCOUNTVAL"]
    #[inline(always)]
    pub fn set_ccountval(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for GlobalCountVal {
    #[inline(always)]
    fn default() -> GlobalCountVal {
        GlobalCountVal(0)
    }
}
#[doc = "global sync enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlobalSyncEn(pub u32);
impl GlobalSyncEn {
    #[doc = "Choose which channels to sync to global sync (7:0) corresponds to the 8 channels"]
    #[inline(always)]
    pub const fn ch_sync_en(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Choose which channels to sync to global sync (7:0) corresponds to the 8 channels"]
    #[inline(always)]
    pub fn set_ch_sync_en(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for GlobalSyncEn {
    #[inline(always)]
    fn default() -> GlobalSyncEn {
        GlobalSyncEn(0)
    }
}
#[doc = "HWVAD input gain register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwvadgain(pub u32);
impl Hwvadgain {
    #[doc = "Gain factor for input signal into HWVAD"]
    #[inline(always)]
    pub const fn inputgain(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Gain factor for input signal into HWVAD"]
    #[inline(always)]
    pub fn set_inputgain(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Hwvadgain {
    #[inline(always)]
    fn default() -> Hwvadgain {
        Hwvadgain(0)
    }
}
#[doc = "HWVAD filter control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwvadhpfs(pub u32);
impl Hwvadhpfs {
    #[doc = "This field chooses the High Pass filter in first part of HWVAD"]
    #[inline(always)]
    pub const fn hpfs(&self) -> super::vals::Hpfs {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Hpfs::from_bits(val as u8)
    }
    #[doc = "This field chooses the High Pass filter in first part of HWVAD"]
    #[inline(always)]
    pub fn set_hpfs(&mut self, val: super::vals::Hpfs) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Hwvadhpfs {
    #[inline(always)]
    fn default() -> Hwvadhpfs {
        Hwvadhpfs(0)
    }
}
#[doc = "HWVAD noise envelope estimator register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwvadlowz(pub u32);
impl Hwvadlowz {
    #[doc = "Average noise-floor value"]
    #[inline(always)]
    pub const fn lowz(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Average noise-floor value"]
    #[inline(always)]
    pub fn set_lowz(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Hwvadlowz {
    #[inline(always)]
    fn default() -> Hwvadlowz {
        Hwvadlowz(0)
    }
}
#[doc = "HWVAD filter reset register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwvadrstt(pub u32);
impl Hwvadrstt {
    #[doc = "Reset HWVAD. Write back to 0 to release reset."]
    #[inline(always)]
    pub const fn rstt(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reset HWVAD. Write back to 0 to release reset."]
    #[inline(always)]
    pub fn set_rstt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Hwvadrstt {
    #[inline(always)]
    fn default() -> Hwvadrstt {
        Hwvadrstt(0)
    }
}
#[doc = "HWVAD control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwvadst10(pub u32);
impl Hwvadst10 {
    #[doc = "1' means enter stage 1 of VAD, ie a sound change has been detected and the HWVAD is being allowed to settle. Use 0 when changing back to detection mode. Allow several milliseconds in stage 1 for settling."]
    #[inline(always)]
    pub const fn st10(&self) -> super::vals::St10 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::St10::from_bits(val as u8)
    }
    #[doc = "1' means enter stage 1 of VAD, ie a sound change has been detected and the HWVAD is being allowed to settle. Use 0 when changing back to detection mode. Allow several milliseconds in stage 1 for settling."]
    #[inline(always)]
    pub fn set_st10(&mut self, val: super::vals::St10) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Hwvadst10 {
    #[inline(always)]
    fn default() -> Hwvadst10 {
        Hwvadst10(0)
    }
}
#[doc = "HWVAD noise estimator gain register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwvadthgn(pub u32);
impl Hwvadthgn {
    #[doc = "Gain Factor for Noise-floor - use a positive number to make average less sensitive to sudden changes"]
    #[inline(always)]
    pub const fn thgn(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Gain Factor for Noise-floor - use a positive number to make average less sensitive to sudden changes"]
    #[inline(always)]
    pub fn set_thgn(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Hwvadthgn {
    #[inline(always)]
    fn default() -> Hwvadthgn {
        Hwvadthgn(0)
    }
}
#[doc = "HWVAD signal estimator gain register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwvadthgs(pub u32);
impl Hwvadthgs {
    #[doc = "Signal Gain factor - use a postive number to make current signal stand out more over longer term average"]
    #[inline(always)]
    pub const fn thgs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Signal Gain factor - use a postive number to make current signal stand out more over longer term average"]
    #[inline(always)]
    pub fn set_thgs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Hwvadthgs {
    #[inline(always)]
    fn default() -> Hwvadthgs {
        Hwvadthgs(0)
    }
}
#[doc = "CIC Filter decimation rate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osr(pub u32);
impl Osr {
    #[doc = "Selects the oversample rate for the related input channel."]
    #[inline(always)]
    pub const fn osr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Selects the oversample rate for the related input channel."]
    #[inline(always)]
    pub fn set_osr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Osr {
    #[inline(always)]
    fn default() -> Osr {
        Osr(0)
    }
}
#[doc = "Phy Ctrl"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PhyCtrl(pub u32);
impl PhyCtrl {
    #[doc = "Capture DMIC on Falling edge (0 means on rising)"]
    #[inline(always)]
    pub const fn phy_fall(&self) -> super::vals::PhyFall {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::PhyFall::from_bits(val as u8)
    }
    #[doc = "Capture DMIC on Falling edge (0 means on rising)"]
    #[inline(always)]
    pub fn set_phy_fall(&mut self, val: super::vals::PhyFall) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Use Half rate sampling (ie Clock to dmic is sent at half the speed than the decimator is providing)"]
    #[inline(always)]
    pub const fn phy_half(&self) -> super::vals::PhyHalf {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::PhyHalf::from_bits(val as u8)
    }
    #[doc = "Use Half rate sampling (ie Clock to dmic is sent at half the speed than the decimator is providing)"]
    #[inline(always)]
    pub fn set_phy_half(&mut self, val: super::vals::PhyHalf) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for PhyCtrl {
    #[inline(always)]
    fn default() -> PhyCtrl {
        PhyCtrl(0)
    }
}
#[doc = "Compensation filter for 2FS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Preac2fscoef(pub u32);
impl Preac2fscoef {
    #[doc = "Co-efficient choice for CIC droop compensation droop filter"]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Co-efficient choice for CIC droop compensation droop filter"]
    #[inline(always)]
    pub fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Preac2fscoef {
    #[inline(always)]
    fn default() -> Preac2fscoef {
        Preac2fscoef(0)
    }
}
#[doc = "Compensation filter for 4FS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Preac4fscoef(pub u32);
impl Preac4fscoef {
    #[doc = "Co-efficient choice for CIC droop compensation droop filter"]
    #[inline(always)]
    pub const fn comp(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Co-efficient choice for CIC droop compensation droop filter"]
    #[inline(always)]
    pub fn set_comp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for Preac4fscoef {
    #[inline(always)]
    fn default() -> Preac4fscoef {
        Preac4fscoef(0)
    }
}
#[doc = "Use 2FS register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Use2fs(pub u32);
impl Use2fs {
    #[doc = "Use 2FS register"]
    #[inline(always)]
    pub const fn use2fs(&self) -> super::vals::Use2fs {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Use2fs::from_bits(val as u8)
    }
    #[doc = "Use 2FS register"]
    #[inline(always)]
    pub fn set_use2fs(&mut self, val: super::vals::Use2fs) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Use2fs {
    #[inline(always)]
    fn default() -> Use2fs {
        Use2fs(0)
    }
}
