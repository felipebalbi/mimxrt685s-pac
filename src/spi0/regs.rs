#[doc = "SPI Configuration register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "SPI enable."]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enable::from_bits(val as u8)
    }
    #[doc = "SPI enable."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Master mode select."]
    #[inline(always)]
    pub const fn master(&self) -> super::vals::Master {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Master::from_bits(val as u8)
    }
    #[doc = "Master mode select."]
    #[inline(always)]
    pub fn set_master(&mut self, val: super::vals::Master) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "LSB First mode enable."]
    #[inline(always)]
    pub const fn lsbf(&self) -> super::vals::Lsbf {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Lsbf::from_bits(val as u8)
    }
    #[doc = "LSB First mode enable."]
    #[inline(always)]
    pub fn set_lsbf(&mut self, val: super::vals::Lsbf) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Clock Phase select."]
    #[inline(always)]
    pub const fn cpha(&self) -> super::vals::Cpha {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Cpha::from_bits(val as u8)
    }
    #[doc = "Clock Phase select."]
    #[inline(always)]
    pub fn set_cpha(&mut self, val: super::vals::Cpha) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Clock Polarity select."]
    #[inline(always)]
    pub const fn cpol(&self) -> super::vals::Cpol {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Cpol::from_bits(val as u8)
    }
    #[doc = "Clock Polarity select."]
    #[inline(always)]
    pub fn set_cpol(&mut self, val: super::vals::Cpol) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub const fn loop_(&self) -> super::vals::Loop {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Loop::from_bits(val as u8)
    }
    #[doc = "Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub fn set_loop_(&mut self, val: super::vals::Loop) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "SSEL0 Polarity select."]
    #[inline(always)]
    pub const fn spol0(&self) -> super::vals::Spol0 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Spol0::from_bits(val as u8)
    }
    #[doc = "SSEL0 Polarity select."]
    #[inline(always)]
    pub fn set_spol0(&mut self, val: super::vals::Spol0) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "SSEL1 Polarity select."]
    #[inline(always)]
    pub const fn spol1(&self) -> super::vals::Spol1 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Spol1::from_bits(val as u8)
    }
    #[doc = "SSEL1 Polarity select."]
    #[inline(always)]
    pub fn set_spol1(&mut self, val: super::vals::Spol1) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "SSEL2 Polarity select."]
    #[inline(always)]
    pub const fn spol2(&self) -> super::vals::Spol2 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Spol2::from_bits(val as u8)
    }
    #[doc = "SSEL2 Polarity select."]
    #[inline(always)]
    pub fn set_spol2(&mut self, val: super::vals::Spol2) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "SSEL3 Polarity select."]
    #[inline(always)]
    pub const fn spol3(&self) -> super::vals::Spol3 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Spol3::from_bits(val as u8)
    }
    #[doc = "SSEL3 Polarity select."]
    #[inline(always)]
    pub fn set_spol3(&mut self, val: super::vals::Spol3) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
