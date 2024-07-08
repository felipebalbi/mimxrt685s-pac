#[doc = "Master Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mconfig(pub u32);
impl Mconfig {
    #[doc = "Master enable"]
    #[inline(always)]
    pub const fn mstena(&self) -> super::vals::Mstena {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Mstena::from_bits(val as u8)
    }
    #[doc = "Master enable"]
    #[inline(always)]
    pub fn set_mstena(&mut self, val: super::vals::Mstena) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Disable Timeout"]
    #[inline(always)]
    pub const fn disto(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Timeout"]
    #[inline(always)]
    pub fn set_disto(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "High-Keeper"]
    #[inline(always)]
    pub const fn hkeep(&self) -> super::vals::Hkeep {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Hkeep::from_bits(val as u8)
    }
    #[doc = "High-Keeper"]
    #[inline(always)]
    pub fn set_hkeep(&mut self, val: super::vals::Hkeep) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Open drain stop"]
    #[inline(always)]
    pub const fn odstop(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Open drain stop"]
    #[inline(always)]
    pub fn set_odstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Push-pull baud rate"]
    #[inline(always)]
    pub const fn ppbaud(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Push-pull baud rate"]
    #[inline(always)]
    pub fn set_ppbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Push-Pull low"]
    #[inline(always)]
    pub const fn pplow(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Push-Pull low"]
    #[inline(always)]
    pub fn set_pplow(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Open drain baud rate"]
    #[inline(always)]
    pub const fn odbaud(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Open drain baud rate"]
    #[inline(always)]
    pub fn set_odbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Open drain high push-pull"]
    #[inline(always)]
    pub const fn odhpp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Open drain high push-pull"]
    #[inline(always)]
    pub fn set_odhpp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Skew"]
    #[inline(always)]
    pub const fn skew(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x07;
        val as u8
    }
    #[doc = "Skew"]
    #[inline(always)]
    pub fn set_skew(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
    }
    #[doc = "I2C baud rate"]
    #[inline(always)]
    pub const fn i2cbaud(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "I2C baud rate"]
    #[inline(always)]
    pub fn set_i2cbaud(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Mconfig {
    #[inline(always)]
    fn default() -> Mconfig {
        Mconfig(0)
    }
}
#[doc = "Master Main Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mctrl(pub u32);
impl Mctrl {
    #[doc = "Request"]
    #[inline(always)]
    pub const fn request(&self) -> super::vals::Request {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Request::from_bits(val as u8)
    }
    #[doc = "Request"]
    #[inline(always)]
    pub fn set_request(&mut self, val: super::vals::Request) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Bus type with START"]
    #[inline(always)]
    pub const fn type_(&self) -> super::vals::Type {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Type::from_bits(val as u8)
    }
    #[doc = "Bus type with START"]
    #[inline(always)]
    pub fn set_type_(&mut self, val: super::vals::Type) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "In-Band Interrupt (IBI) response"]
    #[inline(always)]
    pub const fn ibiresp(&self) -> super::vals::Ibiresp {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Ibiresp::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupt (IBI) response"]
    #[inline(always)]
    pub fn set_ibiresp(&mut self, val: super::vals::Ibiresp) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "DIR"]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::MctrlDir {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::MctrlDir::from_bits(val as u8)
    }
    #[doc = "DIR"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: super::vals::MctrlDir) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "ADDR"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x7f;
        val as u8
    }
    #[doc = "ADDR"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 9usize)) | (((val as u32) & 0x7f) << 9usize);
    }
    #[doc = "Read terminate"]
    #[inline(always)]
    pub const fn rdterm(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Read terminate"]
    #[inline(always)]
    pub fn set_rdterm(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Mctrl {
    #[inline(always)]
    fn default() -> Mctrl {
        Mctrl(0)
    }
}
#[doc = "Master Data Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdatactrl(pub u32);
impl Mdatactrl {
    #[doc = "Flush to-bus buffer/FIFO"]
    #[inline(always)]
    pub const fn flushtb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush to-bus buffer/FIFO"]
    #[inline(always)]
    pub fn set_flushtb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flush from-bus buffer/FIFO"]
    #[inline(always)]
    pub const fn flushfb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flush from-bus buffer/FIFO"]
    #[inline(always)]
    pub fn set_flushfb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Unlock"]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock"]
    #[inline(always)]
    pub fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "TX trigger level"]
    #[inline(always)]
    pub const fn txtrig(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "TX trigger level"]
    #[inline(always)]
    pub fn set_txtrig(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "RX trigger level"]
    #[inline(always)]
    pub const fn rxtrig(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "RX trigger level"]
    #[inline(always)]
    pub fn set_rxtrig(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "TX byte count"]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "TX byte count"]
    #[inline(always)]
    pub fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "RX byte count"]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "RX byte count"]
    #[inline(always)]
    pub fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "TX is full"]
    #[inline(always)]
    pub const fn txfull(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "TX is full"]
    #[inline(always)]
    pub fn set_txfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "RX is empty"]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "RX is empty"]
    #[inline(always)]
    pub fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mdatactrl {
    #[inline(always)]
    fn default() -> Mdatactrl {
        Mdatactrl(0)
    }
}
#[doc = "Master DMA Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdmactrl(pub u32);
impl Mdmactrl {
    #[doc = "DMA from bus"]
    #[inline(always)]
    pub const fn dmafb(&self) -> super::vals::MdmactrlDmafb {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MdmactrlDmafb::from_bits(val as u8)
    }
    #[doc = "DMA from bus"]
    #[inline(always)]
    pub fn set_dmafb(&mut self, val: super::vals::MdmactrlDmafb) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA to bus"]
    #[inline(always)]
    pub const fn dmatb(&self) -> super::vals::MdmactrlDmatb {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::MdmactrlDmatb::from_bits(val as u8)
    }
    #[doc = "DMA to bus"]
    #[inline(always)]
    pub fn set_dmatb(&mut self, val: super::vals::MdmactrlDmatb) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "DMA width"]
    #[inline(always)]
    pub const fn dmawidth(&self) -> super::vals::MdmactrlDmawidth {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::MdmactrlDmawidth::from_bits(val as u8)
    }
    #[doc = "DMA width"]
    #[inline(always)]
    pub fn set_dmawidth(&mut self, val: super::vals::MdmactrlDmawidth) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Mdmactrl {
    #[inline(always)]
    fn default() -> Mdmactrl {
        Mdmactrl(0)
    }
}
#[doc = "Master Dynamic Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mdynaddr(pub u32);
impl Mdynaddr {
    #[doc = "Dynamic address valid"]
    #[inline(always)]
    pub const fn davalid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Dynamic address valid"]
    #[inline(always)]
    pub fn set_davalid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic address"]
    #[inline(always)]
    pub const fn daddr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic address"]
    #[inline(always)]
    pub fn set_daddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Mdynaddr {
    #[inline(always)]
    fn default() -> Mdynaddr {
        Mdynaddr(0)
    }
}
#[doc = "Master Errors and Warnings Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Merrwarn(pub u32);
impl Merrwarn {
    #[doc = "Not acknowledge (NACK) error"]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Not acknowledge (NACK) error"]
    #[inline(always)]
    pub fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "WRABT (Write abort) error"]
    #[inline(always)]
    pub const fn wrabt(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "WRABT (Write abort) error"]
    #[inline(always)]
    pub fn set_wrabt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Terminate error"]
    #[inline(always)]
    pub const fn term(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Terminate error"]
    #[inline(always)]
    pub fn set_term(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "High data rate parity"]
    #[inline(always)]
    pub const fn hpar(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "High data rate parity"]
    #[inline(always)]
    pub fn set_hpar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "High data rate CRC error"]
    #[inline(always)]
    pub const fn hcrc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "High data rate CRC error"]
    #[inline(always)]
    pub fn set_hcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Over-read error"]
    #[inline(always)]
    pub const fn oread(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Over-read error"]
    #[inline(always)]
    pub fn set_oread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Over-write error"]
    #[inline(always)]
    pub const fn owrite(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Over-write error"]
    #[inline(always)]
    pub fn set_owrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Message error"]
    #[inline(always)]
    pub const fn msgerr(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Message error"]
    #[inline(always)]
    pub fn set_msgerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Invalid request error"]
    #[inline(always)]
    pub const fn invreq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid request error"]
    #[inline(always)]
    pub fn set_invreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "TIMEOUT error"]
    #[inline(always)]
    pub const fn timeout(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "TIMEOUT error"]
    #[inline(always)]
    pub fn set_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Merrwarn {
    #[inline(always)]
    fn default() -> Merrwarn {
        Merrwarn(0)
    }
}
#[doc = "Master In-band Interrupt Registry and Rules Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mibirules(pub u32);
impl Mibirules {
    #[doc = "ADDR0"]
    #[inline(always)]
    pub const fn addr0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR0"]
    #[inline(always)]
    pub fn set_addr0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "ADDR1"]
    #[inline(always)]
    pub const fn addr1(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR1"]
    #[inline(always)]
    pub fn set_addr1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
    }
    #[doc = "ADDR2"]
    #[inline(always)]
    pub const fn addr2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR2"]
    #[inline(always)]
    pub fn set_addr2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
    }
    #[doc = "ADDR3"]
    #[inline(always)]
    pub const fn addr3(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR3"]
    #[inline(always)]
    pub fn set_addr3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 18usize)) | (((val as u32) & 0x3f) << 18usize);
    }
    #[doc = "ADDR4"]
    #[inline(always)]
    pub const fn addr4(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "ADDR4"]
    #[inline(always)]
    pub fn set_addr4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "Set Most Significant address Bit to 0"]
    #[inline(always)]
    pub const fn msb0(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Set Most Significant address Bit to 0"]
    #[inline(always)]
    pub fn set_msb0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "No IBI byte"]
    #[inline(always)]
    pub const fn nobyte(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "No IBI byte"]
    #[inline(always)]
    pub fn set_nobyte(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mibirules {
    #[inline(always)]
    fn default() -> Mibirules {
        Mibirules(0)
    }
}
#[doc = "Master Interrupt Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintclr(pub u32);
impl Mintclr {
    #[doc = "SLVSTART interrupt enable clear"]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SLVSTART interrupt enable clear"]
    #[inline(always)]
    pub fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MCTRLDONE interrupt enable clear"]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MCTRLDONE interrupt enable clear"]
    #[inline(always)]
    pub fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "COMPLETE interrupt enable clear"]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "COMPLETE interrupt enable clear"]
    #[inline(always)]
    pub fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND interrupt enable clear"]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND interrupt enable clear"]
    #[inline(always)]
    pub fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXNOTFULL interrupt enable clear"]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXNOTFULL interrupt enable clear"]
    #[inline(always)]
    pub fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IBIWON interrupt enable clear"]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IBIWON interrupt enable clear"]
    #[inline(always)]
    pub fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERRWARN interrupt enable clear"]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN interrupt enable clear"]
    #[inline(always)]
    pub fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NOWMASTER interrupt enable clear"]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "NOWMASTER interrupt enable clear"]
    #[inline(always)]
    pub fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintclr {
    #[inline(always)]
    fn default() -> Mintclr {
        Mintclr(0)
    }
}
#[doc = "Master Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintmasked(pub u32);
impl Mintmasked {
    #[doc = "SLVSTART interrupt mask"]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SLVSTART interrupt mask"]
    #[inline(always)]
    pub fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MCTRLDONE interrupt mask"]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MCTRLDONE interrupt mask"]
    #[inline(always)]
    pub fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "COMPLETE interrupt mask"]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "COMPLETE interrupt mask"]
    #[inline(always)]
    pub fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND interrupt mask"]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND interrupt mask"]
    #[inline(always)]
    pub fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXNOTFULL interrupt mask"]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXNOTFULL interrupt mask"]
    #[inline(always)]
    pub fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "IBIWON interrupt mask"]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "IBIWON interrupt mask"]
    #[inline(always)]
    pub fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "ERRWARN interrupt mask"]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN interrupt mask"]
    #[inline(always)]
    pub fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "NOWMASTER interrupt mask"]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "NOWMASTER interrupt mask"]
    #[inline(always)]
    pub fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintmasked {
    #[inline(always)]
    fn default() -> Mintmasked {
        Mintmasked(0)
    }
}
#[doc = "Master Interrupt Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mintset(pub u32);
impl Mintset {
    #[doc = "Slave start interrupt enable"]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave start interrupt enable"]
    #[inline(always)]
    pub fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Master control done interrupt enable"]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Master control done interrupt enable"]
    #[inline(always)]
    pub fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Completed message interrupt enable"]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Completed message interrupt enable"]
    #[inline(always)]
    pub fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RX pending interrupt enable"]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RX pending interrupt enable"]
    #[inline(always)]
    pub fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TX buffer/FIFO is not full interrupt enable"]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TX buffer/FIFO is not full interrupt enable"]
    #[inline(always)]
    pub fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "In-Band Interrupt (IBI) won interrupt enable"]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupt (IBI) won interrupt enable"]
    #[inline(always)]
    pub fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error or warning (ERRWARN) interrupt enable"]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error or warning (ERRWARN) interrupt enable"]
    #[inline(always)]
    pub fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Now master (now this I3C module is a master) interrupt enable"]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Now master (now this I3C module is a master) interrupt enable"]
    #[inline(always)]
    pub fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Mintset {
    #[inline(always)]
    fn default() -> Mintset {
        Mintset(0)
    }
}
#[doc = "Master Read Data Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdatab(pub u32);
impl Mrdatab {
    #[doc = "VALUE"]
    #[inline(always)]
    pub const fn value(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn set_value(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mrdatab {
    #[inline(always)]
    fn default() -> Mrdatab {
        Mrdatab(0)
    }
}
#[doc = "Master Read Data Half-word Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mrdatah(pub u32);
impl Mrdatah {
    #[doc = "LSB"]
    #[inline(always)]
    pub const fn lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "LSB"]
    #[inline(always)]
    pub fn set_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "MSB"]
    #[inline(always)]
    pub const fn msb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "MSB"]
    #[inline(always)]
    pub fn set_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Mrdatah {
    #[inline(always)]
    fn default() -> Mrdatah {
        Mrdatah(0)
    }
}
#[doc = "Master Read Message in DDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrmsgDdr(pub u32);
impl MrmsgDdr {
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Current length"]
    #[inline(always)]
    pub const fn clen(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "Current length"]
    #[inline(always)]
    pub fn set_clen(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
}
impl Default for MrmsgDdr {
    #[inline(always)]
    fn default() -> MrmsgDdr {
        MrmsgDdr(0)
    }
}
#[doc = "Master Read Message in SDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MrmsgSdr(pub u32);
impl MrmsgSdr {
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MrmsgSdr {
    #[inline(always)]
    fn default() -> MrmsgSdr {
        MrmsgSdr(0)
    }
}
#[doc = "Master Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstatus(pub u32);
impl Mstatus {
    #[doc = "State of the master"]
    #[inline(always)]
    pub const fn state(&self) -> super::vals::State {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::State::from_bits(val as u8)
    }
    #[doc = "State of the master"]
    #[inline(always)]
    pub fn set_state(&mut self, val: super::vals::State) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Between messages or Dynamic Address Assignments (DAA)"]
    #[inline(always)]
    pub const fn between(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Between messages or Dynamic Address Assignments (DAA)"]
    #[inline(always)]
    pub fn set_between(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Not acknowledged"]
    #[inline(always)]
    pub const fn nacked(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Not acknowledged"]
    #[inline(always)]
    pub fn set_nacked(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "In-Band Interrupt (IBI) type"]
    #[inline(always)]
    pub const fn ibitype(&self) -> super::vals::Ibitype {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Ibitype::from_bits(val as u8)
    }
    #[doc = "In-Band Interrupt (IBI) type"]
    #[inline(always)]
    pub fn set_ibitype(&mut self, val: super::vals::Ibitype) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Slave start"]
    #[inline(always)]
    pub const fn slvstart(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave start"]
    #[inline(always)]
    pub fn set_slvstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Master control done"]
    #[inline(always)]
    pub const fn mctrldone(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Master control done"]
    #[inline(always)]
    pub fn set_mctrldone(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "COMPLETE"]
    #[inline(always)]
    pub const fn complete(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "COMPLETE"]
    #[inline(always)]
    pub fn set_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND"]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND"]
    #[inline(always)]
    pub fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TX buffer/FIFO not yet full"]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TX buffer/FIFO not yet full"]
    #[inline(always)]
    pub fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "In-Band Interrupt (IBI) won"]
    #[inline(always)]
    pub const fn ibiwon(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupt (IBI) won"]
    #[inline(always)]
    pub fn set_ibiwon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Error or warning"]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error or warning"]
    #[inline(always)]
    pub fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Now master (now this module is a master)"]
    #[inline(always)]
    pub const fn nowmaster(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Now master (now this module is a master)"]
    #[inline(always)]
    pub fn set_nowmaster(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "IBI address"]
    #[inline(always)]
    pub const fn ibiaddr(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x7f;
        val as u8
    }
    #[doc = "IBI address"]
    #[inline(always)]
    pub fn set_ibiaddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
    }
}
impl Default for Mstatus {
    #[inline(always)]
    fn default() -> Mstatus {
        Mstatus(0)
    }
}
#[doc = "Master Write Data Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatab(pub u32);
impl Mwdatab {
    #[doc = "Data byte"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "End of message also"]
    #[inline(always)]
    pub const fn end_also(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of message also"]
    #[inline(always)]
    pub fn set_end_also(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mwdatab {
    #[inline(always)]
    fn default() -> Mwdatab {
        Mwdatab(0)
    }
}
#[doc = "Master Write Data Byte End Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatabe(pub u32);
impl Mwdatabe {
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mwdatabe {
    #[inline(always)]
    fn default() -> Mwdatabe {
        Mwdatabe(0)
    }
}
#[doc = "Master Write Data Half-word Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatah(pub u32);
impl Mwdatah {
    #[doc = "Data byte 0"]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 0"]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Data byte 1"]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Data byte 1"]
    #[inline(always)]
    pub fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Mwdatah {
    #[inline(always)]
    fn default() -> Mwdatah {
        Mwdatah(0)
    }
}
#[doc = "Master Write Data Byte End Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwdatahe(pub u32);
impl Mwdatahe {
    #[doc = "DATA 0"]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "DATA 0"]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "DATA 1"]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "DATA 1"]
    #[inline(always)]
    pub fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Mwdatahe {
    #[inline(always)]
    fn default() -> Mwdatahe {
        Mwdatahe(0)
    }
}
#[doc = "Master Write Message in DDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgDdrControl(pub u32);
impl MwmsgDdrControl {
    #[doc = "Length of message"]
    #[inline(always)]
    pub const fn len(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "Length of message"]
    #[inline(always)]
    pub fn set_len(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for MwmsgDdrControl {
    #[inline(always)]
    fn default() -> MwmsgDdrControl {
        MwmsgDdrControl(0)
    }
}
#[doc = "Master Write Message Data in DDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgDdrData(pub u32);
impl MwmsgDdrData {
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data16b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn set_data16b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MwmsgDdrData {
    #[inline(always)]
    fn default() -> MwmsgDdrData {
        MwmsgDdrData(0)
    }
}
#[doc = "Master Write Message in SDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgSdrControl(pub u32);
impl MwmsgSdrControl {
    #[doc = "Direction"]
    #[inline(always)]
    pub const fn dir(&self) -> super::vals::MwmsgSdrControlDir {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::MwmsgSdrControlDir::from_bits(val as u8)
    }
    #[doc = "Direction"]
    #[inline(always)]
    pub fn set_dir(&mut self, val: super::vals::MwmsgSdrControlDir) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Address to be written to"]
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Address to be written to"]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "End of SDR message"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End of SDR message"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "I2C"]
    #[inline(always)]
    pub const fn i2c(&self) -> super::vals::I2c {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::I2c::from_bits(val as u8)
    }
    #[doc = "I2C"]
    #[inline(always)]
    pub fn set_i2c(&mut self, val: super::vals::I2c) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "Length"]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Length"]
    #[inline(always)]
    pub fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
}
impl Default for MwmsgSdrControl {
    #[inline(always)]
    fn default() -> MwmsgSdrControl {
        MwmsgSdrControl(0)
    }
}
#[doc = "Master Write Message Data in SDR mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MwmsgSdrData(pub u32);
impl MwmsgSdrData {
    #[doc = "Data"]
    #[inline(always)]
    pub const fn data16b(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data"]
    #[inline(always)]
    pub fn set_data16b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for MwmsgSdrData {
    #[inline(always)]
    fn default() -> MwmsgSdrData {
        MwmsgSdrData(0)
    }
}
#[doc = "Slave Capabilities Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scapabilities(pub u32);
impl Scapabilities {
    #[doc = "ID 48b handler"]
    #[inline(always)]
    pub const fn idena(&self) -> super::vals::Idena {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Idena::from_bits(val as u8)
    }
    #[doc = "ID 48b handler"]
    #[inline(always)]
    pub fn set_idena(&mut self, val: super::vals::Idena) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "ID register"]
    #[inline(always)]
    pub const fn idreg(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "ID register"]
    #[inline(always)]
    pub fn set_idreg(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "HDR support"]
    #[inline(always)]
    pub const fn hdrsupp(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x07;
        val as u8
    }
    #[doc = "HDR support"]
    #[inline(always)]
    pub fn set_hdrsupp(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
    }
    #[doc = "Master"]
    #[inline(always)]
    pub const fn master(&self) -> super::vals::Master {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Master::from_bits(val as u8)
    }
    #[doc = "Master"]
    #[inline(always)]
    pub fn set_master(&mut self, val: super::vals::Master) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Static address"]
    #[inline(always)]
    pub const fn saddr(&self) -> super::vals::Saddr {
        let val = (self.0 >> 10usize) & 0x03;
        super::vals::Saddr::from_bits(val as u8)
    }
    #[doc = "Static address"]
    #[inline(always)]
    pub fn set_saddr(&mut self, val: super::vals::Saddr) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
    }
    #[doc = "Common Command Codes (CCC) handling"]
    #[inline(always)]
    pub const fn ccchandle(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Common Command Codes (CCC) handling"]
    #[inline(always)]
    pub fn set_ccchandle(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "In-Band Interrupts, Master Requests, Hot Join events"]
    #[inline(always)]
    pub const fn ibi_mr_hj(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "In-Band Interrupts, Master Requests, Hot Join events"]
    #[inline(always)]
    pub fn set_ibi_mr_hj(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Time control"]
    #[inline(always)]
    pub const fn timectrl(&self) -> super::vals::ScapabilitiesTimectrl {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::ScapabilitiesTimectrl::from_bits(val as u8)
    }
    #[doc = "Time control"]
    #[inline(always)]
    pub fn set_timectrl(&mut self, val: super::vals::ScapabilitiesTimectrl) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "External FIFO"]
    #[inline(always)]
    pub const fn extfifo(&self) -> super::vals::Extfifo {
        let val = (self.0 >> 23usize) & 0x07;
        super::vals::Extfifo::from_bits(val as u8)
    }
    #[doc = "External FIFO"]
    #[inline(always)]
    pub fn set_extfifo(&mut self, val: super::vals::Extfifo) {
        self.0 = (self.0 & !(0x07 << 23usize)) | (((val.to_bits() as u32) & 0x07) << 23usize);
    }
    #[doc = "FIFO transmit"]
    #[inline(always)]
    pub const fn fifotx(&self) -> super::vals::Fifotx {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::Fifotx::from_bits(val as u8)
    }
    #[doc = "FIFO transmit"]
    #[inline(always)]
    pub fn set_fifotx(&mut self, val: super::vals::Fifotx) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "FIFO receive"]
    #[inline(always)]
    pub const fn fiforx(&self) -> super::vals::Fiforx {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Fiforx::from_bits(val as u8)
    }
    #[doc = "FIFO receive"]
    #[inline(always)]
    pub fn set_fiforx(&mut self, val: super::vals::Fiforx) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "INT"]
    #[inline(always)]
    pub const fn int(&self) -> super::vals::Int {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Int::from_bits(val as u8)
    }
    #[doc = "INT"]
    #[inline(always)]
    pub fn set_int(&mut self, val: super::vals::Int) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DMA"]
    #[inline(always)]
    pub const fn dma(&self) -> super::vals::Dma {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dma::from_bits(val as u8)
    }
    #[doc = "DMA"]
    #[inline(always)]
    pub fn set_dma(&mut self, val: super::vals::Dma) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Scapabilities {
    #[inline(always)]
    fn default() -> Scapabilities {
        Scapabilities(0)
    }
}
#[doc = "Slave Configuration Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sconfig(pub u32);
impl Sconfig {
    #[doc = "Slave enable"]
    #[inline(always)]
    pub const fn slvena(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slave enable"]
    #[inline(always)]
    pub fn set_slvena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Not acknowledge"]
    #[inline(always)]
    pub const fn nack(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Not acknowledge"]
    #[inline(always)]
    pub fn set_nack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Match START or STOP"]
    #[inline(always)]
    pub const fn matchss(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Match START or STOP"]
    #[inline(always)]
    pub fn set_matchss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "S0/S1 errors ignore"]
    #[inline(always)]
    pub const fn s0ignore(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "S0/S1 errors ignore"]
    #[inline(always)]
    pub fn set_s0ignore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Double Data Rate OK"]
    #[inline(always)]
    pub const fn ddrok(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Double Data Rate OK"]
    #[inline(always)]
    pub fn set_ddrok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "ID random"]
    #[inline(always)]
    pub const fn idrand(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "ID random"]
    #[inline(always)]
    pub fn set_idrand(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Offline"]
    #[inline(always)]
    pub const fn offline(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Offline"]
    #[inline(always)]
    pub fn set_offline(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bus available match"]
    #[inline(always)]
    pub const fn bamatch(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bus available match"]
    #[inline(always)]
    pub fn set_bamatch(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Static address"]
    #[inline(always)]
    pub const fn saddr(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x7f;
        val as u8
    }
    #[doc = "Static address"]
    #[inline(always)]
    pub fn set_saddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 25usize)) | (((val as u32) & 0x7f) << 25usize);
    }
}
impl Default for Sconfig {
    #[inline(always)]
    fn default() -> Sconfig {
        Sconfig(0)
    }
}
#[doc = "Slave Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl(pub u32);
impl Sctrl {
    #[doc = "EVENT"]
    #[inline(always)]
    pub const fn event(&self) -> super::vals::Event {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Event::from_bits(val as u8)
    }
    #[doc = "EVENT"]
    #[inline(always)]
    pub fn set_event(&mut self, val: super::vals::Event) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "In-Band Interrupt data"]
    #[inline(always)]
    pub const fn ibidata(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "In-Band Interrupt data"]
    #[inline(always)]
    pub fn set_ibidata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Pending interrupt"]
    #[inline(always)]
    pub const fn pendint(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Pending interrupt"]
    #[inline(always)]
    pub fn set_pendint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Activity state (of slave)"]
    #[inline(always)]
    pub const fn actstate(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[doc = "Activity state (of slave)"]
    #[inline(always)]
    pub fn set_actstate(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[doc = "Vendor information"]
    #[inline(always)]
    pub const fn vendinfo(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "Vendor information"]
    #[inline(always)]
    pub fn set_vendinfo(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for Sctrl {
    #[inline(always)]
    fn default() -> Sctrl {
        Sctrl(0)
    }
}
#[doc = "Slave Data Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdatactrl(pub u32);
impl Sdatactrl {
    #[doc = "Flush the to-bus buffer/FIFO"]
    #[inline(always)]
    pub const fn flushtb(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Flush the to-bus buffer/FIFO"]
    #[inline(always)]
    pub fn set_flushtb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Flushes the from-bus buffer/FIFO"]
    #[inline(always)]
    pub const fn flushfb(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Flushes the from-bus buffer/FIFO"]
    #[inline(always)]
    pub fn set_flushfb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Unlock"]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Unlock"]
    #[inline(always)]
    pub fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Trigger level for TX FIFO emptiness"]
    #[inline(always)]
    pub const fn txtrig(&self) -> super::vals::Txtrig {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Txtrig::from_bits(val as u8)
    }
    #[doc = "Trigger level for TX FIFO emptiness"]
    #[inline(always)]
    pub fn set_txtrig(&mut self, val: super::vals::Txtrig) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Trigger level for RX FIFO fullness"]
    #[inline(always)]
    pub const fn rxtrig(&self) -> super::vals::Rxtrig {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::Rxtrig::from_bits(val as u8)
    }
    #[doc = "Trigger level for RX FIFO fullness"]
    #[inline(always)]
    pub fn set_rxtrig(&mut self, val: super::vals::Rxtrig) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "Count of bytes in TX"]
    #[inline(always)]
    pub const fn txcount(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Count of bytes in TX"]
    #[inline(always)]
    pub fn set_txcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Count of bytes in RX"]
    #[inline(always)]
    pub const fn rxcount(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Count of bytes in RX"]
    #[inline(always)]
    pub fn set_rxcount(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "TX is full"]
    #[inline(always)]
    pub const fn txfull(&self) -> super::vals::Txfull {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Txfull::from_bits(val as u8)
    }
    #[doc = "TX is full"]
    #[inline(always)]
    pub fn set_txfull(&mut self, val: super::vals::Txfull) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "RX is empty"]
    #[inline(always)]
    pub const fn rxempty(&self) -> super::vals::Rxempty {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Rxempty::from_bits(val as u8)
    }
    #[doc = "RX is empty"]
    #[inline(always)]
    pub fn set_rxempty(&mut self, val: super::vals::Rxempty) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdatactrl {
    #[inline(always)]
    fn default() -> Sdatactrl {
        Sdatactrl(0)
    }
}
#[doc = "Slave DMA Control Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmactrl(pub u32);
impl Sdmactrl {
    #[doc = "DMA Read (From-bus) trigger"]
    #[inline(always)]
    pub const fn dmafb(&self) -> super::vals::SdmactrlDmafb {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SdmactrlDmafb::from_bits(val as u8)
    }
    #[doc = "DMA Read (From-bus) trigger"]
    #[inline(always)]
    pub fn set_dmafb(&mut self, val: super::vals::SdmactrlDmafb) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "DMA Write (To-bus) trigger"]
    #[inline(always)]
    pub const fn dmatb(&self) -> super::vals::SdmactrlDmatb {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SdmactrlDmatb::from_bits(val as u8)
    }
    #[doc = "DMA Write (To-bus) trigger"]
    #[inline(always)]
    pub fn set_dmatb(&mut self, val: super::vals::SdmactrlDmatb) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Width of DMA operations"]
    #[inline(always)]
    pub const fn dmawidth(&self) -> super::vals::SdmactrlDmawidth {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SdmactrlDmawidth::from_bits(val as u8)
    }
    #[doc = "Width of DMA operations"]
    #[inline(always)]
    pub fn set_dmawidth(&mut self, val: super::vals::SdmactrlDmawidth) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
}
impl Default for Sdmactrl {
    #[inline(always)]
    fn default() -> Sdmactrl {
        Sdmactrl(0)
    }
}
#[doc = "Slave Dynamic Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdynaddr(pub u32);
impl Sdynaddr {
    #[doc = "DAVALID"]
    #[inline(always)]
    pub const fn davalid(&self) -> super::vals::Davalid {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Davalid::from_bits(val as u8)
    }
    #[doc = "DAVALID"]
    #[inline(always)]
    pub fn set_davalid(&mut self, val: super::vals::Davalid) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Dynamic address"]
    #[inline(always)]
    pub const fn daddr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Dynamic address"]
    #[inline(always)]
    pub fn set_daddr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Mapped Dynamic Address"]
    #[inline(always)]
    pub const fn mapidx(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Mapped Dynamic Address"]
    #[inline(always)]
    pub fn set_mapidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Map a Static Address"]
    #[inline(always)]
    pub const fn mapsa(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Map a Static Address"]
    #[inline(always)]
    pub fn set_mapsa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Key"]
    #[inline(always)]
    pub const fn key(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Key"]
    #[inline(always)]
    pub fn set_key(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Sdynaddr {
    #[inline(always)]
    fn default() -> Sdynaddr {
        Sdynaddr(0)
    }
}
#[doc = "Slave Errors and Warnings Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Serrwarn(pub u32);
impl Serrwarn {
    #[doc = "Overrun error"]
    #[inline(always)]
    pub const fn orun(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error"]
    #[inline(always)]
    pub fn set_orun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Underrun error"]
    #[inline(always)]
    pub const fn urun(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun error"]
    #[inline(always)]
    pub fn set_urun(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Underrun and Not Acknowledged (NACKed) error"]
    #[inline(always)]
    pub const fn urunnack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Underrun and Not Acknowledged (NACKed) error"]
    #[inline(always)]
    pub fn set_urunnack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Terminated error"]
    #[inline(always)]
    pub const fn term(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Terminated error"]
    #[inline(always)]
    pub fn set_term(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Invalid start error"]
    #[inline(always)]
    pub const fn invstart(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Invalid start error"]
    #[inline(always)]
    pub fn set_invstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "SDR parity error"]
    #[inline(always)]
    pub const fn spar(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "SDR parity error"]
    #[inline(always)]
    pub fn set_spar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "HDR parity error"]
    #[inline(always)]
    pub const fn hpar(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "HDR parity error"]
    #[inline(always)]
    pub fn set_hpar(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "HDR-DDR CRC error"]
    #[inline(always)]
    pub const fn hcrc(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "HDR-DDR CRC error"]
    #[inline(always)]
    pub fn set_hcrc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "S0 or S1 error"]
    #[inline(always)]
    pub const fn s0s1(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "S0 or S1 error"]
    #[inline(always)]
    pub fn set_s0s1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Over-read error"]
    #[inline(always)]
    pub const fn oread(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Over-read error"]
    #[inline(always)]
    pub fn set_oread(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Over-write error"]
    #[inline(always)]
    pub const fn owrite(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Over-write error"]
    #[inline(always)]
    pub fn set_owrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Serrwarn {
    #[inline(always)]
    fn default() -> Serrwarn {
        Serrwarn(0)
    }
}
#[doc = "Slave Module ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sid(pub u32);
impl Sid {
    #[doc = "ID"]
    #[inline(always)]
    pub const fn id(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "ID"]
    #[inline(always)]
    pub fn set_id(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sid {
    #[inline(always)]
    fn default() -> Sid {
        Sid(0)
    }
}
#[doc = "Slave ID Extension Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sidext(pub u32);
impl Sidext {
    #[doc = "Device Characteristic Register"]
    #[inline(always)]
    pub const fn dcr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Device Characteristic Register"]
    #[inline(always)]
    pub fn set_dcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Bus Characteristics Register"]
    #[inline(always)]
    pub const fn bcr(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Bus Characteristics Register"]
    #[inline(always)]
    pub fn set_bcr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Sidext {
    #[inline(always)]
    fn default() -> Sidext {
        Sidext(0)
    }
}
#[doc = "Slave ID Part Number Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sidpartno(pub u32);
impl Sidpartno {
    #[doc = "Part number"]
    #[inline(always)]
    pub const fn partno(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Part number"]
    #[inline(always)]
    pub fn set_partno(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Sidpartno {
    #[inline(always)]
    fn default() -> Sidpartno {
        Sidpartno(0)
    }
}
#[doc = "Slave Interrupt Clear Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintclr(pub u32);
impl Sintclr {
    #[doc = "START interrupt enable clear"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "START interrupt enable clear"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MATCHED interrupt enable clear"]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MATCHED interrupt enable clear"]
    #[inline(always)]
    pub fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "STOP interrupt enable clear"]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "STOP interrupt enable clear"]
    #[inline(always)]
    pub fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND interrupt enable clear"]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND interrupt enable clear"]
    #[inline(always)]
    pub fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXSEND interrupt enable clear"]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND interrupt enable clear"]
    #[inline(always)]
    pub fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DACHG interrupt enable clear"]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG interrupt enable clear"]
    #[inline(always)]
    pub fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CCC interrupt enable clear"]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CCC interrupt enable clear"]
    #[inline(always)]
    pub fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERRWARN interrupt enable clear"]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN interrupt enable clear"]
    #[inline(always)]
    pub fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DDRMATCHED interrupt enable clear"]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DDRMATCHED interrupt enable clear"]
    #[inline(always)]
    pub fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CHANDLED interrupt enable clear"]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CHANDLED interrupt enable clear"]
    #[inline(always)]
    pub fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "EVENT interrupt enable clear"]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "EVENT interrupt enable clear"]
    #[inline(always)]
    pub fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintclr {
    #[inline(always)]
    fn default() -> Sintclr {
        Sintclr(0)
    }
}
#[doc = "Slave Interrupt Mask Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintmasked(pub u32);
impl Sintmasked {
    #[doc = "START interrupt mask"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "START interrupt mask"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "MATCHED interrupt mask"]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "MATCHED interrupt mask"]
    #[inline(always)]
    pub fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "STOP interrupt mask"]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "STOP interrupt mask"]
    #[inline(always)]
    pub fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "RXPEND interrupt mask"]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "RXPEND interrupt mask"]
    #[inline(always)]
    pub fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "TXSEND interrupt mask"]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "TXSEND interrupt mask"]
    #[inline(always)]
    pub fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DACHG interrupt mask"]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG interrupt mask"]
    #[inline(always)]
    pub fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "CCC interrupt mask"]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "CCC interrupt mask"]
    #[inline(always)]
    pub fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "ERRWARN interrupt mask"]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "ERRWARN interrupt mask"]
    #[inline(always)]
    pub fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "DDRMATCHED interrupt mask"]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "DDRMATCHED interrupt mask"]
    #[inline(always)]
    pub fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "CHANDLED interrupt mask"]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "CHANDLED interrupt mask"]
    #[inline(always)]
    pub fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "EVENT interrupt mask"]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "EVENT interrupt mask"]
    #[inline(always)]
    pub fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintmasked {
    #[inline(always)]
    fn default() -> Sintmasked {
        Sintmasked(0)
    }
}
#[doc = "Slave Interrupt Set Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sintset(pub u32);
impl Sintset {
    #[doc = "Start interrupt enable"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Start interrupt enable"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Match interrupt enable"]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Match interrupt enable"]
    #[inline(always)]
    pub fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Stop interrupt enable"]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Stop interrupt enable"]
    #[inline(always)]
    pub fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Receive interrupt enable"]
    #[inline(always)]
    pub const fn rxpend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt enable"]
    #[inline(always)]
    pub fn set_rxpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit interrupt enable"]
    #[inline(always)]
    pub const fn txsend(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt enable"]
    #[inline(always)]
    pub fn set_txsend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Dynamic address change interrupt enable"]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Dynamic address change interrupt enable"]
    #[inline(always)]
    pub fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code (CCC) (that was not handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error/warning interrupt enable"]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error/warning interrupt enable"]
    #[inline(always)]
    pub fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Double Data Rate (DDR) interrupt enable"]
    #[inline(always)]
    pub const fn ddrmatched(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Double Data Rate (DDR) interrupt enable"]
    #[inline(always)]
    pub fn set_ddrmatched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code (CCC) (that was handled by I3C module) interrupt enable"]
    #[inline(always)]
    pub fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event interrupt enable"]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event interrupt enable"]
    #[inline(always)]
    pub fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for Sintset {
    #[inline(always)]
    fn default() -> Sintset {
        Sintset(0)
    }
}
#[doc = "Slave Maximum Limits Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smaxlimits(pub u32);
impl Smaxlimits {
    #[doc = "Maximum read length"]
    #[inline(always)]
    pub const fn maxrd(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum read length"]
    #[inline(always)]
    pub fn set_maxrd(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Maximum write length"]
    #[inline(always)]
    pub const fn maxwr(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "Maximum write length"]
    #[inline(always)]
    pub fn set_maxwr(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
}
impl Default for Smaxlimits {
    #[inline(always)]
    fn default() -> Smaxlimits {
        Smaxlimits(0)
    }
}
#[doc = "Slave Message-Mapped Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smsgmapaddr(pub u32);
impl Smsgmapaddr {
    #[doc = "Matched address index"]
    #[inline(always)]
    pub const fn maplast(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Matched address index"]
    #[inline(always)]
    pub fn set_maplast(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Previous match index 1"]
    #[inline(always)]
    pub const fn maplastm1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Previous match index 1"]
    #[inline(always)]
    pub fn set_maplastm1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Previous match index 2"]
    #[inline(always)]
    pub const fn maplastm2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Previous match index 2"]
    #[inline(always)]
    pub fn set_maplastm2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Smsgmapaddr {
    #[inline(always)]
    fn default() -> Smsgmapaddr {
        Smsgmapaddr(0)
    }
}
#[doc = "Slave Read Data Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdatab(pub u32);
impl Srdatab {
    #[doc = "Byte read from the master"]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Byte read from the master"]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Srdatab {
    #[inline(always)]
    fn default() -> Srdatab {
        Srdatab(0)
    }
}
#[doc = "Slave Read Data Half-word Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srdatah(pub u32);
impl Srdatah {
    #[doc = "The 1st byte read from the slave"]
    #[inline(always)]
    pub const fn lsb(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The 1st byte read from the slave"]
    #[inline(always)]
    pub fn set_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The 2nd byte read from the slave"]
    #[inline(always)]
    pub const fn msb(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The 2nd byte read from the slave"]
    #[inline(always)]
    pub fn set_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Srdatah {
    #[inline(always)]
    fn default() -> Srdatah {
        Srdatah(0)
    }
}
#[doc = "Slave Status Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sstatus(pub u32);
impl Sstatus {
    #[doc = "Status not stop"]
    #[inline(always)]
    pub const fn stnotstop(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Status not stop"]
    #[inline(always)]
    pub fn set_stnotstop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Status message"]
    #[inline(always)]
    pub const fn stmsg(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Status message"]
    #[inline(always)]
    pub fn set_stmsg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Status Common Command Code Handler"]
    #[inline(always)]
    pub const fn stccch(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Status Common Command Code Handler"]
    #[inline(always)]
    pub fn set_stccch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Status required"]
    #[inline(always)]
    pub const fn streqrd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Status required"]
    #[inline(always)]
    pub fn set_streqrd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Status request write"]
    #[inline(always)]
    pub const fn streqwr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Status request write"]
    #[inline(always)]
    pub fn set_streqwr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Status Dynamic Address Assignment"]
    #[inline(always)]
    pub const fn stdaa(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Status Dynamic Address Assignment"]
    #[inline(always)]
    pub fn set_stdaa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Status High Data Rate"]
    #[inline(always)]
    pub const fn sthdr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Status High Data Rate"]
    #[inline(always)]
    pub fn set_sthdr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Start"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Start"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Matched"]
    #[inline(always)]
    pub const fn matched(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Matched"]
    #[inline(always)]
    pub fn set_matched(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub const fn stop(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Stop"]
    #[inline(always)]
    pub fn set_stop(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Received message pending"]
    #[inline(always)]
    pub const fn rx_pend(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Received message pending"]
    #[inline(always)]
    pub fn set_rx_pend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Transmit buffer is not full"]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit buffer is not full"]
    #[inline(always)]
    pub fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "DACHG"]
    #[inline(always)]
    pub const fn dachg(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "DACHG"]
    #[inline(always)]
    pub fn set_dachg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Common Command Code"]
    #[inline(always)]
    pub const fn ccc(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Common Command Code"]
    #[inline(always)]
    pub fn set_ccc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Error warning"]
    #[inline(always)]
    pub const fn errwarn(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Error warning"]
    #[inline(always)]
    pub fn set_errwarn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "High Data Rate command match"]
    #[inline(always)]
    pub const fn hdrmatch(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "High Data Rate command match"]
    #[inline(always)]
    pub fn set_hdrmatch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Common-Command-Code handled"]
    #[inline(always)]
    pub const fn chandled(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Common-Command-Code handled"]
    #[inline(always)]
    pub fn set_chandled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Event"]
    #[inline(always)]
    pub const fn event(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Event"]
    #[inline(always)]
    pub fn set_event(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Event details"]
    #[inline(always)]
    pub const fn evdet(&self) -> super::vals::Evdet {
        let val = (self.0 >> 20usize) & 0x03;
        super::vals::Evdet::from_bits(val as u8)
    }
    #[doc = "Event details"]
    #[inline(always)]
    pub fn set_evdet(&mut self, val: super::vals::Evdet) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
    }
    #[doc = "In-Band Interrupts are disabled"]
    #[inline(always)]
    pub const fn ibidis(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "In-Band Interrupts are disabled"]
    #[inline(always)]
    pub fn set_ibidis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Master requests are disabled"]
    #[inline(always)]
    pub const fn mrdis(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Master requests are disabled"]
    #[inline(always)]
    pub fn set_mrdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Hot-Join is disabled"]
    #[inline(always)]
    pub const fn hjdis(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Hot-Join is disabled"]
    #[inline(always)]
    pub fn set_hjdis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Activity state from Common Command Codes (CCC)"]
    #[inline(always)]
    pub const fn actstate(&self) -> super::vals::Actstate {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Actstate::from_bits(val as u8)
    }
    #[doc = "Activity state from Common Command Codes (CCC)"]
    #[inline(always)]
    pub fn set_actstate(&mut self, val: super::vals::Actstate) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Time control"]
    #[inline(always)]
    pub const fn timectrl(&self) -> super::vals::SstatusTimectrl {
        let val = (self.0 >> 30usize) & 0x03;
        super::vals::SstatusTimectrl::from_bits(val as u8)
    }
    #[doc = "Time control"]
    #[inline(always)]
    pub fn set_timectrl(&mut self, val: super::vals::SstatusTimectrl) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
    }
}
impl Default for Sstatus {
    #[inline(always)]
    fn default() -> Sstatus {
        Sstatus(0)
    }
}
#[doc = "Slave Time Control Clock Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stcclock(pub u32);
impl Stcclock {
    #[doc = "Clock accuracy"]
    #[inline(always)]
    pub const fn accuracy(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock accuracy"]
    #[inline(always)]
    pub fn set_accuracy(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Clock frequency"]
    #[inline(always)]
    pub const fn freq(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Clock frequency"]
    #[inline(always)]
    pub fn set_freq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Stcclock {
    #[inline(always)]
    fn default() -> Stcclock {
        Stcclock(0)
    }
}
#[doc = "Slave Vendor ID Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Svendorid(pub u32);
impl Svendorid {
    #[doc = "Vendor ID"]
    #[inline(always)]
    pub const fn vid(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x7fff;
        val as u16
    }
    #[doc = "Vendor ID"]
    #[inline(always)]
    pub fn set_vid(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 0usize)) | (((val as u32) & 0x7fff) << 0usize);
    }
}
impl Default for Svendorid {
    #[inline(always)]
    fn default() -> Svendorid {
        Svendorid(0)
    }
}
#[doc = "Slave Write Data Byte Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatab(pub u32);
impl Swdatab {
    #[doc = "The data byte to send to the master"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The data byte to send to the master"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "End"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "End"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "End also"]
    #[inline(always)]
    pub const fn end_also(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End also"]
    #[inline(always)]
    pub fn set_end_also(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Swdatab {
    #[inline(always)]
    fn default() -> Swdatab {
        Swdatab(0)
    }
}
#[doc = "Slave Write Data Byte End"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatabe(pub u32);
impl Swdatabe {
    #[doc = "The data byte to send to the master"]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The data byte to send to the master"]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Swdatabe {
    #[inline(always)]
    fn default() -> Swdatabe {
        Swdatabe(0)
    }
}
#[doc = "Slave Write Data Half-word Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatah(pub u32);
impl Swdatah {
    #[doc = "The 1st byte to send to the master"]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The 1st byte to send to the master"]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The 2nd byte to send to the master"]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The 2nd byte to send to the master"]
    #[inline(always)]
    pub fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub const fn end(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "End of message"]
    #[inline(always)]
    pub fn set_end(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Swdatah {
    #[inline(always)]
    fn default() -> Swdatah {
        Swdatah(0)
    }
}
#[doc = "Slave Write Data Half-word End Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swdatahe(pub u32);
impl Swdatahe {
    #[doc = "The 1st byte to send to the master"]
    #[inline(always)]
    pub const fn data0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The 1st byte to send to the master"]
    #[inline(always)]
    pub fn set_data0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The 2nd byte to send to the master"]
    #[inline(always)]
    pub const fn data1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The 2nd byte to send to the master"]
    #[inline(always)]
    pub fn set_data1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Swdatahe {
    #[inline(always)]
    fn default() -> Swdatahe {
        Swdatahe(0)
    }
}
