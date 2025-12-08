#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub tr: crate::Reg<tr::TR_SPEC>,
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub dr: crate::Reg<dr::DR_SPEC>,
    #[doc = "0x08 - RTC control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x0c - This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub prer: crate::Reg<prer::PRER_SPEC>,
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub wutr: crate::Reg<wutr::WUTR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x1c..0x24 - This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub alrmr: [crate::Reg<alrmr::ALRMR_SPEC>; 2],
    #[doc = "0x24 - RTC write protection register"]
    pub wpr: crate::Reg<wpr::WPR_SPEC>,
    #[doc = "0x28 - RTC sub second register"]
    pub ssr: crate::Reg<ssr::SSR_SPEC>,
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub shiftr: crate::Reg<shiftr::SHIFTR_SPEC>,
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    pub tstr: crate::Reg<tstr::TSTR_SPEC>,
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    pub tsdr: crate::Reg<tsdr::TSDR_SPEC>,
    #[doc = "0x38 - The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
    pub tsssr: crate::Reg<tsssr::TSSSR_SPEC>,
    #[doc = "0x3c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    pub calr: crate::Reg<calr::CALR_SPEC>,
    #[doc = "0x40 - RTC tamper and alternate function configuration register"]
    pub tampcr: crate::Reg<tampcr::TAMPCR_SPEC>,
    #[doc = "0x44..0x4c - This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
    pub alrmssr: [crate::Reg<alrmssr::ALRMSSR_SPEC>; 2],
    #[doc = "0x4c - RTC option register"]
    pub or: crate::Reg<or::OR_SPEC>,
    #[doc = "0x50..0xd0 - RTC backup registers"]
    pub bkpr: [crate::Reg<bkpr::BKPR_SPEC>; 32],
}
impl RegisterBlock {
    #[doc = "0x1c - This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub fn alrmar(&self) -> &crate::Reg<alrmr::ALRMR_SPEC> {
        &self.alrmr[0]
    }
    #[doc = "0x20 - This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub fn alrmbr(&self) -> &crate::Reg<alrmr::ALRMR_SPEC> {
        &self.alrmr[1]
    }
    #[doc = "0x44 - This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
    #[inline(always)]
    pub fn alrmassr(&self) -> &crate::Reg<alrmssr::ALRMSSR_SPEC> {
        &self.alrmssr[0]
    }
    #[doc = "0x48 - This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
    #[inline(always)]
    pub fn alrmbssr(&self) -> &crate::Reg<alrmssr::ALRMSSR_SPEC> {
        &self.alrmssr[1]
    }
}
#[doc = "TR register accessor: an alias for `Reg<TR_SPEC>`"]
pub type TR = crate::Reg<tr::TR_SPEC>;
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod tr;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod dr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "RTC control register"]
pub mod cr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "This register is write protected (except for RTC_ISR\\[13:8\\]
bits). The write access procedure is described in RTC register write protection on page9."]
pub mod isr;
#[doc = "PRER register accessor: an alias for `Reg<PRER_SPEC>`"]
pub type PRER = crate::Reg<prer::PRER_SPEC>;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod prer;
#[doc = "WUTR register accessor: an alias for `Reg<WUTR_SPEC>`"]
pub type WUTR = crate::Reg<wutr::WUTR_SPEC>;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod wutr;
#[doc = "ALRMR register accessor: an alias for `Reg<ALRMR_SPEC>`"]
pub type ALRMR = crate::Reg<alrmr::ALRMR_SPEC>;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod alrmr;
#[doc = "WPR register accessor: an alias for `Reg<WPR_SPEC>`"]
pub type WPR = crate::Reg<wpr::WPR_SPEC>;
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "SSR register accessor: an alias for `Reg<SSR_SPEC>`"]
pub type SSR = crate::Reg<ssr::SSR_SPEC>;
#[doc = "RTC sub second register"]
pub mod ssr;
#[doc = "SHIFTR register accessor: an alias for `Reg<SHIFTR_SPEC>`"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod shiftr;
#[doc = "TSTR register accessor: an alias for `Reg<TSTR_SPEC>`"]
pub type TSTR = crate::Reg<tstr::TSTR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod tstr;
#[doc = "TSDR register accessor: an alias for `Reg<TSDR_SPEC>`"]
pub type TSDR = crate::Reg<tsdr::TSDR_SPEC>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod tsdr;
#[doc = "TSSSR register accessor: an alias for `Reg<TSSSR_SPEC>`"]
pub type TSSSR = crate::Reg<tsssr::TSSSR_SPEC>;
#[doc = "The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
pub mod tsssr;
#[doc = "CALR register accessor: an alias for `Reg<CALR_SPEC>`"]
pub type CALR = crate::Reg<calr::CALR_SPEC>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod calr;
#[doc = "TAMPCR register accessor: an alias for `Reg<TAMPCR_SPEC>`"]
pub type TAMPCR = crate::Reg<tampcr::TAMPCR_SPEC>;
#[doc = "RTC tamper and alternate function configuration register"]
pub mod tampcr;
#[doc = "ALRMSSR register accessor: an alias for `Reg<ALRMSSR_SPEC>`"]
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSR_SPEC>;
#[doc = "This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
pub mod alrmssr;
#[doc = "BKPR register accessor: an alias for `Reg<BKPR_SPEC>`"]
pub type BKPR = crate::Reg<bkpr::BKPR_SPEC>;
#[doc = "RTC backup registers"]
pub mod bkpr;
#[doc = "OR register accessor: an alias for `Reg<OR_SPEC>`"]
pub type OR = crate::Reg<or::OR_SPEC>;
#[doc = "RTC option register"]
pub mod or;