#[doc = "SPI clock Divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div(pub u32);
impl Div {
    #[doc = "Rate divider value. Specifies how the Flexcomm clock (FCLK) is divided to produce the SPI clock rate in master mode. DIVVAL is -1 encoded such that the value 0 results in FCLK/1, the value 1 results in FCLK/2, up to the maximum possible divide value of 0xFFFF, which results in FCLK/65536."]
    #[inline(always)]
    pub const fn divval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Rate divider value. Specifies how the Flexcomm clock (FCLK) is divided to produce the SPI clock rate in master mode. DIVVAL is -1 encoded such that the value 0 results in FCLK/1, the value 1 results in FCLK/2, up to the maximum possible divide value of 0xFFFF, which results in FCLK/65536."]
    #[inline(always)]
    pub fn set_divval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Div {
    #[inline(always)]
    fn default() -> Div {
        Div(0)
    }
}
#[doc = "SPI Delay register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dly(pub u32);
impl Dly {
    #[doc = "Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn pre_delay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn set_pre_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn post_delay(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn set_post_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub const fn frame_delay(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn set_frame_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub const fn transfer_delay(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub fn set_transfer_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
}
impl Default for Dly {
    #[inline(always)]
    fn default() -> Dly {
        Dly(0)
    }
}
#[doc = "FIFO configuration and enable register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifocfg(pub u32);
impl Fifocfg {
    #[doc = "Enable the transmit FIFO."]
    #[inline(always)]
    pub const fn enabletx(&self) -> super::vals::Enabletx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Enabletx::from_bits(val as u8)
    }
    #[doc = "Enable the transmit FIFO."]
    #[inline(always)]
    pub fn set_enabletx(&mut self, val: super::vals::Enabletx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the receive FIFO."]
    #[inline(always)]
    pub const fn enablerx(&self) -> super::vals::Enablerx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Enablerx::from_bits(val as u8)
    }
    #[doc = "Enable the receive FIFO."]
    #[inline(always)]
    pub fn set_enablerx(&mut self, val: super::vals::Enablerx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline(always)]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "FIFO size configuration. This is a read-only field. 0x0 = FIFO is configured as 16 entries of 8 bits. 0x1, 0x2, 0x3 = not applicable to USART."]
    #[inline(always)]
    pub fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "DMA configuration for transmit."]
    #[inline(always)]
    pub const fn dmatx(&self) -> super::vals::Dmatx {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmatx::from_bits(val as u8)
    }
    #[doc = "DMA configuration for transmit."]
    #[inline(always)]
    pub fn set_dmatx(&mut self, val: super::vals::Dmatx) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "DMA configuration for receive."]
    #[inline(always)]
    pub const fn dmarx(&self) -> super::vals::Dmarx {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmarx::from_bits(val as u8)
    }
    #[doc = "DMA configuration for receive."]
    #[inline(always)]
    pub fn set_dmarx(&mut self, val: super::vals::Dmarx) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub const fn waketx(&self) -> super::vals::Waketx {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Waketx::from_bits(val as u8)
    }
    #[doc = "Wake-up for transmit FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn set_waketx(&mut self, val: super::vals::Waketx) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub const fn wakerx(&self) -> super::vals::Wakerx {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Wakerx::from_bits(val as u8)
    }
    #[doc = "Wake-up for receive FIFO level. This allows the device to be woken from reduced power modes (up to power-down, as long as the peripheral function works in that power mode) without enabling the TXLVL interrupt. Only DMA wakes up, processes data, and goes back to sleep. The CPU will remain stopped until woken by another cause, such as DMA completion. See Hardware Wake-up control register."]
    #[inline(always)]
    pub fn set_wakerx(&mut self, val: super::vals::Wakerx) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub const fn emptytx(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Empty command for the transmit FIFO. When a 1 is written to this bit, the TX FIFO is emptied."]
    #[inline(always)]
    pub fn set_emptytx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub const fn emptyrx(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Empty command for the receive FIFO. When a 1 is written to this bit, the RX FIFO is emptied."]
    #[inline(always)]
    pub fn set_emptyrx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Fifocfg {
    #[inline(always)]
    fn default() -> Fifocfg {
        Fifocfg(0)
    }
}
#[doc = "FIFO interrupt enable clear (disable) and read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifointenclr(pub u32);
impl Fifointenclr {
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn txerr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn set_txerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn rxerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn set_rxerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn txlvl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn set_txlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub const fn rxlvl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn set_rxlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Fifointenclr {
    #[inline(always)]
    fn default() -> Fifointenclr {
        Fifointenclr(0)
    }
}
#[doc = "FIFO interrupt enable set (enable) and read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifointenset(pub u32);
impl Fifointenset {
    #[doc = "Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub const fn txerr(&self) -> super::vals::Txerr {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Txerr::from_bits(val as u8)
    }
    #[doc = "Determines whether an interrupt occurs when a transmit error occurs, based on the TXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn set_txerr(&mut self, val: super::vals::Txerr) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub const fn rxerr(&self) -> super::vals::Rxerr {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxerr::from_bits(val as u8)
    }
    #[doc = "Determines whether an interrupt occurs when a receive error occurs, based on the RXERR flag in the FIFOSTAT register."]
    #[inline(always)]
    pub fn set_rxerr(&mut self, val: super::vals::Rxerr) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub const fn txlvl(&self) -> super::vals::Txlvl {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Txlvl::from_bits(val as u8)
    }
    #[doc = "Determines whether an interrupt occurs when a the transmit FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn set_txlvl(&mut self, val: super::vals::Txlvl) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub const fn rxlvl(&self) -> super::vals::Rxlvl {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Rxlvl::from_bits(val as u8)
    }
    #[doc = "Determines whether an interrupt occurs when a the receive FIFO reaches the level specified by the TXLVL field in the FIFOTRIG register."]
    #[inline(always)]
    pub fn set_rxlvl(&mut self, val: super::vals::Rxlvl) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Fifointenset {
    #[inline(always)]
    fn default() -> Fifointenset {
        Fifointenset(0)
    }
}
#[doc = "FIFO interrupt status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifointstat(pub u32);
impl Fifointstat {
    #[doc = "TX FIFO error."]
    #[inline(always)]
    pub const fn txerr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO error."]
    #[inline(always)]
    pub fn set_txerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO error."]
    #[inline(always)]
    pub const fn rxerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO error."]
    #[inline(always)]
    pub fn set_rxerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO level interrupt."]
    #[inline(always)]
    pub const fn txlvl(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO level interrupt."]
    #[inline(always)]
    pub fn set_txlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO level interrupt."]
    #[inline(always)]
    pub const fn rxlvl(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO level interrupt."]
    #[inline(always)]
    pub fn set_rxlvl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Peripheral interrupt."]
    #[inline(always)]
    pub const fn perint(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral interrupt."]
    #[inline(always)]
    pub fn set_perint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Fifointstat {
    #[inline(always)]
    fn default() -> Fifointstat {
        Fifointstat(0)
    }
}
#[doc = "FIFO read data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fiford(pub u32);
impl Fiford {
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub const fn rxdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub fn set_rxdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn rxssel0_n(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL0 pin to be saved along with received data. The value will reflect the SSEL0 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn set_rxssel0_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn rxssel1_n(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL1 pin to be saved along with received data. The value will reflect the SSEL1 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn set_rxssel1_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn rxssel2_n(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL2 pin to be saved along with received data. The value will reflect the SSEL2 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn set_rxssel2_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub const fn rxssel3_n(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive. This field allows the state of the SSEL3 pin to be saved along with received data. The value will reflect the SSEL3 pin for both master and slave operation. A zero indicates that a slave select is active. The actual polarity of each slave select pin is configured by the related SPOL bit in CFG."]
    #[inline(always)]
    pub fn set_rxssel3_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[inline(always)]
    pub const fn sot(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Start of Transfer flag. This flag will be 1 if this is the first data after the SSELs went from deasserted to asserted (i.e., any previous transfer has ended). This information can be used to identify the first piece of data in cases where the transfer length is greater than 16 bits."]
    #[inline(always)]
    pub fn set_sot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Fiford {
    #[inline(always)]
    fn default() -> Fiford {
        Fiford(0)
    }
}
#[doc = "FIFO data read with no FIFO pop."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifordnopop(pub u32);
impl Fifordnopop {
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub const fn rxdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Received data from the FIFO."]
    #[inline(always)]
    pub fn set_rxdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn rxssel0_n(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub fn set_rxssel0_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn rxssel1_n(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub fn set_rxssel1_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn rxssel2_n(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub fn set_rxssel2_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub const fn rxssel3_n(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select for receive."]
    #[inline(always)]
    pub fn set_rxssel3_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Start of transfer flag."]
    #[inline(always)]
    pub const fn sot(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Start of transfer flag."]
    #[inline(always)]
    pub fn set_sot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Fifordnopop {
    #[inline(always)]
    fn default() -> Fifordnopop {
        Fifordnopop(0)
    }
}
#[doc = "FIFO size register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifosize(pub u32);
impl Fifosize {
    #[doc = "the fifo size is equal to the template parameter \"fifo\"/2 ."]
    #[inline(always)]
    pub const fn fifosize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "the fifo size is equal to the template parameter \"fifo\"/2 ."]
    #[inline(always)]
    pub fn set_fifosize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Fifosize {
    #[inline(always)]
    fn default() -> Fifosize {
        Fifosize(0)
    }
}
#[doc = "FIFO status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifostat(pub u32);
impl Fifostat {
    #[doc = "TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn txerr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "TX FIFO error. Will be set if a transmit FIFO error occurs. This could be an overflow caused by pushing data into a full FIFO, or by an underflow if the FIFO is empty when data is needed. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn set_txerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn rxerr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "RX FIFO error. Will be set if a receive FIFO overflow occurs, caused by software or DMA not emptying the FIFO fast enough. Cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn set_rxerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
    #[inline(always)]
    pub const fn perint(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Peripheral interrupt. When 1, this indicates that the peripheral function has asserted an interrupt. The details can be found by reading the peripheral's STAT register."]
    #[inline(always)]
    pub fn set_perint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
    #[inline(always)]
    pub const fn txempty(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty. When 1, the transmit FIFO is empty. The peripheral may still be processing the last piece of data."]
    #[inline(always)]
    pub fn set_txempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
    #[inline(always)]
    pub const fn txnotfull(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full. When 1, the transmit FIFO is not full, so more data can be written. When 0, the transmit FIFO is full and another write would cause it to overflow."]
    #[inline(always)]
    pub fn set_txnotfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
    #[inline(always)]
    pub const fn rxnotempty(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty. When 1, the receive FIFO is not empty, so data can be read. When 0, the receive FIFO is empty."]
    #[inline(always)]
    pub fn set_rxnotempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full. When 1, the receive FIFO is full. Data needs to be read out to prevent the peripheral from causing an overflow."]
    #[inline(always)]
    pub fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
    #[inline(always)]
    pub const fn txlvl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO current level. A 0 means the TX FIFO is currently empty, and the TXEMPTY and TXNOTFULL flags will be 1. Other values tell how much data is actually in the TX FIFO at the point where the read occurs. If the TX FIFO is full, the TXEMPTY and TXNOTFULL flags will be 0."]
    #[inline(always)]
    pub fn set_txlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
    #[inline(always)]
    pub const fn rxlvl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Receive FIFO current level. A 0 means the RX FIFO is currently empty, and the RXFULL and RXNOTEMPTY flags will be 0. Other values tell how much data is actually in the RX FIFO at the point where the read occurs. If the RX FIFO is full, the RXFULL and RXNOTEMPTY flags will be 1."]
    #[inline(always)]
    pub fn set_rxlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
}
impl Default for Fifostat {
    #[inline(always)]
    fn default() -> Fifostat {
        Fifostat(0)
    }
}
#[doc = "FIFO trigger settings for interrupt and DMA request."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifotrig(pub u32);
impl Fifotrig {
    #[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub const fn txlvlena(&self) -> super::vals::Txlvlena {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Txlvlena::from_bits(val as u8)
    }
    #[doc = "Transmit FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMATX in FIFOCFG is set."]
    #[inline(always)]
    pub fn set_txlvlena(&mut self, val: super::vals::Txlvlena) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub const fn rxlvlena(&self) -> super::vals::Rxlvlena {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Rxlvlena::from_bits(val as u8)
    }
    #[doc = "Receive FIFO level trigger enable. This trigger will become an interrupt if enabled in FIFOINTENSET, or a DMA trigger if DMARX in FIFOCFG is set."]
    #[inline(always)]
    pub fn set_rxlvlena(&mut self, val: super::vals::Rxlvlena) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub const fn txlvl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Transmit FIFO level trigger point. This field is used only when TXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the TX FIFO becomes empty. 1 = trigger when the TX FIFO level decreases to one entry. 15 = trigger when the TX FIFO level decreases to 15 entries (is no longer full)."]
    #[inline(always)]
    pub fn set_txlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub const fn rxlvl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Receive FIFO level trigger point. The RX FIFO level is checked when a new piece of data is received. This field is used only when RXLVLENA = 1. If enabled to do so, the FIFO level can wake up the device just enough to perform DMA, then return to the reduced power mode. See Hardware Wake-up control register. 0 = trigger when the RX FIFO has received one entry (is no longer empty). 1 = trigger when the RX FIFO has received two entries. 15 = trigger when the RX FIFO has received 16 entries (has become full)."]
    #[inline(always)]
    pub fn set_rxlvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for Fifotrig {
    #[inline(always)]
    fn default() -> Fifotrig {
        Fifotrig(0)
    }
}
#[doc = "FIFO write data."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifowr(pub u32);
impl Fifowr {
    #[doc = "Transmit data to the FIFO."]
    #[inline(always)]
    pub const fn txdata(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit data to the FIFO."]
    #[inline(always)]
    pub fn set_txdata(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn txssel0_n(&self) -> super::vals::Txssel0N {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Txssel0N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL0 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn set_txssel0_n(&mut self, val: super::vals::Txssel0N) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn txssel1_n(&self) -> super::vals::Txssel1N {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Txssel1N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL1 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn set_txssel1_n(&mut self, val: super::vals::Txssel1N) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn txssel2_n(&self) -> super::vals::Txssel2N {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Txssel2N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL2 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn set_txssel2_n(&mut self, val: super::vals::Txssel2N) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub const fn txssel3_n(&self) -> super::vals::Txssel3N {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Txssel3N::from_bits(val as u8)
    }
    #[doc = "Transmit slave select. This field asserts SSEL3 in master mode. The output on the pin is active LOW by default."]
    #[inline(always)]
    pub fn set_txssel3_n(&mut self, val: super::vals::Txssel3N) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    pub const fn eot(&self) -> super::vals::Eot {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Eot::from_bits(val as u8)
    }
    #[doc = "End of transfer. The asserted SSEL will be deasserted at the end of a transfer and remain so far at least the time specified by the Transfer_delay value in the DLY register."]
    #[inline(always)]
    pub fn set_eot(&mut self, val: super::vals::Eot) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    pub const fn eof(&self) -> super::vals::Eof {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Eof::from_bits(val as u8)
    }
    #[doc = "End of frame. Between frames, a delay may be inserted, as defined by the Frame_delay value in the DLY register. The end of a frame may not be particularly meaningful if the Frame_delay value = 0. This control can be used as part of the support for frame lengths greater than 16 bits."]
    #[inline(always)]
    pub fn set_eof(&mut self, val: super::vals::Eof) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    pub const fn rxignore(&self) -> super::vals::Rxignore {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Rxignore::from_bits(val as u8)
    }
    #[doc = "Receive Ignore. This allows data to be transmitted using the SPI without the need to read unneeded data from the receiver. Setting this bit simplifies the transmit process and can be used with the DMA."]
    #[inline(always)]
    pub fn set_rxignore(&mut self, val: super::vals::Rxignore) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "Transmit Ignore"]
    #[inline(always)]
    pub const fn txignore(&self) -> super::vals::Txignore {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Txignore::from_bits(val as u8)
    }
    #[doc = "Transmit Ignore"]
    #[inline(always)]
    pub fn set_txignore(&mut self, val: super::vals::Txignore) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    pub const fn len(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Length. Specifies the data length from 4 to 16 bits. Note that transfer lengths greater than 16 bits are supported by implementing multiple sequential transmits. 0x0-2 = Reserved. 0x3 = Data transfer is 4 bits in length. 0x4 = Data transfer is 5 bits in length. 0xF = Data transfer is 16 bits in length."]
    #[inline(always)]
    pub fn set_len(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Fifowr {
    #[inline(always)]
    fn default() -> Fifowr {
        Fifowr(0)
    }
}
#[doc = "Peripheral identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub const fn aperture(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub fn set_aperture(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn ssaen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn set_ssaen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn ssden(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn set_ssden(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub const fn mstidle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 clears the corresponding bit in the INTENSET register."]
    #[inline(always)]
    pub fn set_mstidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Intenclr {
    #[inline(always)]
    fn default() -> Intenclr {
        Intenclr(0)
    }
}
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc = "Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub const fn ssaen(&self) -> super::vals::Ssaen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Ssaen::from_bits(val as u8)
    }
    #[doc = "Slave select assert interrupt enable. Determines whether an interrupt occurs when the Slave Select is asserted."]
    #[inline(always)]
    pub fn set_ssaen(&mut self, val: super::vals::Ssaen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub const fn ssden(&self) -> super::vals::Ssden {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Ssden::from_bits(val as u8)
    }
    #[doc = "Slave select deassert interrupt enable. Determines whether an interrupt occurs when the Slave Select is deasserted."]
    #[inline(always)]
    pub fn set_ssden(&mut self, val: super::vals::Ssden) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "Master idle interrupt enable."]
    #[inline(always)]
    pub const fn mstidleen(&self) -> super::vals::Mstidleen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Mstidleen::from_bits(val as u8)
    }
    #[doc = "Master idle interrupt enable."]
    #[inline(always)]
    pub fn set_mstidleen(&mut self, val: super::vals::Mstidleen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
}
impl Default for Intenset {
    #[inline(always)]
    fn default() -> Intenset {
        Intenset(0)
    }
}
#[doc = "SPI Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Slave Select Assert."]
    #[inline(always)]
    pub const fn ssa(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Assert."]
    #[inline(always)]
    pub fn set_ssa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave Select Deassert."]
    #[inline(always)]
    pub const fn ssd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Deassert."]
    #[inline(always)]
    pub fn set_ssd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Master Idle status flag."]
    #[inline(always)]
    pub const fn mstidle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master Idle status flag."]
    #[inline(always)]
    pub fn set_mstidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Slave Select Assert. This flag is set whenever any slave select transitions from deasserted to asserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become busy, and allows waking up the device from reduced power modes when a slave mode access begins. This flag is cleared by software."]
    #[inline(always)]
    pub const fn ssa(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Assert. This flag is set whenever any slave select transitions from deasserted to asserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become busy, and allows waking up the device from reduced power modes when a slave mode access begins. This flag is cleared by software."]
    #[inline(always)]
    pub fn set_ssa(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Slave Select Deassert. This flag is set whenever any asserted slave selects transition to deasserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become idle. This flag is cleared by software."]
    #[inline(always)]
    pub const fn ssd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Select Deassert. This flag is set whenever any asserted slave selects transition to deasserted, in both master and slave modes. This allows determining when the SPI transmit/receive functions become idle. This flag is cleared by software."]
    #[inline(always)]
    pub fn set_ssd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Stalled status flag. This indicates whether the SPI is currently in a stall condition."]
    #[inline(always)]
    pub const fn stalled(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Stalled status flag. This indicates whether the SPI is currently in a stall condition."]
    #[inline(always)]
    pub fn set_stalled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
    #[inline(always)]
    pub const fn endtransfer(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "End Transfer control bit. Software can set this bit to force an end to the current transfer when the transmitter finishes any activity already in progress, as if the EOT flag had been set prior to the last transmission. This capability is included to support cases where it is not known when transmit data is written that it will be the end of a transfer. The bit is cleared when the transmitter becomes idle as the transfer comes to an end. Forcing an end of transfer in this manner causes any specified FRAME_DELAY and TRANSFER_DELAY to be inserted."]
    #[inline(always)]
    pub fn set_endtransfer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Master idle status flag. This bit is 1 whenever the SPI master function is fully idle. This means that the transmit holding register is empty and the transmitter is not in the process of sending data."]
    #[inline(always)]
    pub const fn mstidle(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Master idle status flag. This bit is 1 whenever the SPI master function is fully idle. This means that the transmit holding register is empty and the transmitter is not in the process of sending data."]
    #[inline(always)]
    pub fn set_mstidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
