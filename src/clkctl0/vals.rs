#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc0fclksel0Sel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "XTALIN Clock."]
    XTALIN_CLK = 0x01,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Adc0fclksel0Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0fclksel0Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0fclksel0Sel {
    #[inline(always)]
    fn from(val: u8) -> Adc0fclksel0Sel {
        Adc0fclksel0Sel::from_bits(val)
    }
}
impl From<Adc0fclksel0Sel> for u8 {
    #[inline(always)]
    fn from(val: Adc0fclksel0Sel) -> u8 {
        Adc0fclksel0Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Adc0fclksel1Sel {
    #[doc = "ADC0FCLKSEL0 Multiplexed Output."]
    ADC0FCLKSEL0_MUX_OUT = 0x0,
    #[doc = "SYSPLL0 MAIN_CLK (PFD0 Output)"]
    SYSPLL0_MAIN_CLK = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x03,
    _RESERVED_4 = 0x04,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Adc0fclksel1Sel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Adc0fclksel1Sel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Adc0fclksel1Sel {
    #[inline(always)]
    fn from(val: u8) -> Adc0fclksel1Sel {
        Adc0fclksel1Sel::from_bits(val)
    }
}
impl From<Adc0fclksel1Sel> for u8 {
    #[inline(always)]
    fn from(val: Adc0fclksel1Sel) -> u8 {
        Adc0fclksel1Sel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Bypass {
    #[doc = "PFD output is PFD programmed clock."]
    PROGRAMMED_CLK = 0x0,
    #[doc = "PFD output is PLL Input clock. (Bypass)"]
    BYPASS = 0x01,
}
impl Bypass {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bypass {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bypass {
    #[inline(always)]
    fn from(val: u8) -> Bypass {
        Bypass::from_bits(val)
    }
}
impl From<Bypass> for u8 {
    #[inline(always)]
    fn from(val: Bypass) -> u8 {
        Bypass::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum BypassEnable {
    #[doc = "Normal Mode."]
    NORMAL_MODE = 0x0,
    #[doc = "Bypass Mode."]
    BYPASS_MODE = 0x01,
}
impl BypassEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> BypassEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for BypassEnable {
    #[inline(always)]
    fn from(val: u8) -> BypassEnable {
        BypassEnable::from_bits(val)
    }
}
impl From<BypassEnable> for u8 {
    #[inline(always)]
    fn from(val: BypassEnable) -> u8 {
        BypassEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Ena32khz {
    #[doc = "disable"]
    DISABLED = 0x0,
    #[doc = "enable"]
    ENABLED = 0x01,
}
impl Ena32khz {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Ena32khz {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Ena32khz {
    #[inline(always)]
    fn from(val: u8) -> Ena32khz {
        Ena32khz::from_bits(val)
    }
}
impl From<Ena32khz> for u8 {
    #[inline(always)]
    fn from(val: Ena32khz) -> u8 {
        Ena32khz::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum FlexspifclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_SYS_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl FlexspifclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> FlexspifclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for FlexspifclkselSel {
    #[inline(always)]
    fn from(val: u8) -> FlexspifclkselSel {
        FlexspifclkselSel::from_bits(val)
    }
}
impl From<FlexspifclkselSel> for u8 {
    #[inline(always)]
    fn from(val: FlexspifclkselSel) -> u8 {
        FlexspifclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HoldringoffEna {
    #[doc = "disbale"]
    DSIABLE = 0x0,
    #[doc = "enable"]
    ENABLE = 0x01,
}
impl HoldringoffEna {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> HoldringoffEna {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for HoldringoffEna {
    #[inline(always)]
    fn from(val: u8) -> HoldringoffEna {
        HoldringoffEna::from_bits(val)
    }
}
impl From<HoldringoffEna> for u8 {
    #[inline(always)]
    fn from(val: HoldringoffEna) -> u8 {
        HoldringoffEna::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LpEnable {
    #[doc = "High Gain Mode(HP)."]
    HP = 0x0,
    #[doc = "Low Power mode (LP)."]
    LP = 0x01,
}
impl LpEnable {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> LpEnable {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for LpEnable {
    #[inline(always)]
    fn from(val: u8) -> LpEnable {
        LpEnable::from_bits(val)
    }
}
impl From<LpEnable> for u8 {
    #[inline(always)]
    fn from(val: LpEnable) -> u8 {
        LpEnable::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MainclkselaSel {
    #[doc = "FFRO Clock Divided by 4."]
    FFRO_DIV_4 = 0x0,
    #[doc = "SYSXTALIN Clock."]
    SYSXTAL_CLK = 0x01,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
}
impl MainclkselaSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkselaSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkselaSel {
    #[inline(always)]
    fn from(val: u8) -> MainclkselaSel {
        MainclkselaSel::from_bits(val)
    }
}
impl From<MainclkselaSel> for u8 {
    #[inline(always)]
    fn from(val: MainclkselaSel) -> u8 {
        MainclkselaSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum MainclkselbSel {
    #[doc = "MAINCLKSELA 1st Stage Clock."]
    MAIN_1ST_CLK = 0x0,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x01,
    #[doc = "Main System PLL Clock."]
    MAIN_PLL_CLK = 0x02,
    #[doc = "RTC 32KHz Clock."]
    RTC_32K_CLK = 0x03,
}
impl MainclkselbSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> MainclkselbSel {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for MainclkselbSel {
    #[inline(always)]
    fn from(val: u8) -> MainclkselbSel {
        MainclkselbSel::from_bits(val)
    }
}
impl From<MainclkselbSel> for u8 {
    #[inline(always)]
    fn from(val: MainclkselbSel) -> u8 {
        MainclkselbSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Mult {
    _RESERVED_0 = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    _RESERVED_7 = 0x07,
    _RESERVED_8 = 0x08,
    _RESERVED_9 = 0x09,
    _RESERVED_a = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    #[doc = "Div 16"]
    DIV_16 = 0x10,
    #[doc = "Div 17"]
    DIV_17 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    #[doc = "Div 20"]
    DIV_20 = 0x14,
    _RESERVED_15 = 0x15,
    #[doc = "Div 22"]
    DIV_22 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    #[doc = "Div 27"]
    DIV_27 = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    _RESERVED_1f = 0x1f,
    _RESERVED_20 = 0x20,
    #[doc = "Div 33"]
    DIV_33 = 0x21,
    _RESERVED_22 = 0x22,
    _RESERVED_23 = 0x23,
    _RESERVED_24 = 0x24,
    _RESERVED_25 = 0x25,
    _RESERVED_26 = 0x26,
    _RESERVED_27 = 0x27,
    _RESERVED_28 = 0x28,
    _RESERVED_29 = 0x29,
    _RESERVED_2a = 0x2a,
    _RESERVED_2b = 0x2b,
    _RESERVED_2c = 0x2c,
    _RESERVED_2d = 0x2d,
    _RESERVED_2e = 0x2e,
    _RESERVED_2f = 0x2f,
    _RESERVED_30 = 0x30,
    _RESERVED_31 = 0x31,
    _RESERVED_32 = 0x32,
    _RESERVED_33 = 0x33,
    _RESERVED_34 = 0x34,
    _RESERVED_35 = 0x35,
    _RESERVED_36 = 0x36,
    _RESERVED_37 = 0x37,
    _RESERVED_38 = 0x38,
    _RESERVED_39 = 0x39,
    _RESERVED_3a = 0x3a,
    _RESERVED_3b = 0x3b,
    _RESERVED_3c = 0x3c,
    _RESERVED_3d = 0x3d,
    _RESERVED_3e = 0x3e,
    _RESERVED_3f = 0x3f,
    _RESERVED_40 = 0x40,
    _RESERVED_41 = 0x41,
    _RESERVED_42 = 0x42,
    _RESERVED_43 = 0x43,
    _RESERVED_44 = 0x44,
    _RESERVED_45 = 0x45,
    _RESERVED_46 = 0x46,
    _RESERVED_47 = 0x47,
    _RESERVED_48 = 0x48,
    _RESERVED_49 = 0x49,
    _RESERVED_4a = 0x4a,
    _RESERVED_4b = 0x4b,
    _RESERVED_4c = 0x4c,
    _RESERVED_4d = 0x4d,
    _RESERVED_4e = 0x4e,
    _RESERVED_4f = 0x4f,
    _RESERVED_50 = 0x50,
    _RESERVED_51 = 0x51,
    _RESERVED_52 = 0x52,
    _RESERVED_53 = 0x53,
    _RESERVED_54 = 0x54,
    _RESERVED_55 = 0x55,
    _RESERVED_56 = 0x56,
    _RESERVED_57 = 0x57,
    _RESERVED_58 = 0x58,
    _RESERVED_59 = 0x59,
    _RESERVED_5a = 0x5a,
    _RESERVED_5b = 0x5b,
    _RESERVED_5c = 0x5c,
    _RESERVED_5d = 0x5d,
    _RESERVED_5e = 0x5e,
    _RESERVED_5f = 0x5f,
    _RESERVED_60 = 0x60,
    _RESERVED_61 = 0x61,
    _RESERVED_62 = 0x62,
    _RESERVED_63 = 0x63,
    _RESERVED_64 = 0x64,
    _RESERVED_65 = 0x65,
    _RESERVED_66 = 0x66,
    _RESERVED_67 = 0x67,
    _RESERVED_68 = 0x68,
    _RESERVED_69 = 0x69,
    _RESERVED_6a = 0x6a,
    _RESERVED_6b = 0x6b,
    _RESERVED_6c = 0x6c,
    _RESERVED_6d = 0x6d,
    _RESERVED_6e = 0x6e,
    _RESERVED_6f = 0x6f,
    _RESERVED_70 = 0x70,
    _RESERVED_71 = 0x71,
    _RESERVED_72 = 0x72,
    _RESERVED_73 = 0x73,
    _RESERVED_74 = 0x74,
    _RESERVED_75 = 0x75,
    _RESERVED_76 = 0x76,
    _RESERVED_77 = 0x77,
    _RESERVED_78 = 0x78,
    _RESERVED_79 = 0x79,
    _RESERVED_7a = 0x7a,
    _RESERVED_7b = 0x7b,
    _RESERVED_7c = 0x7c,
    _RESERVED_7d = 0x7d,
    _RESERVED_7e = 0x7e,
    _RESERVED_7f = 0x7f,
    _RESERVED_80 = 0x80,
    _RESERVED_81 = 0x81,
    _RESERVED_82 = 0x82,
    _RESERVED_83 = 0x83,
    _RESERVED_84 = 0x84,
    _RESERVED_85 = 0x85,
    _RESERVED_86 = 0x86,
    _RESERVED_87 = 0x87,
    _RESERVED_88 = 0x88,
    _RESERVED_89 = 0x89,
    _RESERVED_8a = 0x8a,
    _RESERVED_8b = 0x8b,
    _RESERVED_8c = 0x8c,
    _RESERVED_8d = 0x8d,
    _RESERVED_8e = 0x8e,
    _RESERVED_8f = 0x8f,
    _RESERVED_90 = 0x90,
    _RESERVED_91 = 0x91,
    _RESERVED_92 = 0x92,
    _RESERVED_93 = 0x93,
    _RESERVED_94 = 0x94,
    _RESERVED_95 = 0x95,
    _RESERVED_96 = 0x96,
    _RESERVED_97 = 0x97,
    _RESERVED_98 = 0x98,
    _RESERVED_99 = 0x99,
    _RESERVED_9a = 0x9a,
    _RESERVED_9b = 0x9b,
    _RESERVED_9c = 0x9c,
    _RESERVED_9d = 0x9d,
    _RESERVED_9e = 0x9e,
    _RESERVED_9f = 0x9f,
    _RESERVED_a0 = 0xa0,
    _RESERVED_a1 = 0xa1,
    _RESERVED_a2 = 0xa2,
    _RESERVED_a3 = 0xa3,
    _RESERVED_a4 = 0xa4,
    _RESERVED_a5 = 0xa5,
    _RESERVED_a6 = 0xa6,
    _RESERVED_a7 = 0xa7,
    _RESERVED_a8 = 0xa8,
    _RESERVED_a9 = 0xa9,
    _RESERVED_aa = 0xaa,
    _RESERVED_ab = 0xab,
    _RESERVED_ac = 0xac,
    _RESERVED_ad = 0xad,
    _RESERVED_ae = 0xae,
    _RESERVED_af = 0xaf,
    _RESERVED_b0 = 0xb0,
    _RESERVED_b1 = 0xb1,
    _RESERVED_b2 = 0xb2,
    _RESERVED_b3 = 0xb3,
    _RESERVED_b4 = 0xb4,
    _RESERVED_b5 = 0xb5,
    _RESERVED_b6 = 0xb6,
    _RESERVED_b7 = 0xb7,
    _RESERVED_b8 = 0xb8,
    _RESERVED_b9 = 0xb9,
    _RESERVED_ba = 0xba,
    _RESERVED_bb = 0xbb,
    _RESERVED_bc = 0xbc,
    _RESERVED_bd = 0xbd,
    _RESERVED_be = 0xbe,
    _RESERVED_bf = 0xbf,
    _RESERVED_c0 = 0xc0,
    _RESERVED_c1 = 0xc1,
    _RESERVED_c2 = 0xc2,
    _RESERVED_c3 = 0xc3,
    _RESERVED_c4 = 0xc4,
    _RESERVED_c5 = 0xc5,
    _RESERVED_c6 = 0xc6,
    _RESERVED_c7 = 0xc7,
    _RESERVED_c8 = 0xc8,
    _RESERVED_c9 = 0xc9,
    _RESERVED_ca = 0xca,
    _RESERVED_cb = 0xcb,
    _RESERVED_cc = 0xcc,
    _RESERVED_cd = 0xcd,
    _RESERVED_ce = 0xce,
    _RESERVED_cf = 0xcf,
    _RESERVED_d0 = 0xd0,
    _RESERVED_d1 = 0xd1,
    _RESERVED_d2 = 0xd2,
    _RESERVED_d3 = 0xd3,
    _RESERVED_d4 = 0xd4,
    _RESERVED_d5 = 0xd5,
    _RESERVED_d6 = 0xd6,
    _RESERVED_d7 = 0xd7,
    _RESERVED_d8 = 0xd8,
    _RESERVED_d9 = 0xd9,
    _RESERVED_da = 0xda,
    _RESERVED_db = 0xdb,
    _RESERVED_dc = 0xdc,
    _RESERVED_dd = 0xdd,
    _RESERVED_de = 0xde,
    _RESERVED_df = 0xdf,
    _RESERVED_e0 = 0xe0,
    _RESERVED_e1 = 0xe1,
    _RESERVED_e2 = 0xe2,
    _RESERVED_e3 = 0xe3,
    _RESERVED_e4 = 0xe4,
    _RESERVED_e5 = 0xe5,
    _RESERVED_e6 = 0xe6,
    _RESERVED_e7 = 0xe7,
    _RESERVED_e8 = 0xe8,
    _RESERVED_e9 = 0xe9,
    _RESERVED_ea = 0xea,
    _RESERVED_eb = 0xeb,
    _RESERVED_ec = 0xec,
    _RESERVED_ed = 0xed,
    _RESERVED_ee = 0xee,
    _RESERVED_ef = 0xef,
    _RESERVED_f0 = 0xf0,
    _RESERVED_f1 = 0xf1,
    _RESERVED_f2 = 0xf2,
    _RESERVED_f3 = 0xf3,
    _RESERVED_f4 = 0xf4,
    _RESERVED_f5 = 0xf5,
    _RESERVED_f6 = 0xf6,
    _RESERVED_f7 = 0xf7,
    _RESERVED_f8 = 0xf8,
    _RESERVED_f9 = 0xf9,
    _RESERVED_fa = 0xfa,
    _RESERVED_fb = 0xfb,
    _RESERVED_fc = 0xfc,
    _RESERVED_fd = 0xfd,
    _RESERVED_fe = 0xfe,
    _RESERVED_ff = 0xff,
}
impl Mult {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Mult {
        unsafe { core::mem::transmute(val & 0xff) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Mult {
    #[inline(always)]
    fn from(val: u8) -> Mult {
        Mult::from_bits(val)
    }
}
impl From<Mult> for u8 {
    #[inline(always)]
    fn from(val: Mult) -> u8 {
        Mult::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pfd0Clkgate {
    #[doc = "PFD0 clock is not gated."]
    NOT_GATED = 0x0,
    #[doc = "PFD0 clock is gated."]
    GATED = 0x01,
}
impl Pfd0Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfd0Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfd0Clkgate {
    #[inline(always)]
    fn from(val: u8) -> Pfd0Clkgate {
        Pfd0Clkgate::from_bits(val)
    }
}
impl From<Pfd0Clkgate> for u8 {
    #[inline(always)]
    fn from(val: Pfd0Clkgate) -> u8 {
        Pfd0Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pfd1Clkgate {
    #[doc = "PFD1 clock is not gated."]
    NOT_GATED = 0x0,
    #[doc = "PFD1 clock is gated."]
    GATED = 0x01,
}
impl Pfd1Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfd1Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfd1Clkgate {
    #[inline(always)]
    fn from(val: u8) -> Pfd1Clkgate {
        Pfd1Clkgate::from_bits(val)
    }
}
impl From<Pfd1Clkgate> for u8 {
    #[inline(always)]
    fn from(val: Pfd1Clkgate) -> u8 {
        Pfd1Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pfd2Clkgate {
    #[doc = "PFD2 clock is not gated."]
    NOT_GATED = 0x0,
    #[doc = "PFD2 clock is gated."]
    GATED = 0x01,
}
impl Pfd2Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfd2Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfd2Clkgate {
    #[inline(always)]
    fn from(val: u8) -> Pfd2Clkgate {
        Pfd2Clkgate::from_bits(val)
    }
}
impl From<Pfd2Clkgate> for u8 {
    #[inline(always)]
    fn from(val: Pfd2Clkgate) -> u8 {
        Pfd2Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pfd3Clkgate {
    #[doc = "PFD3 clock is not gated."]
    NOT_GATED = 0x0,
    #[doc = "PFD3 clock is gated."]
    GATED = 0x01,
}
impl Pfd3Clkgate {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pfd3Clkgate {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pfd3Clkgate {
    #[inline(always)]
    fn from(val: u8) -> Pfd3Clkgate {
        Pfd3Clkgate::from_bits(val)
    }
}
impl From<Pfd3Clkgate> for u8 {
    #[inline(always)]
    fn from(val: Pfd3Clkgate) -> u8 {
        Pfd3Clkgate::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0CasperClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0CasperClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0CasperClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0CasperClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0CasperClk {
        Pscctl0CasperClk::from_bits(val)
    }
}
impl From<Pscctl0CasperClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0CasperClk) -> u8 {
        Pscctl0CasperClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrCasperClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrCasperClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrCasperClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrCasperClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrCasperClk {
        Pscctl0ClrCasperClk::from_bits(val)
    }
}
impl From<Pscctl0ClrCasperClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrCasperClk) -> u8 {
        Pscctl0ClrCasperClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrFlexspiOtfadClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrFlexspiOtfadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrFlexspiOtfadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrFlexspiOtfadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrFlexspiOtfadClk {
        Pscctl0ClrFlexspiOtfadClk::from_bits(val)
    }
}
impl From<Pscctl0ClrFlexspiOtfadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrFlexspiOtfadClk) -> u8 {
        Pscctl0ClrFlexspiOtfadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrHashcryptClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrHashcryptClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrHashcryptClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrHashcryptClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrHashcryptClk {
        Pscctl0ClrHashcryptClk::from_bits(val)
    }
}
impl From<Pscctl0ClrHashcryptClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrHashcryptClk) -> u8 {
        Pscctl0ClrHashcryptClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrOtpClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrOtpClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrOtpClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrOtpClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrOtpClk {
        Pscctl0ClrOtpClk::from_bits(val)
    }
}
impl From<Pscctl0ClrOtpClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrOtpClk) -> u8 {
        Pscctl0ClrOtpClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrPowerquadClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrPowerquadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrPowerquadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrPowerquadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrPowerquadClk {
        Pscctl0ClrPowerquadClk::from_bits(val)
    }
}
impl From<Pscctl0ClrPowerquadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrPowerquadClk) -> u8 {
        Pscctl0ClrPowerquadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrPufClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrPufClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrPufClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrPufClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrPufClk {
        Pscctl0ClrPufClk::from_bits(val)
    }
}
impl From<Pscctl0ClrPufClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrPufClk) -> u8 {
        Pscctl0ClrPufClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrRngClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrRngClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrRngClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrRngClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrRngClk {
        Pscctl0ClrRngClk::from_bits(val)
    }
}
impl From<Pscctl0ClrRngClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrRngClk) -> u8 {
        Pscctl0ClrRngClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrRomCtl128kbClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrRomCtl128kbClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrRomCtl128kbClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrRomCtl128kbClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrRomCtl128kbClk {
        Pscctl0ClrRomCtl128kbClk::from_bits(val)
    }
}
impl From<Pscctl0ClrRomCtl128kbClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrRomCtl128kbClk) -> u8 {
        Pscctl0ClrRomCtl128kbClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrSctClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrSctClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrSctClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrSctClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrSctClk {
        Pscctl0ClrSctClk::from_bits(val)
    }
}
impl From<Pscctl0ClrSctClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrSctClk) -> u8 {
        Pscctl0ClrSctClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrUsbhsDeviceClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrUsbhsDeviceClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrUsbhsDeviceClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrUsbhsDeviceClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrUsbhsDeviceClk {
        Pscctl0ClrUsbhsDeviceClk::from_bits(val)
    }
}
impl From<Pscctl0ClrUsbhsDeviceClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrUsbhsDeviceClk) -> u8 {
        Pscctl0ClrUsbhsDeviceClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrUsbhsHostClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrUsbhsHostClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrUsbhsHostClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrUsbhsHostClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrUsbhsHostClk {
        Pscctl0ClrUsbhsHostClk::from_bits(val)
    }
}
impl From<Pscctl0ClrUsbhsHostClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrUsbhsHostClk) -> u8 {
        Pscctl0ClrUsbhsHostClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrUsbhsPhyClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrUsbhsPhyClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrUsbhsPhyClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrUsbhsPhyClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrUsbhsPhyClk {
        Pscctl0ClrUsbhsPhyClk::from_bits(val)
    }
}
impl From<Pscctl0ClrUsbhsPhyClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrUsbhsPhyClk) -> u8 {
        Pscctl0ClrUsbhsPhyClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0ClrUsbhsSramClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL0 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl0ClrUsbhsSramClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0ClrUsbhsSramClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0ClrUsbhsSramClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0ClrUsbhsSramClk {
        Pscctl0ClrUsbhsSramClk::from_bits(val)
    }
}
impl From<Pscctl0ClrUsbhsSramClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0ClrUsbhsSramClk) -> u8 {
        Pscctl0ClrUsbhsSramClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0FlexspiOtfadClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0FlexspiOtfadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0FlexspiOtfadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0FlexspiOtfadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0FlexspiOtfadClk {
        Pscctl0FlexspiOtfadClk::from_bits(val)
    }
}
impl From<Pscctl0FlexspiOtfadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0FlexspiOtfadClk) -> u8 {
        Pscctl0FlexspiOtfadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0HashcryptClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0HashcryptClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0HashcryptClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0HashcryptClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0HashcryptClk {
        Pscctl0HashcryptClk::from_bits(val)
    }
}
impl From<Pscctl0HashcryptClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0HashcryptClk) -> u8 {
        Pscctl0HashcryptClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0OtpClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0OtpClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0OtpClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0OtpClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0OtpClk {
        Pscctl0OtpClk::from_bits(val)
    }
}
impl From<Pscctl0OtpClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0OtpClk) -> u8 {
        Pscctl0OtpClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0PowerquadClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0PowerquadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0PowerquadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0PowerquadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0PowerquadClk {
        Pscctl0PowerquadClk::from_bits(val)
    }
}
impl From<Pscctl0PowerquadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0PowerquadClk) -> u8 {
        Pscctl0PowerquadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0PufClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0PufClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0PufClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0PufClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0PufClk {
        Pscctl0PufClk::from_bits(val)
    }
}
impl From<Pscctl0PufClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0PufClk) -> u8 {
        Pscctl0PufClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0RngClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0RngClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0RngClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0RngClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0RngClk {
        Pscctl0RngClk::from_bits(val)
    }
}
impl From<Pscctl0RngClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0RngClk) -> u8 {
        Pscctl0RngClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SctClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0SctClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SctClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SctClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SctClk {
        Pscctl0SctClk::from_bits(val)
    }
}
impl From<Pscctl0SctClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SctClk) -> u8 {
        Pscctl0SctClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetCasperClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetCasperClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetCasperClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetCasperClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetCasperClk {
        Pscctl0SetCasperClk::from_bits(val)
    }
}
impl From<Pscctl0SetCasperClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetCasperClk) -> u8 {
        Pscctl0SetCasperClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetFlexspiOtfadClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetFlexspiOtfadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetFlexspiOtfadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetFlexspiOtfadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetFlexspiOtfadClk {
        Pscctl0SetFlexspiOtfadClk::from_bits(val)
    }
}
impl From<Pscctl0SetFlexspiOtfadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetFlexspiOtfadClk) -> u8 {
        Pscctl0SetFlexspiOtfadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetHashcryptClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetHashcryptClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetHashcryptClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetHashcryptClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetHashcryptClk {
        Pscctl0SetHashcryptClk::from_bits(val)
    }
}
impl From<Pscctl0SetHashcryptClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetHashcryptClk) -> u8 {
        Pscctl0SetHashcryptClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetOtpClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetOtpClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetOtpClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetOtpClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetOtpClk {
        Pscctl0SetOtpClk::from_bits(val)
    }
}
impl From<Pscctl0SetOtpClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetOtpClk) -> u8 {
        Pscctl0SetOtpClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetPowerquadClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetPowerquadClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetPowerquadClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetPowerquadClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetPowerquadClk {
        Pscctl0SetPowerquadClk::from_bits(val)
    }
}
impl From<Pscctl0SetPowerquadClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetPowerquadClk) -> u8 {
        Pscctl0SetPowerquadClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetPufClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetPufClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetPufClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetPufClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetPufClk {
        Pscctl0SetPufClk::from_bits(val)
    }
}
impl From<Pscctl0SetPufClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetPufClk) -> u8 {
        Pscctl0SetPufClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetRngClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetRngClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetRngClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetRngClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetRngClk {
        Pscctl0SetRngClk::from_bits(val)
    }
}
impl From<Pscctl0SetRngClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetRngClk) -> u8 {
        Pscctl0SetRngClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetRomCtl128kbClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetRomCtl128kbClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetRomCtl128kbClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetRomCtl128kbClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetRomCtl128kbClk {
        Pscctl0SetRomCtl128kbClk::from_bits(val)
    }
}
impl From<Pscctl0SetRomCtl128kbClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetRomCtl128kbClk) -> u8 {
        Pscctl0SetRomCtl128kbClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetSctClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetSctClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetSctClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetSctClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetSctClk {
        Pscctl0SetSctClk::from_bits(val)
    }
}
impl From<Pscctl0SetSctClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetSctClk) -> u8 {
        Pscctl0SetSctClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetUsbhsDeviceClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetUsbhsDeviceClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetUsbhsDeviceClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetUsbhsDeviceClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetUsbhsDeviceClk {
        Pscctl0SetUsbhsDeviceClk::from_bits(val)
    }
}
impl From<Pscctl0SetUsbhsDeviceClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetUsbhsDeviceClk) -> u8 {
        Pscctl0SetUsbhsDeviceClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetUsbhsHostClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetUsbhsHostClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetUsbhsHostClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetUsbhsHostClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetUsbhsHostClk {
        Pscctl0SetUsbhsHostClk::from_bits(val)
    }
}
impl From<Pscctl0SetUsbhsHostClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetUsbhsHostClk) -> u8 {
        Pscctl0SetUsbhsHostClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetUsbhsPhyClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetUsbhsPhyClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetUsbhsPhyClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetUsbhsPhyClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetUsbhsPhyClk {
        Pscctl0SetUsbhsPhyClk::from_bits(val)
    }
}
impl From<Pscctl0SetUsbhsPhyClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetUsbhsPhyClk) -> u8 {
        Pscctl0SetUsbhsPhyClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0SetUsbhsSramClk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL0 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl0SetUsbhsSramClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0SetUsbhsSramClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0SetUsbhsSramClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0SetUsbhsSramClk {
        Pscctl0SetUsbhsSramClk::from_bits(val)
    }
}
impl From<Pscctl0SetUsbhsSramClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0SetUsbhsSramClk) -> u8 {
        Pscctl0SetUsbhsSramClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0UsbhsDeviceClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0UsbhsDeviceClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0UsbhsDeviceClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0UsbhsDeviceClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0UsbhsDeviceClk {
        Pscctl0UsbhsDeviceClk::from_bits(val)
    }
}
impl From<Pscctl0UsbhsDeviceClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0UsbhsDeviceClk) -> u8 {
        Pscctl0UsbhsDeviceClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0UsbhsHostClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0UsbhsHostClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0UsbhsHostClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0UsbhsHostClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0UsbhsHostClk {
        Pscctl0UsbhsHostClk::from_bits(val)
    }
}
impl From<Pscctl0UsbhsHostClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0UsbhsHostClk) -> u8 {
        Pscctl0UsbhsHostClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0UsbhsPhyClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0UsbhsPhyClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0UsbhsPhyClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0UsbhsPhyClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0UsbhsPhyClk {
        Pscctl0UsbhsPhyClk::from_bits(val)
    }
}
impl From<Pscctl0UsbhsPhyClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0UsbhsPhyClk) -> u8 {
        Pscctl0UsbhsPhyClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl0UsbhsSramClk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl0UsbhsSramClk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl0UsbhsSramClk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl0UsbhsSramClk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl0UsbhsSramClk {
        Pscctl0UsbhsSramClk::from_bits(val)
    }
}
impl From<Pscctl0UsbhsSramClk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl0UsbhsSramClk) -> u8 {
        Pscctl0UsbhsSramClk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1Acmp0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Acmp0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Acmp0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Acmp0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Acmp0Clk {
        Pscctl1Acmp0Clk::from_bits(val)
    }
}
impl From<Pscctl1Acmp0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Acmp0Clk) -> u8 {
        Pscctl1Acmp0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1Adc0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Adc0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Adc0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Adc0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Adc0Clk {
        Pscctl1Adc0Clk::from_bits(val)
    }
}
impl From<Pscctl1Adc0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Adc0Clk) -> u8 {
        Pscctl1Adc0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1ClrAcmp0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrAcmp0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrAcmp0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrAcmp0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrAcmp0Clk {
        Pscctl1ClrAcmp0Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrAcmp0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrAcmp0Clk) -> u8 {
        Pscctl1ClrAcmp0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1ClrAdc0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrAdc0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrAdc0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrAdc0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrAdc0Clk {
        Pscctl1ClrAdc0Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrAdc0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrAdc0Clk) -> u8 {
        Pscctl1ClrAdc0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1ClrSdio0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrSdio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrSdio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrSdio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrSdio0Clk {
        Pscctl1ClrSdio0Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrSdio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrSdio0Clk) -> u8 {
        Pscctl1ClrSdio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1ClrSdio1Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrSdio1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrSdio1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrSdio1Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrSdio1Clk {
        Pscctl1ClrSdio1Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrSdio1Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrSdio1Clk) -> u8 {
        Pscctl1ClrSdio1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1ClrShsgpio0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL1 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl1ClrShsgpio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1ClrShsgpio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1ClrShsgpio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1ClrShsgpio0Clk {
        Pscctl1ClrShsgpio0Clk::from_bits(val)
    }
}
impl From<Pscctl1ClrShsgpio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1ClrShsgpio0Clk) -> u8 {
        Pscctl1ClrShsgpio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1Sdio0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Sdio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Sdio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Sdio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Sdio0Clk {
        Pscctl1Sdio0Clk::from_bits(val)
    }
}
impl From<Pscctl1Sdio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Sdio0Clk) -> u8 {
        Pscctl1Sdio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1Sdio1Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Sdio1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Sdio1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Sdio1Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Sdio1Clk {
        Pscctl1Sdio1Clk::from_bits(val)
    }
}
impl From<Pscctl1Sdio1Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Sdio1Clk) -> u8 {
        Pscctl1Sdio1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1SetAcmp0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetAcmp0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetAcmp0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetAcmp0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetAcmp0Clk {
        Pscctl1SetAcmp0Clk::from_bits(val)
    }
}
impl From<Pscctl1SetAcmp0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetAcmp0Clk) -> u8 {
        Pscctl1SetAcmp0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1SetAdc0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetAdc0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetAdc0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetAdc0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetAdc0Clk {
        Pscctl1SetAdc0Clk::from_bits(val)
    }
}
impl From<Pscctl1SetAdc0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetAdc0Clk) -> u8 {
        Pscctl1SetAdc0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1SetSdio0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetSdio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetSdio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetSdio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetSdio0Clk {
        Pscctl1SetSdio0Clk::from_bits(val)
    }
}
impl From<Pscctl1SetSdio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetSdio0Clk) -> u8 {
        Pscctl1SetSdio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1SetSdio1Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetSdio1Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetSdio1Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetSdio1Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetSdio1Clk {
        Pscctl1SetSdio1Clk::from_bits(val)
    }
}
impl From<Pscctl1SetSdio1Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetSdio1Clk) -> u8 {
        Pscctl1SetSdio1Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1SetShsgpio0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL1 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl1SetShsgpio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1SetShsgpio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1SetShsgpio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1SetShsgpio0Clk {
        Pscctl1SetShsgpio0Clk::from_bits(val)
    }
}
impl From<Pscctl1SetShsgpio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1SetShsgpio0Clk) -> u8 {
        Pscctl1SetShsgpio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl1Shsgpio0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl1Shsgpio0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl1Shsgpio0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl1Shsgpio0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl1Shsgpio0Clk {
        Pscctl1Shsgpio0Clk::from_bits(val)
    }
}
impl From<Pscctl1Shsgpio0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl1Shsgpio0Clk) -> u8 {
        Pscctl1Shsgpio0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl2ClrUtick0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl2ClrUtick0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2ClrUtick0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2ClrUtick0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2ClrUtick0Clk {
        Pscctl2ClrUtick0Clk::from_bits(val)
    }
}
impl From<Pscctl2ClrUtick0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2ClrUtick0Clk) -> u8 {
        Pscctl2ClrUtick0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl2ClrWwdt0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Clears the PSCCTL2 Bit"]
    CLR_CLOCK = 0x01,
}
impl Pscctl2ClrWwdt0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2ClrWwdt0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2ClrWwdt0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2ClrWwdt0Clk {
        Pscctl2ClrWwdt0Clk::from_bits(val)
    }
}
impl From<Pscctl2ClrWwdt0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2ClrWwdt0Clk) -> u8 {
        Pscctl2ClrWwdt0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl2SetUtick0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl2SetUtick0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2SetUtick0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2SetUtick0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2SetUtick0Clk {
        Pscctl2SetUtick0Clk::from_bits(val)
    }
}
impl From<Pscctl2SetUtick0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2SetUtick0Clk) -> u8 {
        Pscctl2SetUtick0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl2SetWwdt0Clk {
    #[doc = "No Effect"]
    NO_EFFECT = 0x0,
    #[doc = "Sets the PSCCTL2 Bit"]
    SET_CLOCK = 0x01,
}
impl Pscctl2SetWwdt0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2SetWwdt0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2SetWwdt0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2SetWwdt0Clk {
        Pscctl2SetWwdt0Clk::from_bits(val)
    }
}
impl From<Pscctl2SetWwdt0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2SetWwdt0Clk) -> u8 {
        Pscctl2SetWwdt0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl2Utick0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl2Utick0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2Utick0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2Utick0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2Utick0Clk {
        Pscctl2Utick0Clk::from_bits(val)
    }
}
impl From<Pscctl2Utick0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2Utick0Clk) -> u8 {
        Pscctl2Utick0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Pscctl2Wwdt0Clk {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl Pscctl2Wwdt0Clk {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pscctl2Wwdt0Clk {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pscctl2Wwdt0Clk {
    #[inline(always)]
    fn from(val: u8) -> Pscctl2Wwdt0Clk {
        Pscctl2Wwdt0Clk::from_bits(val)
    }
}
impl From<Pscctl2Wwdt0Clk> for u8 {
    #[inline(always)]
    fn from(val: Pscctl2Wwdt0Clk) -> u8 {
        Pscctl2Wwdt0Clk::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Reset {
    #[doc = "SYSPLL0 reset is removed."]
    NORMAL = 0x0,
    #[doc = "SYSPLL0 is placed into reset."]
    FORCED_INTO_RESET = 0x01,
}
impl Reset {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Reset {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Reset {
    #[inline(always)]
    fn from(val: u8) -> Reset {
        Reset::from_bits(val)
    }
}
impl From<Reset> for u8 {
    #[inline(always)]
    fn from(val: Reset) -> u8 {
        Reset::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum RomCtl128kb {
    #[doc = "Disable Clock"]
    DISABLE_CLOCK = 0x0,
    #[doc = "Enable Clock"]
    ENABLE_CLOCK = 0x01,
}
impl RomCtl128kb {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> RomCtl128kb {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for RomCtl128kb {
    #[inline(always)]
    fn from(val: u8) -> RomCtl128kb {
        RomCtl128kb::from_bits(val)
    }
}
impl From<RomCtl128kb> for u8 {
    #[inline(always)]
    fn from(val: RomCtl128kb) -> u8 {
        RomCtl128kb::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SctfclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_SYS_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x04,
    #[doc = "AUDIO PLL Clock"]
    AUDIO_PLL_CLK = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl SctfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SctfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SctfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SctfclkselSel {
        SctfclkselSel::from_bits(val)
    }
}
impl From<SctfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SctfclkselSel) -> u8 {
        SctfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sdio0fclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_SYS_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Sdio0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdio0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdio0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sdio0fclkselSel {
        Sdio0fclkselSel::from_bits(val)
    }
}
impl From<Sdio0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sdio0fclkselSel) -> u8 {
        Sdio0fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Sdio1fclkselSel {
    #[doc = "Main Clock."]
    MAIN_CLK = 0x0,
    #[doc = "Main System PLL Clock."]
    MAIN_SYS_PLL_CLK = 0x01,
    #[doc = "SYSPLL0 AUX0_PLL_Clock."]
    SYSPLL0_AUX0_PLL_CLOCK = 0x02,
    #[doc = "FFRO Clock."]
    FFRO_CLK = 0x03,
    #[doc = "SYSPLL0 AUX1_PLL_Clock."]
    SYSPLL0_AUX1_PLL_CLOCK = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Sdio1fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sdio1fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sdio1fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Sdio1fclkselSel {
        Sdio1fclkselSel::from_bits(val)
    }
}
impl From<Sdio1fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Sdio1fclkselSel) -> u8 {
        Sdio1fclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SysoscbypassSel {
    #[doc = "External XTAL Clock."]
    EXT_XTAL_CLK = 0x0,
    #[doc = "Clock IN Clock."]
    CLOCK_IN_CLK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "NONE.this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl SysoscbypassSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SysoscbypassSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SysoscbypassSel {
    #[inline(always)]
    fn from(val: u8) -> SysoscbypassSel {
        SysoscbypassSel::from_bits(val)
    }
}
impl From<SysoscbypassSel> for u8 {
    #[inline(always)]
    fn from(val: SysoscbypassSel) -> u8 {
        SysoscbypassSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Syspll0clkselSel {
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x0,
    #[doc = "SYSXTALIN Clock."]
    SYSXTAL_CLK = 0x01,
    #[doc = "FFRO Clock Divided by 2."]
    FFRO_DIV_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Syspll0clkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Syspll0clkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Syspll0clkselSel {
    #[inline(always)]
    fn from(val: u8) -> Syspll0clkselSel {
        Syspll0clkselSel::from_bits(val)
    }
}
impl From<Syspll0clkselSel> for u8 {
    #[inline(always)]
    fn from(val: Syspll0clkselSel) -> u8 {
        Syspll0clkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SystickfclkselSel {
    #[doc = "Systick Divider Output Clock."]
    SYSTICK_DIV_CLK = 0x0,
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x01,
    #[doc = "32KHz RTC Clock."]
    RTC_32KHZ = 0x02,
    #[doc = "SFRO Clock."]
    SFRO_CLK = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl SystickfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SystickfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SystickfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> SystickfclkselSel {
        SystickfclkselSel::from_bits(val)
    }
}
impl From<SystickfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: SystickfclkselSel) -> u8 {
        SystickfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TrimRange {
    #[doc = "48MHz."]
    FFRO_48MHZ = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    #[doc = "60MHz."]
    FFRO_60MHZ = 0x03,
}
impl TrimRange {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> TrimRange {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for TrimRange {
    #[inline(always)]
    fn from(val: u8) -> TrimRange {
        TrimRange::from_bits(val)
    }
}
impl From<TrimRange> for u8 {
    #[inline(always)]
    fn from(val: TrimRange) -> u8 {
        TrimRange::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Update {
    #[doc = "Normal Mode."]
    NORMAL_MODE = 0x0,
    #[doc = "Update Safe Mode."]
    UPDATE_SAFE_MODE = 0x01,
}
impl Update {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Update {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Update {
    #[inline(always)]
    fn from(val: u8) -> Update {
        Update::from_bits(val)
    }
}
impl From<Update> for u8 {
    #[inline(always)]
    fn from(val: Update) -> u8 {
        Update::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UsbhsfclkselSel {
    #[doc = "XTALIN Clock."]
    XTALIN_CLK = 0x0,
    #[doc = "Main Clock."]
    MAIN_CLK = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl UsbhsfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UsbhsfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UsbhsfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> UsbhsfclkselSel {
        UsbhsfclkselSel::from_bits(val)
    }
}
impl From<UsbhsfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: UsbhsfclkselSel) -> u8 {
        UsbhsfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum UtickfclkselSel {
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl UtickfclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> UtickfclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for UtickfclkselSel {
    #[inline(always)]
    fn from(val: u8) -> UtickfclkselSel {
        UtickfclkselSel::from_bits(val)
    }
}
impl From<UtickfclkselSel> for u8 {
    #[inline(always)]
    fn from(val: UtickfclkselSel) -> u8 {
        UtickfclkselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wakeclk32khzselSel {
    #[doc = "32KHz"]
    FREQ_32KHZ = 0x0,
    #[doc = "LPOSC (Divided by 32 by default)."]
    LPOSC = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Wakeclk32khzselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wakeclk32khzselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wakeclk32khzselSel {
    #[inline(always)]
    fn from(val: u8) -> Wakeclk32khzselSel {
        Wakeclk32khzselSel::from_bits(val)
    }
}
impl From<Wakeclk32khzselSel> for u8 {
    #[inline(always)]
    fn from(val: Wakeclk32khzselSel) -> u8 {
        Wakeclk32khzselSel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Wdt0fclkselSel {
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    LPOSC = 0x0,
    _RESERVED_1 = 0x01,
    _RESERVED_2 = 0x02,
    _RESERVED_3 = 0x03,
    _RESERVED_4 = 0x04,
    _RESERVED_5 = 0x05,
    _RESERVED_6 = 0x06,
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    NONE = 0x07,
}
impl Wdt0fclkselSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Wdt0fclkselSel {
        unsafe { core::mem::transmute(val & 0x07) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Wdt0fclkselSel {
    #[inline(always)]
    fn from(val: u8) -> Wdt0fclkselSel {
        Wdt0fclkselSel::from_bits(val)
    }
}
impl From<Wdt0fclkselSel> for u8 {
    #[inline(always)]
    fn from(val: Wdt0fclkselSel) -> u8 {
        Wdt0fclkselSel::to_bits(val)
    }
}
