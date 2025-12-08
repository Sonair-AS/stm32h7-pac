#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - HSEM register HSEM_R%s HSEM_R31"]
    pub r: [crate::Reg<r::R_SPEC>; 32],
    #[doc = "0x80..0x100 - HSEM Read lock register"]
    pub rlr: [crate::Reg<rlr::RLR_SPEC>; 32],
    #[doc = "0x100 - HSEM Interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x104 - HSEM Interrupt clear register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x108 - HSEM Interrupt status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    pub misr: crate::Reg<misr::MISR_SPEC>,
    _reserved6: [u8; 0x30],
    #[doc = "0x140 - HSEM Clear register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x144 - HSEM Interrupt clear register"]
    pub keyr: crate::Reg<keyr::KEYR_SPEC>,
}
#[doc = "R register accessor: an alias for `Reg<R_SPEC>`"]
pub type R = crate::Reg<r::R_SPEC>;
#[doc = "HSEM register HSEM_R%s HSEM_R31"]
pub mod r;
#[doc = "RLR register accessor: an alias for `Reg<RLR_SPEC>`"]
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
#[doc = "HSEM Read lock register"]
pub mod rlr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "HSEM Interrupt enable register"]
pub mod ier;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod icr;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "HSEM Interrupt status register"]
pub mod isr;
#[doc = "MISR register accessor: an alias for `Reg<MISR_SPEC>`"]
pub type MISR = crate::Reg<misr::MISR_SPEC>;
#[doc = "HSEM Masked interrupt status register"]
pub mod misr;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "HSEM Clear register"]
pub mod cr;
#[doc = "KEYR register accessor: an alias for `Reg<KEYR_SPEC>`"]
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
#[doc = "HSEM Interrupt clear register"]
pub mod keyr;
