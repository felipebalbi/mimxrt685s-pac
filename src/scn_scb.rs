#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    cppwr: Cppwr,
}
impl RegisterBlock {
    #[doc = "0x0c - Coprocessor Power Control Register"]
    #[inline(always)]
    pub const fn cppwr(&self) -> &Cppwr {
        &self.cppwr
    }
}
#[doc = "CPPWR (rw) register accessor: Coprocessor Power Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cppwr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cppwr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cppwr`]
module"]
#[doc(alias = "CPPWR")]
pub type Cppwr = crate::Reg<cppwr::CppwrSpec>;
#[doc = "Coprocessor Power Control Register"]
pub mod cppwr;
