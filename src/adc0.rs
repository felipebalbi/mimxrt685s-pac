#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    verid: Verid,
    param: Param,
    _reserved2: [u8; 0x08],
    ctrl: Ctrl,
    stat: Stat,
    ie: Ie,
    de: De,
    cfg: Cfg,
    pause: Pause,
    _reserved8: [u8; 0x08],
    fctrl: Fctrl,
    swtrig: Swtrig,
    _reserved10: [u8; 0x88],
    tctrl: [Tctrl; 16],
    cmdl1: Cmdl1,
    cmdh1: Cmdh1,
    cmdl2: Cmdl2,
    cmdh2: Cmdh2,
    cmdl3: Cmdl3,
    cmdh3: Cmdh3,
    cmdl4: Cmdl4,
    cmdh4: Cmdh4,
    cmdl5: Cmdl5,
    cmdh5: Cmdh5,
    cmdl6: Cmdl6,
    cmdh6: Cmdh6,
    cmdl7: Cmdl7,
    cmdh7: Cmdh7,
    cmdl8: Cmdl8,
    cmdh8: Cmdh8,
    cmdl9: Cmdl9,
    cmdh9: Cmdh9,
    cmdl10: Cmdl10,
    cmdh10: Cmdh10,
    cmdl11: Cmdl11,
    cmdh11: Cmdh11,
    cmdl12: Cmdl12,
    cmdh12: Cmdh12,
    cmdl13: Cmdl13,
    cmdh13: Cmdh13,
    cmdl14: Cmdl14,
    cmdh14: Cmdh14,
    cmdl15: Cmdl15,
    cmdh15: Cmdh15,
    _reserved41: [u8; 0x88],
    cv: [Cv; 4],
    _reserved42: [u8; 0xf0],
    resfifo: Resfifo,
}
impl RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
    #[doc = "0x04 - Parameter Register"]
    #[inline(always)]
    pub const fn param(&self) -> &Param {
        &self.param
    }
    #[doc = "0x10 - ADC Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x14 - ADC Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x18 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x1c - DMA Enable Register"]
    #[inline(always)]
    pub const fn de(&self) -> &De {
        &self.de
    }
    #[doc = "0x20 - ADC Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x24 - ADC Pause Register"]
    #[inline(always)]
    pub const fn pause(&self) -> &Pause {
        &self.pause
    }
    #[doc = "0x30 - ADC FIFO Control Register"]
    #[inline(always)]
    pub const fn fctrl(&self) -> &Fctrl {
        &self.fctrl
    }
    #[doc = "0x34 - Software Trigger Register"]
    #[inline(always)]
    pub const fn swtrig(&self) -> &Swtrig {
        &self.swtrig
    }
    #[doc = "0xc0..0x100 - Trigger Control Register"]
    #[inline(always)]
    pub const fn tctrl(&self, n: usize) -> &Tctrl {
        &self.tctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0x100 - Trigger Control Register"]
    #[inline(always)]
    pub fn tctrl_iter(&self) -> impl Iterator<Item = &Tctrl> {
        self.tctrl.iter()
    }
    #[doc = "0x100 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl1(&self) -> &Cmdl1 {
        &self.cmdl1
    }
    #[doc = "0x104 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh1(&self) -> &Cmdh1 {
        &self.cmdh1
    }
    #[doc = "0x108 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl2(&self) -> &Cmdl2 {
        &self.cmdl2
    }
    #[doc = "0x10c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh2(&self) -> &Cmdh2 {
        &self.cmdh2
    }
    #[doc = "0x110 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl3(&self) -> &Cmdl3 {
        &self.cmdl3
    }
    #[doc = "0x114 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh3(&self) -> &Cmdh3 {
        &self.cmdh3
    }
    #[doc = "0x118 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl4(&self) -> &Cmdl4 {
        &self.cmdl4
    }
    #[doc = "0x11c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh4(&self) -> &Cmdh4 {
        &self.cmdh4
    }
    #[doc = "0x120 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl5(&self) -> &Cmdl5 {
        &self.cmdl5
    }
    #[doc = "0x124 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh5(&self) -> &Cmdh5 {
        &self.cmdh5
    }
    #[doc = "0x128 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl6(&self) -> &Cmdl6 {
        &self.cmdl6
    }
    #[doc = "0x12c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh6(&self) -> &Cmdh6 {
        &self.cmdh6
    }
    #[doc = "0x130 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl7(&self) -> &Cmdl7 {
        &self.cmdl7
    }
    #[doc = "0x134 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh7(&self) -> &Cmdh7 {
        &self.cmdh7
    }
    #[doc = "0x138 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl8(&self) -> &Cmdl8 {
        &self.cmdl8
    }
    #[doc = "0x13c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh8(&self) -> &Cmdh8 {
        &self.cmdh8
    }
    #[doc = "0x140 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl9(&self) -> &Cmdl9 {
        &self.cmdl9
    }
    #[doc = "0x144 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh9(&self) -> &Cmdh9 {
        &self.cmdh9
    }
    #[doc = "0x148 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl10(&self) -> &Cmdl10 {
        &self.cmdl10
    }
    #[doc = "0x14c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh10(&self) -> &Cmdh10 {
        &self.cmdh10
    }
    #[doc = "0x150 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl11(&self) -> &Cmdl11 {
        &self.cmdl11
    }
    #[doc = "0x154 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh11(&self) -> &Cmdh11 {
        &self.cmdh11
    }
    #[doc = "0x158 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl12(&self) -> &Cmdl12 {
        &self.cmdl12
    }
    #[doc = "0x15c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh12(&self) -> &Cmdh12 {
        &self.cmdh12
    }
    #[doc = "0x160 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl13(&self) -> &Cmdl13 {
        &self.cmdl13
    }
    #[doc = "0x164 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh13(&self) -> &Cmdh13 {
        &self.cmdh13
    }
    #[doc = "0x168 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl14(&self) -> &Cmdl14 {
        &self.cmdl14
    }
    #[doc = "0x16c - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh14(&self) -> &Cmdh14 {
        &self.cmdh14
    }
    #[doc = "0x170 - ADC Command Low Buffer Register"]
    #[inline(always)]
    pub const fn cmdl15(&self) -> &Cmdl15 {
        &self.cmdl15
    }
    #[doc = "0x174 - ADC Command High Buffer Register"]
    #[inline(always)]
    pub const fn cmdh15(&self) -> &Cmdh15 {
        &self.cmdh15
    }
    #[doc = "0x200..0x210 - Compare Value Register"]
    #[inline(always)]
    pub const fn cv(&self, n: usize) -> &Cv {
        &self.cv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x210 - Compare Value Register"]
    #[inline(always)]
    pub fn cv_iter(&self) -> impl Iterator<Item = &Cv> {
        self.cv.iter()
    }
    #[doc = "0x200 - Compare Value Register"]
    #[inline(always)]
    pub const fn cv1(&self) -> &Cv {
        self.cv(0)
    }
    #[doc = "0x204 - Compare Value Register"]
    #[inline(always)]
    pub const fn cv2(&self) -> &Cv {
        self.cv(1)
    }
    #[doc = "0x208 - Compare Value Register"]
    #[inline(always)]
    pub const fn cv3(&self) -> &Cv {
        self.cv(2)
    }
    #[doc = "0x20c - Compare Value Register"]
    #[inline(always)]
    pub const fn cv4(&self) -> &Cv {
        self.cv(3)
    }
    #[doc = "0x300 - ADC Data Result FIFO Register"]
    #[inline(always)]
    pub const fn resfifo(&self) -> &Resfifo {
        &self.resfifo
    }
}
#[doc = "VERID (r) register accessor: Version ID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`]
module"]
#[doc(alias = "VERID")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: Parameter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`param::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`]
module"]
#[doc(alias = "PARAM")]
pub type Param = crate::Reg<param::ParamSpec>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CTRL (rw) register accessor: ADC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "ADC Control Register"]
pub mod ctrl;
#[doc = "STAT (rw) register accessor: ADC Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "ADC Status Register"]
pub mod stat;
#[doc = "IE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`]
module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "DE (rw) register accessor: DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`de::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`de::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@de`]
module"]
#[doc(alias = "DE")]
pub type De = crate::Reg<de::DeSpec>;
#[doc = "DMA Enable Register"]
pub mod de;
#[doc = "CFG (rw) register accessor: ADC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "ADC Configuration Register"]
pub mod cfg;
#[doc = "PAUSE (rw) register accessor: ADC Pause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pause::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pause::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pause`]
module"]
#[doc(alias = "PAUSE")]
pub type Pause = crate::Reg<pause::PauseSpec>;
#[doc = "ADC Pause Register"]
pub mod pause;
#[doc = "FCTRL (rw) register accessor: ADC FIFO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl`]
module"]
#[doc(alias = "FCTRL")]
pub type Fctrl = crate::Reg<fctrl::FctrlSpec>;
#[doc = "ADC FIFO Control Register"]
pub mod fctrl;
#[doc = "SWTRIG (rw) register accessor: Software Trigger Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swtrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swtrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrig`]
module"]
#[doc(alias = "SWTRIG")]
pub type Swtrig = crate::Reg<swtrig::SwtrigSpec>;
#[doc = "Software Trigger Register"]
pub mod swtrig;
#[doc = "TCTRL (rw) register accessor: Trigger Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tctrl`]
module"]
#[doc(alias = "TCTRL")]
pub type Tctrl = crate::Reg<tctrl::TctrlSpec>;
#[doc = "Trigger Control Register"]
pub mod tctrl;
#[doc = "CMDL1 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl1`]
module"]
#[doc(alias = "CMDL1")]
pub type Cmdl1 = crate::Reg<cmdl1::Cmdl1Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl1;
#[doc = "CMDH1 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh1`]
module"]
#[doc(alias = "CMDH1")]
pub type Cmdh1 = crate::Reg<cmdh1::Cmdh1Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh1;
#[doc = "CMDL2 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl2`]
module"]
#[doc(alias = "CMDL2")]
pub type Cmdl2 = crate::Reg<cmdl2::Cmdl2Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl2;
#[doc = "CMDH2 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh2`]
module"]
#[doc(alias = "CMDH2")]
pub type Cmdh2 = crate::Reg<cmdh2::Cmdh2Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh2;
#[doc = "CMDL3 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl3`]
module"]
#[doc(alias = "CMDL3")]
pub type Cmdl3 = crate::Reg<cmdl3::Cmdl3Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl3;
#[doc = "CMDH3 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh3`]
module"]
#[doc(alias = "CMDH3")]
pub type Cmdh3 = crate::Reg<cmdh3::Cmdh3Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh3;
#[doc = "CMDL4 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl4`]
module"]
#[doc(alias = "CMDL4")]
pub type Cmdl4 = crate::Reg<cmdl4::Cmdl4Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl4;
#[doc = "CMDH4 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh4`]
module"]
#[doc(alias = "CMDH4")]
pub type Cmdh4 = crate::Reg<cmdh4::Cmdh4Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh4;
#[doc = "CMDL5 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl5`]
module"]
#[doc(alias = "CMDL5")]
pub type Cmdl5 = crate::Reg<cmdl5::Cmdl5Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl5;
#[doc = "CMDH5 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh5`]
module"]
#[doc(alias = "CMDH5")]
pub type Cmdh5 = crate::Reg<cmdh5::Cmdh5Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh5;
#[doc = "CMDL6 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl6`]
module"]
#[doc(alias = "CMDL6")]
pub type Cmdl6 = crate::Reg<cmdl6::Cmdl6Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl6;
#[doc = "CMDH6 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh6`]
module"]
#[doc(alias = "CMDH6")]
pub type Cmdh6 = crate::Reg<cmdh6::Cmdh6Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh6;
#[doc = "CMDL7 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl7`]
module"]
#[doc(alias = "CMDL7")]
pub type Cmdl7 = crate::Reg<cmdl7::Cmdl7Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl7;
#[doc = "CMDH7 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh7`]
module"]
#[doc(alias = "CMDH7")]
pub type Cmdh7 = crate::Reg<cmdh7::Cmdh7Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh7;
#[doc = "CMDL8 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl8`]
module"]
#[doc(alias = "CMDL8")]
pub type Cmdl8 = crate::Reg<cmdl8::Cmdl8Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl8;
#[doc = "CMDH8 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh8`]
module"]
#[doc(alias = "CMDH8")]
pub type Cmdh8 = crate::Reg<cmdh8::Cmdh8Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh8;
#[doc = "CMDL9 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl9`]
module"]
#[doc(alias = "CMDL9")]
pub type Cmdl9 = crate::Reg<cmdl9::Cmdl9Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl9;
#[doc = "CMDH9 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh9`]
module"]
#[doc(alias = "CMDH9")]
pub type Cmdh9 = crate::Reg<cmdh9::Cmdh9Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh9;
#[doc = "CMDL10 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl10`]
module"]
#[doc(alias = "CMDL10")]
pub type Cmdl10 = crate::Reg<cmdl10::Cmdl10Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl10;
#[doc = "CMDH10 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh10`]
module"]
#[doc(alias = "CMDH10")]
pub type Cmdh10 = crate::Reg<cmdh10::Cmdh10Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh10;
#[doc = "CMDL11 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl11`]
module"]
#[doc(alias = "CMDL11")]
pub type Cmdl11 = crate::Reg<cmdl11::Cmdl11Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl11;
#[doc = "CMDH11 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh11`]
module"]
#[doc(alias = "CMDH11")]
pub type Cmdh11 = crate::Reg<cmdh11::Cmdh11Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh11;
#[doc = "CMDL12 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl12`]
module"]
#[doc(alias = "CMDL12")]
pub type Cmdl12 = crate::Reg<cmdl12::Cmdl12Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl12;
#[doc = "CMDH12 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh12`]
module"]
#[doc(alias = "CMDH12")]
pub type Cmdh12 = crate::Reg<cmdh12::Cmdh12Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh12;
#[doc = "CMDL13 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl13`]
module"]
#[doc(alias = "CMDL13")]
pub type Cmdl13 = crate::Reg<cmdl13::Cmdl13Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl13;
#[doc = "CMDH13 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh13`]
module"]
#[doc(alias = "CMDH13")]
pub type Cmdh13 = crate::Reg<cmdh13::Cmdh13Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh13;
#[doc = "CMDL14 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl14`]
module"]
#[doc(alias = "CMDL14")]
pub type Cmdl14 = crate::Reg<cmdl14::Cmdl14Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl14;
#[doc = "CMDH14 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh14`]
module"]
#[doc(alias = "CMDH14")]
pub type Cmdh14 = crate::Reg<cmdh14::Cmdh14Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh14;
#[doc = "CMDL15 (rw) register accessor: ADC Command Low Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdl15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdl15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdl15`]
module"]
#[doc(alias = "CMDL15")]
pub type Cmdl15 = crate::Reg<cmdl15::Cmdl15Spec>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl15;
#[doc = "CMDH15 (rw) register accessor: ADC Command High Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdh15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdh15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdh15`]
module"]
#[doc(alias = "CMDH15")]
pub type Cmdh15 = crate::Reg<cmdh15::Cmdh15Spec>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh15;
#[doc = "CV (rw) register accessor: Compare Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv`]
module"]
#[doc(alias = "CV")]
pub type Cv = crate::Reg<cv::CvSpec>;
#[doc = "Compare Value Register"]
pub mod cv;
#[doc = "RESFIFO (r) register accessor: ADC Data Result FIFO Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resfifo::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resfifo`]
module"]
#[doc(alias = "RESFIFO")]
pub type Resfifo = crate::Reg<resfifo::ResfifoSpec>;
#[doc = "ADC Data Result FIFO Register"]
pub mod resfifo;
