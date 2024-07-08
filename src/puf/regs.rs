#[doc = "PUF Allow"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Allow(pub u32);
impl Allow {
    #[doc = "Allow Enroll"]
    #[inline(always)]
    pub const fn allowenroll(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Enroll"]
    #[inline(always)]
    pub fn set_allowenroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Allow Start"]
    #[inline(always)]
    pub const fn allowstart(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Start"]
    #[inline(always)]
    pub fn set_allowstart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Allow Set Key"]
    #[inline(always)]
    pub const fn allowsetkey(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Set Key"]
    #[inline(always)]
    pub fn set_allowsetkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Allow Get Key"]
    #[inline(always)]
    pub const fn allowgetkey(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Allow Get Key"]
    #[inline(always)]
    pub fn set_allowgetkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Allow {
    #[inline(always)]
    fn default() -> Allow {
        Allow(0)
    }
}
#[doc = "PUF Configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Block Enroll and Set Key Operation"]
    #[inline(always)]
    pub const fn blockenroll_setkey(&self) -> super::vals::BlockenrollSetkey {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::BlockenrollSetkey::from_bits(val as u8)
    }
    #[doc = "Block Enroll and Set Key Operation"]
    #[inline(always)]
    pub fn set_blockenroll_setkey(&mut self, val: super::vals::BlockenrollSetkey) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Block Key Output Data"]
    #[inline(always)]
    pub const fn blockkeyoutput(&self) -> super::vals::Blockkeyoutput {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Blockkeyoutput::from_bits(val as u8)
    }
    #[doc = "Block Key Output Data"]
    #[inline(always)]
    pub fn set_blockkeyoutput(&mut self, val: super::vals::Blockkeyoutput) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
#[doc = "PUF Code Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Codeinput(pub u32);
impl Codeinput {
    #[doc = "AC/KC Input Data"]
    #[inline(always)]
    pub const fn codein(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AC/KC Input Data"]
    #[inline(always)]
    pub fn set_codein(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Codeinput {
    #[inline(always)]
    fn default() -> Codeinput {
        Codeinput(0)
    }
}
#[doc = "PUF Code Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Codeoutput(pub u32);
impl Codeoutput {
    #[doc = "AC/KC Output Data"]
    #[inline(always)]
    pub const fn codeout(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "AC/KC Output Data"]
    #[inline(always)]
    pub fn set_codeout(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Codeoutput {
    #[inline(always)]
    fn default() -> Codeoutput {
        Codeoutput(0)
    }
}
#[doc = "PUF Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Zeroize. Begin Zeroize operation for Quiddikey and go to Error state"]
    #[inline(always)]
    pub const fn zeroize(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Zeroize. Begin Zeroize operation for Quiddikey and go to Error state"]
    #[inline(always)]
    pub fn set_zeroize(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enroll. Begin Enroll operation"]
    #[inline(always)]
    pub const fn enroll(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enroll. Begin Enroll operation"]
    #[inline(always)]
    pub fn set_enroll(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start. Begin Start operation"]
    #[inline(always)]
    pub const fn start(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start. Begin Start operation"]
    #[inline(always)]
    pub fn set_start(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set Intrinsic Key. Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub const fn generatekey(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set Intrinsic Key. Begin Set Intrinsic Key operation"]
    #[inline(always)]
    pub fn set_generatekey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Set Key. Begin Set User Key operation"]
    #[inline(always)]
    pub const fn setkey(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set Key. Begin Set User Key operation"]
    #[inline(always)]
    pub fn set_setkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Get Key. Begin Get Key operation"]
    #[inline(always)]
    pub const fn getkey(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Get Key. Begin Get Key operation"]
    #[inline(always)]
    pub fn set_getkey(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Index Block High"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdxblkH(pub u32);
impl IdxblkH {
    #[doc = "Index 8"]
    #[inline(always)]
    pub const fn idx8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Index 8"]
    #[inline(always)]
    pub fn set_idx8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Index 9"]
    #[inline(always)]
    pub const fn idx9(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Index 9"]
    #[inline(always)]
    pub fn set_idx9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Index 10"]
    #[inline(always)]
    pub const fn idx10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Index 10"]
    #[inline(always)]
    pub fn set_idx10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Index 11"]
    #[inline(always)]
    pub const fn idx11(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index 11"]
    #[inline(always)]
    pub fn set_idx11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Index 12"]
    #[inline(always)]
    pub const fn idx12(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Index 12"]
    #[inline(always)]
    pub fn set_idx12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Index 13"]
    #[inline(always)]
    pub const fn idx13(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Index 13"]
    #[inline(always)]
    pub fn set_idx13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Index 14"]
    #[inline(always)]
    pub const fn idx14(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Index 14"]
    #[inline(always)]
    pub fn set_idx14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Index 15"]
    #[inline(always)]
    pub const fn idx15(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Index 15"]
    #[inline(always)]
    pub fn set_idx15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Lock Index"]
    #[inline(always)]
    pub const fn lock_idx(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Lock Index"]
    #[inline(always)]
    pub fn set_lock_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for IdxblkH {
    #[inline(always)]
    fn default() -> IdxblkH {
        IdxblkH(0)
    }
}
#[doc = "Index Block High Duplicate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdxblkHDp(pub u32);
impl IdxblkHDp {
    #[doc = "Index 8"]
    #[inline(always)]
    pub const fn idx8(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Index 8"]
    #[inline(always)]
    pub fn set_idx8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Index 9"]
    #[inline(always)]
    pub const fn idx9(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Index 9"]
    #[inline(always)]
    pub fn set_idx9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Index 10"]
    #[inline(always)]
    pub const fn idx10(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Index 10"]
    #[inline(always)]
    pub fn set_idx10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Index 11"]
    #[inline(always)]
    pub const fn idx11(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index 11"]
    #[inline(always)]
    pub fn set_idx11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Index 12"]
    #[inline(always)]
    pub const fn idx12(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Index 12"]
    #[inline(always)]
    pub fn set_idx12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Index 13"]
    #[inline(always)]
    pub const fn idx13(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Index 13"]
    #[inline(always)]
    pub fn set_idx13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Index 14"]
    #[inline(always)]
    pub const fn idx14(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Index 14"]
    #[inline(always)]
    pub fn set_idx14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Index 15"]
    #[inline(always)]
    pub const fn idx15(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Index 15"]
    #[inline(always)]
    pub fn set_idx15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for IdxblkHDp {
    #[inline(always)]
    fn default() -> IdxblkHDp {
        IdxblkHDp(0)
    }
}
#[doc = "Index Block Low"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdxblkL(pub u32);
impl IdxblkL {
    #[doc = "Index 1"]
    #[inline(always)]
    pub const fn idx1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Index 1"]
    #[inline(always)]
    pub fn set_idx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Index 2"]
    #[inline(always)]
    pub const fn idx2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Index 2"]
    #[inline(always)]
    pub fn set_idx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Index 3"]
    #[inline(always)]
    pub const fn idx3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index 3"]
    #[inline(always)]
    pub fn set_idx3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Index 4"]
    #[inline(always)]
    pub const fn idx4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Index 4"]
    #[inline(always)]
    pub fn set_idx4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Index 5"]
    #[inline(always)]
    pub const fn idx5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Index 5"]
    #[inline(always)]
    pub fn set_idx5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Index 6"]
    #[inline(always)]
    pub const fn idx6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Index 6"]
    #[inline(always)]
    pub fn set_idx6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Index 7"]
    #[inline(always)]
    pub const fn idx7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Index 7"]
    #[inline(always)]
    pub fn set_idx7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[doc = "Lock Index"]
    #[inline(always)]
    pub const fn lock_idx(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Lock Index"]
    #[inline(always)]
    pub fn set_lock_idx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for IdxblkL {
    #[inline(always)]
    fn default() -> IdxblkL {
        IdxblkL(0)
    }
}
#[doc = "Index Block Low Duplicate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdxblkLDp(pub u32);
impl IdxblkLDp {
    #[doc = "Index 0"]
    #[inline(always)]
    pub const fn idx0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Index 0"]
    #[inline(always)]
    pub fn set_idx0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Index 1"]
    #[inline(always)]
    pub const fn idx1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Index 1"]
    #[inline(always)]
    pub fn set_idx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Index 2"]
    #[inline(always)]
    pub const fn idx2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Index 2"]
    #[inline(always)]
    pub fn set_idx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Index 3"]
    #[inline(always)]
    pub const fn idx3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Index 3"]
    #[inline(always)]
    pub fn set_idx3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Index 4"]
    #[inline(always)]
    pub const fn idx4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Index 4"]
    #[inline(always)]
    pub fn set_idx4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Index 5"]
    #[inline(always)]
    pub const fn idx5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Index 5"]
    #[inline(always)]
    pub fn set_idx5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Index 6"]
    #[inline(always)]
    pub const fn idx6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Index 6"]
    #[inline(always)]
    pub fn set_idx6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Index 7"]
    #[inline(always)]
    pub const fn idx7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Index 7"]
    #[inline(always)]
    pub fn set_idx7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
}
impl Default for IdxblkLDp {
    #[inline(always)]
    fn default() -> IdxblkLDp {
        IdxblkLDp(0)
    }
}
#[doc = "PUF Interface Status and Clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ifstat(pub u32);
impl Ifstat {
    #[doc = "Error"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ifstat {
    #[inline(always)]
    fn default() -> Ifstat {
        Ifstat(0)
    }
}
#[doc = "PUF Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inten(pub u32);
impl Inten {
    #[doc = "Enable corresponding interrupt in STAT, which indicates that the initialization or a operation is completed."]
    #[inline(always)]
    pub const fn readyen(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates that the initialization or a operation is completed."]
    #[inline(always)]
    pub fn set_readyen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates last operation was successful."]
    #[inline(always)]
    pub const fn succesen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates last operation was successful."]
    #[inline(always)]
    pub fn set_succesen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates that PUF is in the error state and no operations can be performed."]
    #[inline(always)]
    pub const fn erroren(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which indicates that PUF is in the error state and no operations can be performed."]
    #[inline(always)]
    pub fn set_erroren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which is request for next part of key."]
    #[inline(always)]
    pub const fn keyinreqen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which is request for next part of key."]
    #[inline(always)]
    pub fn set_keyinreqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which is next part of key is available."]
    #[inline(always)]
    pub const fn keyoutavailen(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which is next part of key is available."]
    #[inline(always)]
    pub fn set_keyoutavailen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which is request for next part of AC/KC."]
    #[inline(always)]
    pub const fn codeinreqen(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which is request for next part of AC/KC."]
    #[inline(always)]
    pub fn set_codeinreqen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Enable corresponding interrupt in STAT, which is next part of AC/KC is available."]
    #[inline(always)]
    pub const fn codeoutavailen(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Enable corresponding interrupt in STAT, which is next part of AC/KC is available."]
    #[inline(always)]
    pub fn set_codeoutavailen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Inten {
    #[inline(always)]
    fn default() -> Inten {
        Inten(0)
    }
}
#[doc = "PUF Interrupt Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Ready"]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Success"]
    #[inline(always)]
    pub const fn success(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Success"]
    #[inline(always)]
    pub fn set_success(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Key In Request"]
    #[inline(always)]
    pub const fn keyinreq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Key In Request"]
    #[inline(always)]
    pub fn set_keyinreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Key Out Available"]
    #[inline(always)]
    pub const fn keyoutavail(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key Out Available"]
    #[inline(always)]
    pub fn set_keyoutavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Code In Request"]
    #[inline(always)]
    pub const fn codeinreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Code In Request"]
    #[inline(always)]
    pub fn set_codeinreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Code Out Available"]
    #[inline(always)]
    pub const fn codeoutavail(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Code Out Available"]
    #[inline(always)]
    pub fn set_codeoutavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
#[doc = "Key Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyenable(pub u32);
impl Keyenable {
    #[doc = "Key 0"]
    #[inline(always)]
    pub const fn key0(&self) -> super::vals::KeyenableKey0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KeyenableKey0::from_bits(val as u8)
    }
    #[doc = "Key 0"]
    #[inline(always)]
    pub fn set_key0(&mut self, val: super::vals::KeyenableKey0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub const fn key1(&self) -> super::vals::KeyenableKey1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::KeyenableKey1::from_bits(val as u8)
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub fn set_key1(&mut self, val: super::vals::KeyenableKey1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub const fn key2(&self) -> super::vals::KeyenableKey2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::KeyenableKey2::from_bits(val as u8)
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub fn set_key2(&mut self, val: super::vals::KeyenableKey2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub const fn key3(&self) -> super::vals::KeyenableKey3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::KeyenableKey3::from_bits(val as u8)
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub fn set_key3(&mut self, val: super::vals::KeyenableKey3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for Keyenable {
    #[inline(always)]
    fn default() -> Keyenable {
        Keyenable(0)
    }
}
#[doc = "PUF Key Index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyindex(pub u32);
impl Keyindex {
    #[doc = "Key index for Set Key operations"]
    #[inline(always)]
    pub const fn keyidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Key index for Set Key operations"]
    #[inline(always)]
    pub fn set_keyidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Keyindex {
    #[inline(always)]
    fn default() -> Keyindex {
        Keyindex(0)
    }
}
#[doc = "PUF Key Input"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyinput(pub u32);
impl Keyinput {
    #[doc = "Key Input Data"]
    #[inline(always)]
    pub const fn keyin(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key Input Data"]
    #[inline(always)]
    pub fn set_keyin(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyinput {
    #[inline(always)]
    fn default() -> Keyinput {
        Keyinput(0)
    }
}
#[doc = "Key Lock"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keylock(pub u32);
impl Keylock {
    #[doc = "Key 0"]
    #[inline(always)]
    pub const fn key0(&self) -> super::vals::KeylockKey0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KeylockKey0::from_bits(val as u8)
    }
    #[doc = "Key 0"]
    #[inline(always)]
    pub fn set_key0(&mut self, val: super::vals::KeylockKey0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub const fn key1(&self) -> super::vals::KeylockKey1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::KeylockKey1::from_bits(val as u8)
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub fn set_key1(&mut self, val: super::vals::KeylockKey1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub const fn key2(&self) -> super::vals::KeylockKey2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::KeylockKey2::from_bits(val as u8)
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub fn set_key2(&mut self, val: super::vals::KeylockKey2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub const fn key3(&self) -> super::vals::KeylockKey3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::KeylockKey3::from_bits(val as u8)
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub fn set_key3(&mut self, val: super::vals::KeylockKey3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for Keylock {
    #[inline(always)]
    fn default() -> Keylock {
        Keylock(0)
    }
}
#[doc = "Key Mask x"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keymask(pub u32);
impl Keymask {
    #[doc = "Key a Mask"]
    #[inline(always)]
    pub const fn keymask(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key a Mask"]
    #[inline(always)]
    pub fn set_keymask(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keymask {
    #[inline(always)]
    fn default() -> Keymask {
        Keymask(0)
    }
}
#[doc = "PUF Key Output Index"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyoutindex(pub u32);
impl Keyoutindex {
    #[doc = "Key Output Index"]
    #[inline(always)]
    pub const fn keyoutidx(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Key Output Index"]
    #[inline(always)]
    pub fn set_keyoutidx(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Keyoutindex {
    #[inline(always)]
    fn default() -> Keyoutindex {
        Keyoutindex(0)
    }
}
#[doc = "PUF Key Output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyoutput(pub u32);
impl Keyoutput {
    #[doc = "Key Output Data"]
    #[inline(always)]
    pub const fn keyout(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Key Output Data"]
    #[inline(always)]
    pub fn set_keyout(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Keyoutput {
    #[inline(always)]
    fn default() -> Keyoutput {
        Keyoutput(0)
    }
}
#[doc = "Key Reset"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyreset(pub u32);
impl Keyreset {
    #[doc = "Key 0"]
    #[inline(always)]
    pub const fn key0(&self) -> super::vals::KeyresetKey0 {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::KeyresetKey0::from_bits(val as u8)
    }
    #[doc = "Key 0"]
    #[inline(always)]
    pub fn set_key0(&mut self, val: super::vals::KeyresetKey0) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub const fn key1(&self) -> super::vals::KeyresetKey1 {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::KeyresetKey1::from_bits(val as u8)
    }
    #[doc = "Key 1"]
    #[inline(always)]
    pub fn set_key1(&mut self, val: super::vals::KeyresetKey1) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub const fn key2(&self) -> super::vals::KeyresetKey2 {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::KeyresetKey2::from_bits(val as u8)
    }
    #[doc = "Key 2"]
    #[inline(always)]
    pub fn set_key2(&mut self, val: super::vals::KeyresetKey2) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub const fn key3(&self) -> super::vals::KeyresetKey3 {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::KeyresetKey3::from_bits(val as u8)
    }
    #[doc = "Key 3"]
    #[inline(always)]
    pub fn set_key3(&mut self, val: super::vals::KeyresetKey3) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
}
impl Default for Keyreset {
    #[inline(always)]
    fn default() -> Keyreset {
        Keyreset(0)
    }
}
#[doc = "PUF Key Size"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keysize(pub u32);
impl Keysize {
    #[doc = "Key Size for Set Key operations"]
    #[inline(always)]
    pub const fn keysize(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Key Size for Set Key operations"]
    #[inline(always)]
    pub fn set_keysize(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Keysize {
    #[inline(always)]
    fn default() -> Keysize {
        Keysize(0)
    }
}
#[doc = "PUF Power Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrctrl(pub u32);
impl Pwrctrl {
    #[doc = "RAM Power On"]
    #[inline(always)]
    pub const fn ram_on(&self) -> super::vals::RamOn {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::RamOn::from_bits(val as u8)
    }
    #[doc = "RAM Power On"]
    #[inline(always)]
    pub fn set_ram_on(&mut self, val: super::vals::RamOn) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "PUF Clock control."]
    #[inline(always)]
    pub const fn ck_dis(&self) -> super::vals::CkDis {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::CkDis::from_bits(val as u8)
    }
    #[doc = "PUF Clock control."]
    #[inline(always)]
    pub fn set_ck_dis(&mut self, val: super::vals::CkDis) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Pwrctrl {
    #[inline(always)]
    fn default() -> Pwrctrl {
        Pwrctrl(0)
    }
}
#[doc = "PUF Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Busy"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Success"]
    #[inline(always)]
    pub const fn success(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Success"]
    #[inline(always)]
    pub fn set_success(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Error"]
    #[inline(always)]
    pub const fn error(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Error"]
    #[inline(always)]
    pub fn set_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Key In Request"]
    #[inline(always)]
    pub const fn keyinreq(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Key In Request"]
    #[inline(always)]
    pub fn set_keyinreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Key Out Available"]
    #[inline(always)]
    pub const fn keyoutavail(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Key Out Available"]
    #[inline(always)]
    pub fn set_keyoutavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Code In Request"]
    #[inline(always)]
    pub const fn codeinreq(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Code In Request"]
    #[inline(always)]
    pub fn set_codeinreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Code Out Available"]
    #[inline(always)]
    pub const fn codeoutavail(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Code Out Available"]
    #[inline(always)]
    pub fn set_codeoutavail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
#[doc = "PUF Version"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Version(pub u32);
impl Version {
    #[doc = "Version"]
    #[inline(always)]
    pub const fn version(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Version"]
    #[inline(always)]
    pub fn set_version(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Version {
    #[inline(always)]
    fn default() -> Version {
        Version(0)
    }
}
