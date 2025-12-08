#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control and status register"]
    pub csr: crate::Reg<csr::CSR_SPEC>,
    #[doc = "0x04 - Argument register"]
    pub wdata: crate::Reg<wdata::WDATA_SPEC>,
    #[doc = "0x08 - Result register"]
    pub rdata: crate::Reg<rdata::RDATA_SPEC>,
}
#[doc = "CSR register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "Control and status register"]
pub mod csr;
#[doc = "WDATA register accessor: an alias for `Reg<WDATA_SPEC>`"]
pub type WDATA = crate::Reg<wdata::WDATA_SPEC>;
#[doc = "Argument register"]
pub mod wdata;
#[doc = "RDATA register accessor: an alias for `Reg<RDATA_SPEC>`"]
pub type RDATA = crate::Reg<rdata::RDATA_SPEC>;
#[doc = "Result register"]
pub mod rdata;
