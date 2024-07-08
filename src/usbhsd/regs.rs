#[doc = "USB Data buffer start address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Databufstart(pub u32);
impl Databufstart {
    #[doc = "The fixed portion of the data buffer start address."]
    #[inline(always)]
    pub const fn da_buf_fixed(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[doc = "The fixed portion of the data buffer start address."]
    #[inline(always)]
    pub fn set_da_buf_fixed(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[doc = "Programmable portion of the data buffer start address."]
    #[inline(always)]
    pub const fn da_buf(&self) -> u16 {
        let val = (self.0 >> 18usize) & 0x3fff;
        val as u16
    }
    #[doc = "Programmable portion of the data buffer start address."]
    #[inline(always)]
    pub fn set_da_buf(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 18usize)) | (((val as u32) & 0x3fff) << 18usize);
    }
}
impl Default for Databufstart {
    #[inline(always)]
    fn default() -> Databufstart {
        Databufstart(0)
    }
}
#[doc = "USB Device Command/Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Devcmdstat(pub u32);
impl Devcmdstat {
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub fn set_dev_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "USB device enable."]
    #[inline(always)]
    pub const fn dev_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "USB device enable."]
    #[inline(always)]
    pub fn set_dev_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "SETUP token received."]
    #[inline(always)]
    pub const fn setup(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SETUP token received."]
    #[inline(always)]
    pub fn set_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub const fn force_needclk(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub fn set_force_needclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If this bit is set to 1, the VBUS voltage indicators from the PHY are overruled."]
    #[inline(always)]
    pub const fn force_vbus(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set to 1, the VBUS voltage indicators from the PHY are overruled."]
    #[inline(always)]
    pub fn set_force_vbus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "LPM Supported:."]
    #[inline(always)]
    pub const fn lpm_sup(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Supported:."]
    #[inline(always)]
    pub fn set_lpm_sup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub const fn intonnak_ao(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub fn set_intonnak_ao(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub const fn intonnak_ai(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub fn set_intonnak_ai(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub const fn intonnak_co(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub fn set_intonnak_co(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub const fn intonnak_ci(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub fn set_intonnak_ci(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Device status - connect."]
    #[inline(always)]
    pub const fn dcon(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - connect."]
    #[inline(always)]
    pub fn set_dcon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Device status - suspend."]
    #[inline(always)]
    pub const fn dsus(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - suspend."]
    #[inline(always)]
    pub fn set_dsus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Device status - LPM Suspend."]
    #[inline(always)]
    pub const fn lpm_sus(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - LPM Suspend."]
    #[inline(always)]
    pub fn set_lpm_sus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    pub const fn lpm_rewp(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    pub fn set_lpm_rewp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[inline(always)]
    pub const fn speed(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[doc = "This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[inline(always)]
    pub fn set_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[doc = "Device status - connect change."]
    #[inline(always)]
    pub const fn dcon_c(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - connect change."]
    #[inline(always)]
    pub fn set_dcon_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Device status - suspend change."]
    #[inline(always)]
    pub const fn dsus_c(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - suspend change."]
    #[inline(always)]
    pub fn set_dsus_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Device status - reset change."]
    #[inline(always)]
    pub const fn dres_c(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Device status - reset change."]
    #[inline(always)]
    pub fn set_dres_c(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "This bit indicates if VBUS is detected or not."]
    #[inline(always)]
    pub const fn vbus_debounced(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "This bit indicates if VBUS is detected or not."]
    #[inline(always)]
    pub fn set_vbus_debounced(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
    #[inline(always)]
    pub const fn phy_test_mode(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification."]
    #[inline(always)]
    pub fn set_phy_test_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for Devcmdstat {
    #[inline(always)]
    fn default() -> Devcmdstat {
        Devcmdstat(0)
    }
}
#[doc = "USB Endpoint Buffer Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epbufcfg(pub u32);
impl Epbufcfg {
    #[doc = "Buffer usage: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub const fn buf_sb(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Buffer usage: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn set_buf_sb(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
    }
}
impl Default for Epbufcfg {
    #[inline(always)]
    fn default() -> Epbufcfg {
        Epbufcfg(0)
    }
}
#[doc = "USB Endpoint Buffer in use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epinuse(pub u32);
impl Epinuse {
    #[doc = "Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub const fn buf(&self) -> u16 {
        let val = (self.0 >> 2usize) & 0x03ff;
        val as u16
    }
    #[doc = "Buffer in use: This register has one bit per physical endpoint."]
    #[inline(always)]
    pub fn set_buf(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 2usize)) | (((val as u32) & 0x03ff) << 2usize);
    }
}
impl Default for Epinuse {
    #[inline(always)]
    fn default() -> Epinuse {
        Epinuse(0)
    }
}
#[doc = "USB EP Command/Status List start address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epliststart(pub u32);
impl Epliststart {
    #[doc = "Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub const fn ep_list_prg(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0x0fff;
        val as u16
    }
    #[doc = "Programmable portion of the USB EP Command/Status List address."]
    #[inline(always)]
    pub fn set_ep_list_prg(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 8usize)) | (((val as u32) & 0x0fff) << 8usize);
    }
    #[doc = "Fixed portion of USB EP Command/Status List address."]
    #[inline(always)]
    pub const fn ep_list_fixed(&self) -> u16 {
        let val = (self.0 >> 20usize) & 0x0fff;
        val as u16
    }
    #[doc = "Fixed portion of USB EP Command/Status List address."]
    #[inline(always)]
    pub fn set_ep_list_fixed(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 20usize)) | (((val as u32) & 0x0fff) << 20usize);
    }
}
impl Default for Epliststart {
    #[inline(always)]
    fn default() -> Epliststart {
        Epliststart(0)
    }
}
#[doc = "USB Endpoint skip"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epskip(pub u32);
impl Epskip {
    #[doc = "Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub const fn skip(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Endpoint skip: Writing 1 to one of these bits, will indicate to HW that it must deactivate the buffer assigned to this endpoint and return control back to software."]
    #[inline(always)]
    pub fn set_skip(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Epskip {
    #[inline(always)]
    fn default() -> Epskip {
        Epskip(0)
    }
}
#[doc = "USB Endpoint toggle register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eptoggle(pub u32);
impl Eptoggle {
    #[doc = "Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub const fn toggle(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "Endpoint data toggle: This field indicates the current value of the data toggle for the corresponding endpoint."]
    #[inline(always)]
    pub fn set_toggle(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Eptoggle {
    #[inline(always)]
    fn default() -> Eptoggle {
        Eptoggle(0)
    }
}
#[doc = "USB Info register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Info(pub u32);
impl Info {
    #[doc = "Frame number."]
    #[inline(always)]
    pub const fn frame_nr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "Frame number."]
    #[inline(always)]
    pub fn set_frame_nr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
    #[doc = "The error code which last occurred:."]
    #[inline(always)]
    pub const fn err_code(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "The error code which last occurred:."]
    #[inline(always)]
    pub fn set_err_code(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Minor revision."]
    #[inline(always)]
    pub const fn minrev(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Minor revision."]
    #[inline(always)]
    pub fn set_minrev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Major revision."]
    #[inline(always)]
    pub const fn majrev(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Major revision."]
    #[inline(always)]
    pub fn set_majrev(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Info {
    #[inline(always)]
    fn default() -> Info {
        Info(0)
    }
}
#[doc = "USB interrupt enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn ep_int_en(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn set_ep_int_en(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn frame_int_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn set_frame_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub const fn dev_int_en(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If this bit is set and the corresponding USB interrupt status bit is set, a HW interrupt is generated on the interrupt line."]
    #[inline(always)]
    pub fn set_dev_int_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "USB set interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intsetstat(pub u32);
impl Intsetstat {
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn ep_set_int(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn set_ep_set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn frame_set_int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn set_frame_set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub const fn dev_set_int(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If software writes a one to one of these bits, the corresponding USB interrupt status bit is set."]
    #[inline(always)]
    pub fn set_dev_set_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Intsetstat {
    #[inline(always)]
    fn default() -> Intsetstat {
        Intsetstat(0)
    }
}
#[doc = "USB interrupt status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub const fn ep0out(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the Control EP0 OUT direction."]
    #[inline(always)]
    pub fn set_ep0out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub const fn ep0in(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the Control EP0 IN direction."]
    #[inline(always)]
    pub fn set_ep0in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub const fn ep1out(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP1 OUT direction."]
    #[inline(always)]
    pub fn set_ep1out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub const fn ep1in(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP1 IN direction."]
    #[inline(always)]
    pub fn set_ep1in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub const fn ep2out(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP2 OUT direction."]
    #[inline(always)]
    pub fn set_ep2out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub const fn ep2in(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP2 IN direction."]
    #[inline(always)]
    pub fn set_ep2in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub const fn ep3out(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP3 OUT direction."]
    #[inline(always)]
    pub fn set_ep3out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub const fn ep3in(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP3 IN direction."]
    #[inline(always)]
    pub fn set_ep3in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub const fn ep4out(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP4 OUT direction."]
    #[inline(always)]
    pub fn set_ep4out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub const fn ep4in(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP4 IN direction."]
    #[inline(always)]
    pub fn set_ep4in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub const fn ep5out(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP5 OUT direction."]
    #[inline(always)]
    pub fn set_ep5out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub const fn ep5in(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt status register bit for the EP5 IN direction."]
    #[inline(always)]
    pub fn set_ep5in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Frame interrupt."]
    #[inline(always)]
    pub const fn frame_int(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Frame interrupt."]
    #[inline(always)]
    pub fn set_frame_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Device status interrupt."]
    #[inline(always)]
    pub const fn dev_int(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Device status interrupt."]
    #[inline(always)]
    pub fn set_dev_int(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
#[doc = "USB Link Power Management register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpm(pub u32);
impl Lpm {
    #[doc = "Host Initiated Resume Duration - HW."]
    #[inline(always)]
    pub const fn hird_hw(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Host Initiated Resume Duration - HW."]
    #[inline(always)]
    pub fn set_hird_hw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub const fn hird_sw(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Host Initiated Resume Duration - SW."]
    #[inline(always)]
    pub fn set_hird_sw(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub const fn data_pending(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "As long as this bit is set to one and LPM supported bit is set to one, HW will return a NYET handshake on every LPM token it receives."]
    #[inline(always)]
    pub fn set_data_pending(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Lpm {
    #[inline(always)]
    fn default() -> Lpm {
        Lpm(0)
    }
}
