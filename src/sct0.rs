#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    ctrl: Ctrl,
    limit: Limit,
    halt: Halt,
    stop: Stop,
    start: Start,
    _reserved6: [u8; 0x28],
    count: Count,
    state: State,
    input: Input,
    regmode: Regmode,
    output: Output,
    outputdirctrl: Outputdirctrl,
    res: Res,
    dmareq0: Dmareq0,
    dmareq1: Dmareq1,
    _reserved15: [u8; 0x8c],
    even: Even,
    evflag: Evflag,
    conen: Conen,
    conflag: Conflag,
    _reserved_19_cap_match: [u8; 0x04],
    _reserved_20_cap_match: [u8; 0x04],
    _reserved_21_cap_match: [u8; 0x04],
    _reserved_22_cap_match: [u8; 0x04],
    _reserved_23_cap_match: [u8; 0x04],
    _reserved_24_cap_match: [u8; 0x04],
    _reserved_25_cap_match: [u8; 0x04],
    _reserved_26_cap_match: [u8; 0x04],
    _reserved_27_cap_match: [u8; 0x04],
    _reserved_28_cap_match: [u8; 0x04],
    _reserved_29_cap_match: [u8; 0x04],
    _reserved_30_cap_match: [u8; 0x04],
    _reserved_31_cap_match: [u8; 0x04],
    _reserved_32_cap_match: [u8; 0x04],
    _reserved_33_cap_match: [u8; 0x04],
    _reserved_34_cap_match: [u8; 0x04],
    _reserved35: [u8; 0xc0],
    _reserved_35_capctrl_matchrel: [u8; 0x04],
    _reserved_36_capctrl_matchrel: [u8; 0x04],
    _reserved_37_capctrl_matchrel: [u8; 0x04],
    _reserved_38_capctrl_matchrel: [u8; 0x04],
    _reserved_39_capctrl_matchrel: [u8; 0x04],
    _reserved_40_capctrl_matchrel: [u8; 0x04],
    _reserved_41_capctrl_matchrel: [u8; 0x04],
    _reserved_42_capctrl_matchrel: [u8; 0x04],
    _reserved_43_capctrl_matchrel: [u8; 0x04],
    _reserved_44_capctrl_matchrel: [u8; 0x04],
    _reserved_45_capctrl_matchrel: [u8; 0x04],
    _reserved_46_capctrl_matchrel: [u8; 0x04],
    _reserved_47_capctrl_matchrel: [u8; 0x04],
    _reserved_48_capctrl_matchrel: [u8; 0x04],
    _reserved_49_capctrl_matchrel: [u8; 0x04],
    _reserved_50_capctrl_matchrel: [u8; 0x04],
    _reserved51: [u8; 0xc0],
    ev: [Ev; 16],
    _reserved52: [u8; 0x0180],
    out: [Out; 10],
}
impl RegisterBlock {
    #[doc = "0x00 - SCT configuration register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - SCT control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - SCT limit event select register"]
    #[inline(always)]
    pub const fn limit(&self) -> &Limit {
        &self.limit
    }
    #[doc = "0x0c - SCT halt event select register"]
    #[inline(always)]
    pub const fn halt(&self) -> &Halt {
        &self.halt
    }
    #[doc = "0x10 - SCT stop event select register"]
    #[inline(always)]
    pub const fn stop(&self) -> &Stop {
        &self.stop
    }
    #[doc = "0x14 - SCT start event select register"]
    #[inline(always)]
    pub const fn start(&self) -> &Start {
        &self.start
    }
    #[doc = "0x40 - SCT counter register"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x44 - SCT state register"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x48 - SCT input register"]
    #[inline(always)]
    pub const fn input(&self) -> &Input {
        &self.input
    }
    #[doc = "0x4c - SCT match/capture mode register"]
    #[inline(always)]
    pub const fn regmode(&self) -> &Regmode {
        &self.regmode
    }
    #[doc = "0x50 - SCT output register"]
    #[inline(always)]
    pub const fn output(&self) -> &Output {
        &self.output
    }
    #[doc = "0x54 - SCT output counter direction control register"]
    #[inline(always)]
    pub const fn outputdirctrl(&self) -> &Outputdirctrl {
        &self.outputdirctrl
    }
    #[doc = "0x58 - SCT conflict resolution register"]
    #[inline(always)]
    pub const fn res(&self) -> &Res {
        &self.res
    }
    #[doc = "0x5c - SCT DMA request 0 register"]
    #[inline(always)]
    pub const fn dmareq0(&self) -> &Dmareq0 {
        &self.dmareq0
    }
    #[doc = "0x60 - SCT DMA request 1 register"]
    #[inline(always)]
    pub const fn dmareq1(&self) -> &Dmareq1 {
        &self.dmareq1
    }
    #[doc = "0xf0 - SCT event interrupt enable register"]
    #[inline(always)]
    pub const fn even(&self) -> &Even {
        &self.even
    }
    #[doc = "0xf4 - SCT event flag register"]
    #[inline(always)]
    pub const fn evflag(&self) -> &Evflag {
        &self.evflag
    }
    #[doc = "0xf8 - SCT conflict interrupt enable register"]
    #[inline(always)]
    pub const fn conen(&self) -> &Conen {
        &self.conen
    }
    #[doc = "0xfc - SCT conflict flag register"]
    #[inline(always)]
    pub const fn conflag(&self) -> &Conflag {
        &self.conflag
    }
    #[doc = "0x100 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match0(&self) -> &CapMatchMatch0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x100 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap0(&self) -> &CapMatchCap0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(256).cast() }
    }
    #[doc = "0x104 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match1(&self) -> &CapMatchMatch1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x104 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap1(&self) -> &CapMatchCap1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260).cast() }
    }
    #[doc = "0x108 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match2(&self) -> &CapMatchMatch2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x108 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap2(&self) -> &CapMatchCap2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x10c - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match3(&self) -> &CapMatchMatch3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x10c - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap3(&self) -> &CapMatchCap3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(268).cast() }
    }
    #[doc = "0x110 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match4(&self) -> &CapMatchMatch4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x110 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap4(&self) -> &CapMatchCap4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(272).cast() }
    }
    #[doc = "0x114 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match5(&self) -> &CapMatchMatch5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x114 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap5(&self) -> &CapMatchCap5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(276).cast() }
    }
    #[doc = "0x118 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match6(&self) -> &CapMatchMatch6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x118 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap6(&self) -> &CapMatchCap6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11c - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match7(&self) -> &CapMatchMatch7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x11c - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap7(&self) -> &CapMatchCap7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(284).cast() }
    }
    #[doc = "0x120 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match8(&self) -> &CapMatchMatch8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x120 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap8(&self) -> &CapMatchCap8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(288).cast() }
    }
    #[doc = "0x124 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match9(&self) -> &CapMatchMatch9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x124 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap9(&self) -> &CapMatchCap9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(292).cast() }
    }
    #[doc = "0x128 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match10(&self) -> &CapMatchMatch10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x128 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap10(&self) -> &CapMatchCap10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12c - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match11(&self) -> &CapMatchMatch11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x12c - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap11(&self) -> &CapMatchCap11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(300).cast() }
    }
    #[doc = "0x130 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match12(&self) -> &CapMatchMatch12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x130 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap12(&self) -> &CapMatchCap12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(304).cast() }
    }
    #[doc = "0x134 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match13(&self) -> &CapMatchMatch13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x134 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap13(&self) -> &CapMatchCap13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(308).cast() }
    }
    #[doc = "0x138 - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match14(&self) -> &CapMatchMatch14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x138 - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap14(&self) -> &CapMatchCap14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x13c - SCT match value register of match channels"]
    #[inline(always)]
    pub const fn cap_match_match15(&self) -> &CapMatchMatch15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x13c - SCT capture register of capture channel"]
    #[inline(always)]
    pub const fn cap_match_cap15(&self) -> &CapMatchCap15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(316).cast() }
    }
    #[doc = "0x200 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel0(&self) -> &CapctrlMatchrelMatchrel0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x200 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl0(&self) -> &CapctrlMatchrelCapctrl0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x204 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel1(&self) -> &CapctrlMatchrelMatchrel1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x204 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl1(&self) -> &CapctrlMatchrelCapctrl1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x208 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel2(&self) -> &CapctrlMatchrelMatchrel2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x208 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl2(&self) -> &CapctrlMatchrelCapctrl2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x20c - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel3(&self) -> &CapctrlMatchrelMatchrel3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x20c - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl3(&self) -> &CapctrlMatchrelCapctrl3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(524).cast() }
    }
    #[doc = "0x210 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel4(&self) -> &CapctrlMatchrelMatchrel4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x210 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl4(&self) -> &CapctrlMatchrelCapctrl4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x214 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel5(&self) -> &CapctrlMatchrelMatchrel5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x214 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl5(&self) -> &CapctrlMatchrelCapctrl5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x218 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel6(&self) -> &CapctrlMatchrelMatchrel6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x218 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl6(&self) -> &CapctrlMatchrelCapctrl6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x21c - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel7(&self) -> &CapctrlMatchrelMatchrel7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x21c - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl7(&self) -> &CapctrlMatchrelCapctrl7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(540).cast() }
    }
    #[doc = "0x220 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel8(&self) -> &CapctrlMatchrelMatchrel8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x220 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl8(&self) -> &CapctrlMatchrelCapctrl8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x224 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel9(&self) -> &CapctrlMatchrelMatchrel9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x224 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl9(&self) -> &CapctrlMatchrelCapctrl9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x228 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel10(&self) -> &CapctrlMatchrelMatchrel10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x228 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl10(&self) -> &CapctrlMatchrelCapctrl10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x22c - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel11(&self) -> &CapctrlMatchrelMatchrel11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x22c - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl11(&self) -> &CapctrlMatchrelCapctrl11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(556).cast() }
    }
    #[doc = "0x230 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel12(&self) -> &CapctrlMatchrelMatchrel12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x230 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl12(&self) -> &CapctrlMatchrelCapctrl12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x234 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel13(&self) -> &CapctrlMatchrelMatchrel13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x234 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl13(&self) -> &CapctrlMatchrelCapctrl13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x238 - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel14(&self) -> &CapctrlMatchrelMatchrel14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x238 - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl14(&self) -> &CapctrlMatchrelCapctrl14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x23c - SCT match reload value register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_matchrel15(&self) -> &CapctrlMatchrelMatchrel15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x23c - SCT capture control register"]
    #[inline(always)]
    pub const fn capctrl_matchrel_capctrl15(&self) -> &CapctrlMatchrelCapctrl15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(572).cast() }
    }
    #[doc = "0x300..0x380 - no description available"]
    #[inline(always)]
    pub const fn ev(&self, n: usize) -> &Ev {
        &self.ev[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x380 - no description available"]
    #[inline(always)]
    pub fn ev_iter(&self) -> impl Iterator<Item = &Ev> {
        self.ev.iter()
    }
    #[doc = "0x500..0x550 - no description available"]
    #[inline(always)]
    pub const fn out(&self, n: usize) -> &Out {
        &self.out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x550 - no description available"]
    #[inline(always)]
    pub fn out_iter(&self) -> impl Iterator<Item = &Out> {
        self.out.iter()
    }
}
#[doc = "CONFIG (rw) register accessor: SCT configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "SCT configuration register"]
pub mod config;
#[doc = "CTRL (rw) register accessor: SCT control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "SCT control register"]
pub mod ctrl;
#[doc = "LIMIT (rw) register accessor: SCT limit event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`limit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`limit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@limit`]
module"]
#[doc(alias = "LIMIT")]
pub type Limit = crate::Reg<limit::LimitSpec>;
#[doc = "SCT limit event select register"]
pub mod limit;
#[doc = "HALT (rw) register accessor: SCT halt event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`halt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`halt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@halt`]
module"]
#[doc(alias = "HALT")]
pub type Halt = crate::Reg<halt::HaltSpec>;
#[doc = "SCT halt event select register"]
pub mod halt;
#[doc = "STOP (rw) register accessor: SCT stop event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stop`]
module"]
#[doc(alias = "STOP")]
pub type Stop = crate::Reg<stop::StopSpec>;
#[doc = "SCT stop event select register"]
pub mod stop;
#[doc = "START (rw) register accessor: SCT start event select register\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@start`]
module"]
#[doc(alias = "START")]
pub type Start = crate::Reg<start::StartSpec>;
#[doc = "SCT start event select register"]
pub mod start;
#[doc = "COUNT (rw) register accessor: SCT counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "SCT counter register"]
pub mod count;
#[doc = "STATE (rw) register accessor: SCT state register\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`state::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "SCT state register"]
pub mod state;
#[doc = "INPUT (r) register accessor: SCT input register\n\nYou can [`read`](crate::Reg::read) this register and get [`input::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@input`]
module"]
#[doc(alias = "INPUT")]
pub type Input = crate::Reg<input::InputSpec>;
#[doc = "SCT input register"]
pub mod input;
#[doc = "REGMODE (rw) register accessor: SCT match/capture mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`regmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regmode`]
module"]
#[doc(alias = "REGMODE")]
pub type Regmode = crate::Reg<regmode::RegmodeSpec>;
#[doc = "SCT match/capture mode register"]
pub mod regmode;
#[doc = "OUTPUT (rw) register accessor: SCT output register\n\nYou can [`read`](crate::Reg::read) this register and get [`output::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`output::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@output`]
module"]
#[doc(alias = "OUTPUT")]
pub type Output = crate::Reg<output::OutputSpec>;
#[doc = "SCT output register"]
pub mod output;
#[doc = "OUTPUTDIRCTRL (rw) register accessor: SCT output counter direction control register\n\nYou can [`read`](crate::Reg::read) this register and get [`outputdirctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outputdirctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@outputdirctrl`]
module"]
#[doc(alias = "OUTPUTDIRCTRL")]
pub type Outputdirctrl = crate::Reg<outputdirctrl::OutputdirctrlSpec>;
#[doc = "SCT output counter direction control register"]
pub mod outputdirctrl;
#[doc = "RES (rw) register accessor: SCT conflict resolution register\n\nYou can [`read`](crate::Reg::read) this register and get [`res::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@res`]
module"]
#[doc(alias = "RES")]
pub type Res = crate::Reg<res::ResSpec>;
#[doc = "SCT conflict resolution register"]
pub mod res;
#[doc = "DMAREQ0 (rw) register accessor: SCT DMA request 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmareq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq0`]
module"]
#[doc(alias = "DMAREQ0")]
pub type Dmareq0 = crate::Reg<dmareq0::Dmareq0Spec>;
#[doc = "SCT DMA request 0 register"]
pub mod dmareq0;
#[doc = "DMAREQ1 (rw) register accessor: SCT DMA request 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmareq1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmareq1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmareq1`]
module"]
#[doc(alias = "DMAREQ1")]
pub type Dmareq1 = crate::Reg<dmareq1::Dmareq1Spec>;
#[doc = "SCT DMA request 1 register"]
pub mod dmareq1;
#[doc = "EVEN (rw) register accessor: SCT event interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`even::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`even::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@even`]
module"]
#[doc(alias = "EVEN")]
pub type Even = crate::Reg<even::EvenSpec>;
#[doc = "SCT event interrupt enable register"]
pub mod even;
#[doc = "EVFLAG (rw) register accessor: SCT event flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`evflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evflag`]
module"]
#[doc(alias = "EVFLAG")]
pub type Evflag = crate::Reg<evflag::EvflagSpec>;
#[doc = "SCT event flag register"]
pub mod evflag;
#[doc = "CONEN (rw) register accessor: SCT conflict interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`conen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conen`]
module"]
#[doc(alias = "CONEN")]
pub type Conen = crate::Reg<conen::ConenSpec>;
#[doc = "SCT conflict interrupt enable register"]
pub mod conen;
#[doc = "CONFLAG (rw) register accessor: SCT conflict flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`conflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conflag`]
module"]
#[doc(alias = "CONFLAG")]
pub type Conflag = crate::Reg<conflag::ConflagSpec>;
#[doc = "SCT conflict flag register"]
pub mod conflag;
#[doc = "CAP_MATCH_CAP0 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap0`]
module"]
#[doc(alias = "CAP_MATCH_CAP0")]
pub type CapMatchCap0 = crate::Reg<cap_match_cap0::CapMatchCap0Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap0;
#[doc = "CAP_MATCH_MATCH0 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match0`]
module"]
#[doc(alias = "CAP_MATCH_MATCH0")]
pub type CapMatchMatch0 = crate::Reg<cap_match_match0::CapMatchMatch0Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match0;
#[doc = "CAP_MATCH_CAP1 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap1`]
module"]
#[doc(alias = "CAP_MATCH_CAP1")]
pub type CapMatchCap1 = crate::Reg<cap_match_cap1::CapMatchCap1Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap1;
#[doc = "CAP_MATCH_MATCH1 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match1`]
module"]
#[doc(alias = "CAP_MATCH_MATCH1")]
pub type CapMatchMatch1 = crate::Reg<cap_match_match1::CapMatchMatch1Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match1;
#[doc = "CAP_MATCH_CAP2 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap2`]
module"]
#[doc(alias = "CAP_MATCH_CAP2")]
pub type CapMatchCap2 = crate::Reg<cap_match_cap2::CapMatchCap2Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap2;
#[doc = "CAP_MATCH_MATCH2 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match2`]
module"]
#[doc(alias = "CAP_MATCH_MATCH2")]
pub type CapMatchMatch2 = crate::Reg<cap_match_match2::CapMatchMatch2Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match2;
#[doc = "CAP_MATCH_CAP3 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap3`]
module"]
#[doc(alias = "CAP_MATCH_CAP3")]
pub type CapMatchCap3 = crate::Reg<cap_match_cap3::CapMatchCap3Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap3;
#[doc = "CAP_MATCH_MATCH3 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match3`]
module"]
#[doc(alias = "CAP_MATCH_MATCH3")]
pub type CapMatchMatch3 = crate::Reg<cap_match_match3::CapMatchMatch3Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match3;
#[doc = "CAP_MATCH_CAP4 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap4`]
module"]
#[doc(alias = "CAP_MATCH_CAP4")]
pub type CapMatchCap4 = crate::Reg<cap_match_cap4::CapMatchCap4Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap4;
#[doc = "CAP_MATCH_MATCH4 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match4`]
module"]
#[doc(alias = "CAP_MATCH_MATCH4")]
pub type CapMatchMatch4 = crate::Reg<cap_match_match4::CapMatchMatch4Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match4;
#[doc = "CAP_MATCH_CAP5 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap5`]
module"]
#[doc(alias = "CAP_MATCH_CAP5")]
pub type CapMatchCap5 = crate::Reg<cap_match_cap5::CapMatchCap5Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap5;
#[doc = "CAP_MATCH_MATCH5 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match5`]
module"]
#[doc(alias = "CAP_MATCH_MATCH5")]
pub type CapMatchMatch5 = crate::Reg<cap_match_match5::CapMatchMatch5Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match5;
#[doc = "CAP_MATCH_CAP6 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap6`]
module"]
#[doc(alias = "CAP_MATCH_CAP6")]
pub type CapMatchCap6 = crate::Reg<cap_match_cap6::CapMatchCap6Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap6;
#[doc = "CAP_MATCH_MATCH6 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match6`]
module"]
#[doc(alias = "CAP_MATCH_MATCH6")]
pub type CapMatchMatch6 = crate::Reg<cap_match_match6::CapMatchMatch6Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match6;
#[doc = "CAP_MATCH_CAP7 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap7`]
module"]
#[doc(alias = "CAP_MATCH_CAP7")]
pub type CapMatchCap7 = crate::Reg<cap_match_cap7::CapMatchCap7Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap7;
#[doc = "CAP_MATCH_MATCH7 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match7`]
module"]
#[doc(alias = "CAP_MATCH_MATCH7")]
pub type CapMatchMatch7 = crate::Reg<cap_match_match7::CapMatchMatch7Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match7;
#[doc = "CAP_MATCH_CAP8 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap8`]
module"]
#[doc(alias = "CAP_MATCH_CAP8")]
pub type CapMatchCap8 = crate::Reg<cap_match_cap8::CapMatchCap8Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap8;
#[doc = "CAP_MATCH_MATCH8 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match8`]
module"]
#[doc(alias = "CAP_MATCH_MATCH8")]
pub type CapMatchMatch8 = crate::Reg<cap_match_match8::CapMatchMatch8Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match8;
#[doc = "CAP_MATCH_CAP9 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap9`]
module"]
#[doc(alias = "CAP_MATCH_CAP9")]
pub type CapMatchCap9 = crate::Reg<cap_match_cap9::CapMatchCap9Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap9;
#[doc = "CAP_MATCH_MATCH9 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match9`]
module"]
#[doc(alias = "CAP_MATCH_MATCH9")]
pub type CapMatchMatch9 = crate::Reg<cap_match_match9::CapMatchMatch9Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match9;
#[doc = "CAP_MATCH_CAP10 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap10`]
module"]
#[doc(alias = "CAP_MATCH_CAP10")]
pub type CapMatchCap10 = crate::Reg<cap_match_cap10::CapMatchCap10Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap10;
#[doc = "CAP_MATCH_MATCH10 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match10`]
module"]
#[doc(alias = "CAP_MATCH_MATCH10")]
pub type CapMatchMatch10 = crate::Reg<cap_match_match10::CapMatchMatch10Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match10;
#[doc = "CAP_MATCH_CAP11 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap11`]
module"]
#[doc(alias = "CAP_MATCH_CAP11")]
pub type CapMatchCap11 = crate::Reg<cap_match_cap11::CapMatchCap11Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap11;
#[doc = "CAP_MATCH_MATCH11 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match11`]
module"]
#[doc(alias = "CAP_MATCH_MATCH11")]
pub type CapMatchMatch11 = crate::Reg<cap_match_match11::CapMatchMatch11Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match11;
#[doc = "CAP_MATCH_CAP12 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap12`]
module"]
#[doc(alias = "CAP_MATCH_CAP12")]
pub type CapMatchCap12 = crate::Reg<cap_match_cap12::CapMatchCap12Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap12;
#[doc = "CAP_MATCH_MATCH12 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match12`]
module"]
#[doc(alias = "CAP_MATCH_MATCH12")]
pub type CapMatchMatch12 = crate::Reg<cap_match_match12::CapMatchMatch12Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match12;
#[doc = "CAP_MATCH_CAP13 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap13`]
module"]
#[doc(alias = "CAP_MATCH_CAP13")]
pub type CapMatchCap13 = crate::Reg<cap_match_cap13::CapMatchCap13Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap13;
#[doc = "CAP_MATCH_MATCH13 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match13`]
module"]
#[doc(alias = "CAP_MATCH_MATCH13")]
pub type CapMatchMatch13 = crate::Reg<cap_match_match13::CapMatchMatch13Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match13;
#[doc = "CAP_MATCH_CAP14 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap14`]
module"]
#[doc(alias = "CAP_MATCH_CAP14")]
pub type CapMatchCap14 = crate::Reg<cap_match_cap14::CapMatchCap14Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap14;
#[doc = "CAP_MATCH_MATCH14 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match14`]
module"]
#[doc(alias = "CAP_MATCH_MATCH14")]
pub type CapMatchMatch14 = crate::Reg<cap_match_match14::CapMatchMatch14Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match14;
#[doc = "CAP_MATCH_CAP15 (rw) register accessor: SCT capture register of capture channel\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_cap15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_cap15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_cap15`]
module"]
#[doc(alias = "CAP_MATCH_CAP15")]
pub type CapMatchCap15 = crate::Reg<cap_match_cap15::CapMatchCap15Spec>;
#[doc = "SCT capture register of capture channel"]
pub mod cap_match_cap15;
#[doc = "CAP_MATCH_MATCH15 (rw) register accessor: SCT match value register of match channels\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_match_match15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_match_match15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_match_match15`]
module"]
#[doc(alias = "CAP_MATCH_MATCH15")]
pub type CapMatchMatch15 = crate::Reg<cap_match_match15::CapMatchMatch15Spec>;
#[doc = "SCT match value register of match channels"]
pub mod cap_match_match15;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL0 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl0`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL0")]
pub type CapctrlMatchrelCapctrl0 =
    crate::Reg<capctrl_matchrel_capctrl0::CapctrlMatchrelCapctrl0Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl0;
#[doc = "CAPCTRL_MATCHREL_MATCHREL0 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel0`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL0")]
pub type CapctrlMatchrelMatchrel0 =
    crate::Reg<capctrl_matchrel_matchrel0::CapctrlMatchrelMatchrel0Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel0;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL1 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl1`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL1")]
pub type CapctrlMatchrelCapctrl1 =
    crate::Reg<capctrl_matchrel_capctrl1::CapctrlMatchrelCapctrl1Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl1;
#[doc = "CAPCTRL_MATCHREL_MATCHREL1 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel1`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL1")]
pub type CapctrlMatchrelMatchrel1 =
    crate::Reg<capctrl_matchrel_matchrel1::CapctrlMatchrelMatchrel1Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel1;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL2 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl2`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL2")]
pub type CapctrlMatchrelCapctrl2 =
    crate::Reg<capctrl_matchrel_capctrl2::CapctrlMatchrelCapctrl2Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl2;
#[doc = "CAPCTRL_MATCHREL_MATCHREL2 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel2`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL2")]
pub type CapctrlMatchrelMatchrel2 =
    crate::Reg<capctrl_matchrel_matchrel2::CapctrlMatchrelMatchrel2Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel2;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL3 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl3`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL3")]
pub type CapctrlMatchrelCapctrl3 =
    crate::Reg<capctrl_matchrel_capctrl3::CapctrlMatchrelCapctrl3Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl3;
#[doc = "CAPCTRL_MATCHREL_MATCHREL3 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel3`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL3")]
pub type CapctrlMatchrelMatchrel3 =
    crate::Reg<capctrl_matchrel_matchrel3::CapctrlMatchrelMatchrel3Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel3;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL4 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl4`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL4")]
pub type CapctrlMatchrelCapctrl4 =
    crate::Reg<capctrl_matchrel_capctrl4::CapctrlMatchrelCapctrl4Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl4;
#[doc = "CAPCTRL_MATCHREL_MATCHREL4 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel4`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL4")]
pub type CapctrlMatchrelMatchrel4 =
    crate::Reg<capctrl_matchrel_matchrel4::CapctrlMatchrelMatchrel4Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel4;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL5 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl5`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL5")]
pub type CapctrlMatchrelCapctrl5 =
    crate::Reg<capctrl_matchrel_capctrl5::CapctrlMatchrelCapctrl5Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl5;
#[doc = "CAPCTRL_MATCHREL_MATCHREL5 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel5`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL5")]
pub type CapctrlMatchrelMatchrel5 =
    crate::Reg<capctrl_matchrel_matchrel5::CapctrlMatchrelMatchrel5Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel5;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL6 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl6`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL6")]
pub type CapctrlMatchrelCapctrl6 =
    crate::Reg<capctrl_matchrel_capctrl6::CapctrlMatchrelCapctrl6Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl6;
#[doc = "CAPCTRL_MATCHREL_MATCHREL6 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel6`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL6")]
pub type CapctrlMatchrelMatchrel6 =
    crate::Reg<capctrl_matchrel_matchrel6::CapctrlMatchrelMatchrel6Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel6;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL7 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl7`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL7")]
pub type CapctrlMatchrelCapctrl7 =
    crate::Reg<capctrl_matchrel_capctrl7::CapctrlMatchrelCapctrl7Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl7;
#[doc = "CAPCTRL_MATCHREL_MATCHREL7 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel7`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL7")]
pub type CapctrlMatchrelMatchrel7 =
    crate::Reg<capctrl_matchrel_matchrel7::CapctrlMatchrelMatchrel7Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel7;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL8 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl8`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL8")]
pub type CapctrlMatchrelCapctrl8 =
    crate::Reg<capctrl_matchrel_capctrl8::CapctrlMatchrelCapctrl8Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl8;
#[doc = "CAPCTRL_MATCHREL_MATCHREL8 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel8`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL8")]
pub type CapctrlMatchrelMatchrel8 =
    crate::Reg<capctrl_matchrel_matchrel8::CapctrlMatchrelMatchrel8Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel8;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL9 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl9`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL9")]
pub type CapctrlMatchrelCapctrl9 =
    crate::Reg<capctrl_matchrel_capctrl9::CapctrlMatchrelCapctrl9Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl9;
#[doc = "CAPCTRL_MATCHREL_MATCHREL9 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel9`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL9")]
pub type CapctrlMatchrelMatchrel9 =
    crate::Reg<capctrl_matchrel_matchrel9::CapctrlMatchrelMatchrel9Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel9;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL10 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl10`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL10")]
pub type CapctrlMatchrelCapctrl10 =
    crate::Reg<capctrl_matchrel_capctrl10::CapctrlMatchrelCapctrl10Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl10;
#[doc = "CAPCTRL_MATCHREL_MATCHREL10 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel10`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL10")]
pub type CapctrlMatchrelMatchrel10 =
    crate::Reg<capctrl_matchrel_matchrel10::CapctrlMatchrelMatchrel10Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel10;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL11 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl11`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL11")]
pub type CapctrlMatchrelCapctrl11 =
    crate::Reg<capctrl_matchrel_capctrl11::CapctrlMatchrelCapctrl11Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl11;
#[doc = "CAPCTRL_MATCHREL_MATCHREL11 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel11`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL11")]
pub type CapctrlMatchrelMatchrel11 =
    crate::Reg<capctrl_matchrel_matchrel11::CapctrlMatchrelMatchrel11Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel11;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL12 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl12`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL12")]
pub type CapctrlMatchrelCapctrl12 =
    crate::Reg<capctrl_matchrel_capctrl12::CapctrlMatchrelCapctrl12Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl12;
#[doc = "CAPCTRL_MATCHREL_MATCHREL12 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel12`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL12")]
pub type CapctrlMatchrelMatchrel12 =
    crate::Reg<capctrl_matchrel_matchrel12::CapctrlMatchrelMatchrel12Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel12;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL13 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl13`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL13")]
pub type CapctrlMatchrelCapctrl13 =
    crate::Reg<capctrl_matchrel_capctrl13::CapctrlMatchrelCapctrl13Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl13;
#[doc = "CAPCTRL_MATCHREL_MATCHREL13 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel13`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL13")]
pub type CapctrlMatchrelMatchrel13 =
    crate::Reg<capctrl_matchrel_matchrel13::CapctrlMatchrelMatchrel13Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel13;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL14 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl14`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL14")]
pub type CapctrlMatchrelCapctrl14 =
    crate::Reg<capctrl_matchrel_capctrl14::CapctrlMatchrelCapctrl14Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl14;
#[doc = "CAPCTRL_MATCHREL_MATCHREL14 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel14`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL14")]
pub type CapctrlMatchrelMatchrel14 =
    crate::Reg<capctrl_matchrel_matchrel14::CapctrlMatchrelMatchrel14Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel14;
#[doc = "CAPCTRL_MATCHREL_CAPCTRL15 (rw) register accessor: SCT capture control register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_capctrl15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_capctrl15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_capctrl15`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_CAPCTRL15")]
pub type CapctrlMatchrelCapctrl15 =
    crate::Reg<capctrl_matchrel_capctrl15::CapctrlMatchrelCapctrl15Spec>;
#[doc = "SCT capture control register"]
pub mod capctrl_matchrel_capctrl15;
#[doc = "CAPCTRL_MATCHREL_MATCHREL15 (rw) register accessor: SCT match reload value register\n\nYou can [`read`](crate::Reg::read) this register and get [`capctrl_matchrel_matchrel15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capctrl_matchrel_matchrel15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@capctrl_matchrel_matchrel15`]
module"]
#[doc(alias = "CAPCTRL_MATCHREL_MATCHREL15")]
pub type CapctrlMatchrelMatchrel15 =
    crate::Reg<capctrl_matchrel_matchrel15::CapctrlMatchrelMatchrel15Spec>;
#[doc = "SCT match reload value register"]
pub mod capctrl_matchrel_matchrel15;
#[doc = "no description available"]
pub use self::ev::Ev;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod ev;
#[doc = "no description available"]
pub use self::out::Out;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod out;
