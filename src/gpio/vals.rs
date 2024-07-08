#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Edge(pub u32);
impl Edge {
    #[doc = "level"]
    pub const LEVEL: Self = Self(0x0);
    #[doc = "edge"]
    pub const EDGE: Self = Self(0x01);
}
impl Edge {
    pub const fn from_bits(val: u32) -> Edge {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for Edge {
    #[inline(always)]
    fn from(val: u32) -> Edge {
        Edge::from_bits(val)
    }
}
impl From<Edge> for u32 {
    #[inline(always)]
    fn from(val: Edge) -> u32 {
        Edge::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct PolCtl(pub u32);
impl PolCtl {
    #[doc = "interrupt when gpio high"]
    pub const HIHG: Self = Self(0x0);
    #[doc = "interrupt when gpio low"]
    pub const LOW: Self = Self(0x01);
}
impl PolCtl {
    pub const fn from_bits(val: u32) -> PolCtl {
        Self(val & 0xffff_ffff)
    }
    pub const fn to_bits(self) -> u32 {
        self.0
    }
}
impl From<u32> for PolCtl {
    #[inline(always)]
    fn from(val: u32) -> PolCtl {
        PolCtl::from_bits(val)
    }
}
impl From<PolCtl> for u32 {
    #[inline(always)]
    fn from(val: PolCtl) -> u32 {
        PolCtl::to_bits(val)
    }
}
