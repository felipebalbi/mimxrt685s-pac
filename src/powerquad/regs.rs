#[doc = "Compute register bank"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compreg(pub u32);
impl Compreg {
    #[doc = "Compute register bank"]
    #[inline(always)]
    pub const fn compreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Compute register bank"]
    #[inline(always)]
    pub fn set_compreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Compreg {
    #[inline(always)]
    fn default() -> Compreg {
        Compreg(0)
    }
}
#[doc = "PowerQuad Control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Control(pub u32);
impl Control {
    #[doc = "opcode specific to decode_machine"]
    #[inline(always)]
    pub const fn decode_opcode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "opcode specific to decode_machine"]
    #[inline(always)]
    pub fn set_decode_opcode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
    #[inline(always)]
    pub const fn decode_machine(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "0 : Coprocessor , 1 : matrix , 2 : fft , 3 : fir , 4 : stat , 5 : cordic , 6 -15 : NA"]
    #[inline(always)]
    pub fn set_decode_machine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Instruction busy signal when high indicates processing is on"]
    #[inline(always)]
    pub const fn inst_busy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction busy signal when high indicates processing is on"]
    #[inline(always)]
    pub fn set_inst_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Control {
    #[inline(always)]
    fn default() -> Control {
        Control(0)
    }
}
#[doc = "Cordic input X register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicX(pub u32);
impl CordicX {
    #[doc = "Cordic input x"]
    #[inline(always)]
    pub const fn cordic_x(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input x"]
    #[inline(always)]
    pub fn set_cordic_x(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CordicX {
    #[inline(always)]
    fn default() -> CordicX {
        CordicX(0)
    }
}
#[doc = "Cordic input Y register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicY(pub u32);
impl CordicY {
    #[doc = "Cordic input y"]
    #[inline(always)]
    pub const fn cordic_y(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input y"]
    #[inline(always)]
    pub fn set_cordic_y(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CordicY {
    #[inline(always)]
    fn default() -> CordicY {
        CordicY(0)
    }
}
#[doc = "Cordic input Z register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CordicZ(pub u32);
impl CordicZ {
    #[doc = "Cordic input z"]
    #[inline(always)]
    pub const fn cordic_z(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Cordic input z"]
    #[inline(always)]
    pub fn set_cordic_z(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for CordicZ {
    #[inline(always)]
    fn default() -> CordicZ {
        CordicZ(0)
    }
}
#[doc = "Pre-scale register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cppre(pub u32);
impl Cppre {
    #[doc = "co-processor scaling of input"]
    #[inline(always)]
    pub const fn cppre_in(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "co-processor scaling of input"]
    #[inline(always)]
    pub fn set_cppre_in(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "co-processor fixed point output"]
    #[inline(always)]
    pub const fn cppre_out(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "co-processor fixed point output"]
    #[inline(always)]
    pub fn set_cppre_out(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub const fn cppre_sat(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "1 : forces sub-32 bit saturation"]
    #[inline(always)]
    pub fn set_cppre_sat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub const fn cppre_sat8(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "0 = 8bits, 1 = 16bits"]
    #[inline(always)]
    pub fn set_cppre_sat8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Cppre {
    #[inline(always)]
    fn default() -> Cppre {
        Cppre(0)
    }
}
#[doc = "Cursory register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cursory(pub u32);
impl Cursory {
    #[doc = "1 : Enable cursory mode"]
    #[inline(always)]
    pub const fn cursory(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable cursory mode"]
    #[inline(always)]
    pub fn set_cursory(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cursory {
    #[inline(always)]
    fn default() -> Cursory {
        Cursory(0)
    }
}
#[doc = "Read/Write register where error statuses are captured (sticky)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errstat(pub u32);
impl Errstat {
    #[doc = "overflow"]
    #[inline(always)]
    pub const fn overflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "overflow"]
    #[inline(always)]
    pub fn set_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "nan"]
    #[inline(always)]
    pub const fn nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "nan"]
    #[inline(always)]
    pub fn set_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "fixed_pt_overflow"]
    #[inline(always)]
    pub const fn fixedoverflow(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "fixed_pt_overflow"]
    #[inline(always)]
    pub fn set_fixedoverflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "underflow"]
    #[inline(always)]
    pub const fn underflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "underflow"]
    #[inline(always)]
    pub fn set_underflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "bus_error"]
    #[inline(always)]
    pub const fn buserror(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "bus_error"]
    #[inline(always)]
    pub fn set_buserror(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Errstat {
    #[inline(always)]
    fn default() -> Errstat {
        Errstat(0)
    }
}
#[doc = "Event Enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eventen(pub u32);
impl Eventen {
    #[doc = "1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    pub const fn event_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Floating point overflow"]
    #[inline(always)]
    pub fn set_event_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    pub const fn event_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Floating point NaN"]
    #[inline(always)]
    pub fn set_event_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    pub const fn event_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on Fixed point Overflow"]
    #[inline(always)]
    pub fn set_event_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    pub const fn event_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable event trigger on Subnormal truncation"]
    #[inline(always)]
    pub fn set_event_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    pub const fn event_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on AHBM Buss Error"]
    #[inline(always)]
    pub fn set_event_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1: Enable event trigger on instruction completion"]
    #[inline(always)]
    pub const fn event_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable event trigger on instruction completion"]
    #[inline(always)]
    pub fn set_event_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Eventen {
    #[inline(always)]
    fn default() -> Eventen {
        Eventen(0)
    }
}
#[doc = "General purpose register bank N."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpreg(pub u32);
impl Gpreg {
    #[doc = "General purpose register bank"]
    #[inline(always)]
    pub const fn gpreg(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "General purpose register bank"]
    #[inline(always)]
    pub fn set_gpreg(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Gpreg {
    #[inline(always)]
    fn default() -> Gpreg {
        Gpreg(0)
    }
}
#[doc = "Base address register for input A region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inabase(pub u32);
impl Inabase {
    #[doc = "Base address register for the input A region"]
    #[inline(always)]
    pub const fn inabase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the input A region"]
    #[inline(always)]
    pub fn set_inabase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Inabase {
    #[inline(always)]
    fn default() -> Inabase {
        Inabase(0)
    }
}
#[doc = "Input A format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inaformat(pub u32);
impl Inaformat {
    #[doc = "Input A Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn ina_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Input A Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn set_ina_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input A External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn ina_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Input A External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn set_ina_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Input A Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub const fn ina_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input A Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn set_ina_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Inaformat {
    #[inline(always)]
    fn default() -> Inaformat {
        Inaformat(0)
    }
}
#[doc = "Base address register for input B region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inbbase(pub u32);
impl Inbbase {
    #[doc = "Base address register for the input B region"]
    #[inline(always)]
    pub const fn inbbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the input B region"]
    #[inline(always)]
    pub fn set_inbbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Inbbase {
    #[inline(always)]
    fn default() -> Inbbase {
        Inbbase(0)
    }
}
#[doc = "Input B format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inbformat(pub u32);
impl Inbformat {
    #[doc = "Input B Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn inb_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Input B Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn set_inb_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Input B External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn inb_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Input B External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn set_inb_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Input B Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub const fn inb_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Input B Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn set_inb_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Inbformat {
    #[inline(always)]
    fn default() -> Inbformat {
        Inbformat(0)
    }
}
#[doc = "INTERRUPT enable register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intren(pub u32);
impl Intren {
    #[doc = "1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    pub const fn intr_oflow(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Floating point overflow"]
    #[inline(always)]
    pub fn set_intr_oflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    pub const fn intr_nan(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Floating point NaN"]
    #[inline(always)]
    pub fn set_intr_nan(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    pub const fn intr_fixed(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on Fixed point Overflow"]
    #[inline(always)]
    pub fn set_intr_fixed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    pub const fn intr_uflow(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1 : Enable interrupt on Subnormal truncation"]
    #[inline(always)]
    pub fn set_intr_uflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    pub const fn intr_berr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on AHBM Buss Error"]
    #[inline(always)]
    pub fn set_intr_berr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "1: Enable interrupt on instruction completion"]
    #[inline(always)]
    pub const fn intr_comp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable interrupt on instruction completion"]
    #[inline(always)]
    pub fn set_intr_comp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Intren {
    #[inline(always)]
    fn default() -> Intren {
        Intren(0)
    }
}
#[doc = "INTERRUPT STATUS register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intrstat(pub u32);
impl Intrstat {
    #[doc = "Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub const fn intr_stat(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Intr status ( 1 bit to indicate interrupt captured, 0 means no new interrupt), write any value will clear this bit"]
    #[inline(always)]
    pub fn set_intr_stat(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Intrstat {
    #[inline(always)]
    fn default() -> Intrstat {
        Intrstat(0)
    }
}
#[doc = "Length register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Length(pub u32);
impl Length {
    #[doc = "Length register. When FIR : fir_xlength = inst_length\\[15:0\\] , fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\] , cols_a = inst_length\\[12:8\\] , cols_b = inst_length\\[20:16\\]"]
    #[inline(always)]
    pub const fn inst_length(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Length register. When FIR : fir_xlength = inst_length\\[15:0\\] , fir_tlength = inst_len\\[31:16\\]. When MTX : rows_a = inst_length\\[4:0\\] , cols_a = inst_length\\[12:8\\] , cols_b = inst_length\\[20:16\\]"]
    #[inline(always)]
    pub fn set_inst_length(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Length {
    #[inline(always)]
    fn default() -> Length {
        Length(0)
    }
}
#[doc = "Misc register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Misc(pub u32);
impl Misc {
    #[doc = "Misc register. For Matrix : Used for scale factor"]
    #[inline(always)]
    pub const fn inst_misc(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Misc register. For Matrix : Used for scale factor"]
    #[inline(always)]
    pub fn set_inst_misc(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Misc {
    #[inline(always)]
    fn default() -> Misc {
        Misc(0)
    }
}
#[doc = "Base address register for output region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outbase(pub u32);
impl Outbase {
    #[doc = "Base address register for the output region"]
    #[inline(always)]
    pub const fn outbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the output region"]
    #[inline(always)]
    pub fn set_outbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Outbase {
    #[inline(always)]
    fn default() -> Outbase {
        Outbase(0)
    }
}
#[doc = "Output format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Outformat(pub u32);
impl Outformat {
    #[doc = "Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn out_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Output Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn set_out_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Output External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn out_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Output External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn set_out_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Output Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub const fn out_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Output Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn set_out_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Outformat {
    #[inline(always)]
    fn default() -> Outformat {
        Outformat(0)
    }
}
#[doc = "Base address register for temp region"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpbase(pub u32);
impl Tmpbase {
    #[doc = "Base address register for the temporary region"]
    #[inline(always)]
    pub const fn tmpbase(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Base address register for the temporary region"]
    #[inline(always)]
    pub fn set_tmpbase(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Tmpbase {
    #[inline(always)]
    fn default() -> Tmpbase {
        Tmpbase(0)
    }
}
#[doc = "Temp format"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tmpformat(pub u32);
impl Tmpformat {
    #[doc = "Temp Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn tmp_formatint(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Temp Internal format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn set_tmp_formatint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Temp External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub const fn tmp_formatext(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Temp External format (00: q15; 01:q31; 10:float)"]
    #[inline(always)]
    pub fn set_tmp_formatext(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Temp Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub const fn tmp_scaler(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Temp Scaler value (for scaled 'q31' formats)"]
    #[inline(always)]
    pub fn set_tmp_scaler(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Tmpformat {
    #[inline(always)]
    fn default() -> Tmpformat {
        Tmpformat(0)
    }
}
