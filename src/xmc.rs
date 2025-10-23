#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bk1ctrl1: Bk1ctrl1,
    bk1tmg1: Bk1tmg1,
    _reserved2: [u8; 0x10],
    bk1ctrl4: Bk1ctrl4,
    bk1tmg4: Bk1tmg4,
    _reserved4: [u8; 0x40],
    bk2ctrl: Bk2ctrl,
    bk2is: Bk2is,
    bk2tmgmem: Bk2tmgmem,
    bk2tmgatt: Bk2tmgatt,
    _reserved8: [u8; 0x04],
    bk2ecc: Bk2ecc,
    _reserved9: [u8; 0x8c],
    bk1tmgwr1: Bk1tmgwr1,
    _reserved10: [u8; 0x14],
    bk1tmgwr4: Bk1tmgwr4,
    _reserved11: [u8; 0x0100],
    ext1: Ext1,
    _reserved12: [u8; 0x08],
    ext4: Ext4,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    #[inline(always)]
    pub const fn bk1ctrl1(&self) -> &Bk1ctrl1 {
        &self.bk1ctrl1
    }
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    #[inline(always)]
    pub const fn bk1tmg1(&self) -> &Bk1tmg1 {
        &self.bk1tmg1
    }
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    #[inline(always)]
    pub const fn bk1ctrl4(&self) -> &Bk1ctrl4 {
        &self.bk1ctrl4
    }
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    #[inline(always)]
    pub const fn bk1tmg4(&self) -> &Bk1tmg4 {
        &self.bk1tmg4
    }
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    #[inline(always)]
    pub const fn bk2ctrl(&self) -> &Bk2ctrl {
        &self.bk2ctrl
    }
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    #[inline(always)]
    pub const fn bk2is(&self) -> &Bk2is {
        &self.bk2is
    }
    #[doc = "0x68 - Regular memory space timing register 2"]
    #[inline(always)]
    pub const fn bk2tmgmem(&self) -> &Bk2tmgmem {
        &self.bk2tmgmem
    }
    #[doc = "0x6c - special memory space timing register 2"]
    #[inline(always)]
    pub const fn bk2tmgatt(&self) -> &Bk2tmgatt {
        &self.bk2tmgatt
    }
    #[doc = "0x74 - ECC result register 2"]
    #[inline(always)]
    pub const fn bk2ecc(&self) -> &Bk2ecc {
        &self.bk2ecc
    }
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    #[inline(always)]
    pub const fn bk1tmgwr1(&self) -> &Bk1tmgwr1 {
        &self.bk1tmgwr1
    }
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    #[inline(always)]
    pub const fn bk1tmgwr4(&self) -> &Bk1tmgwr4 {
        &self.bk1tmgwr4
    }
    #[doc = "0x220 - externl timeing register 1"]
    #[inline(always)]
    pub const fn ext1(&self) -> &Ext1 {
        &self.ext1
    }
    #[doc = "0x22c - externl timeing register 4"]
    #[inline(always)]
    pub const fn ext4(&self) -> &Ext4 {
        &self.ext4
    }
}
#[doc = "BK1CTRL1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl1`] module"]
#[doc(alias = "BK1CTRL1")]
pub type Bk1ctrl1 = crate::Reg<bk1ctrl1::Bk1ctrl1Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bk1ctrl1;
#[doc = "BK1TMG1 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg1`] module"]
#[doc(alias = "BK1TMG1")]
pub type Bk1tmg1 = crate::Reg<bk1tmg1::Bk1tmg1Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod bk1tmg1;
#[doc = "BK1CTRL4 (rw) register accessor: SRAM/NOR-Flash chip-select control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1ctrl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1ctrl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl4`] module"]
#[doc(alias = "BK1CTRL4")]
pub type Bk1ctrl4 = crate::Reg<bk1ctrl4::Bk1ctrl4Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bk1ctrl4;
#[doc = "BK1TMG4 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmg4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmg4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg4`] module"]
#[doc(alias = "BK1TMG4")]
pub type Bk1tmg4 = crate::Reg<bk1tmg4::Bk1tmg4Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod bk1tmg4;
#[doc = "BK2CTRL (rw) register accessor: PC Card/NAND Flash control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2ctrl`] module"]
#[doc(alias = "BK2CTRL")]
pub type Bk2ctrl = crate::Reg<bk2ctrl::Bk2ctrlSpec>;
#[doc = "PC Card/NAND Flash control register 2"]
pub mod bk2ctrl;
#[doc = "BK2IS (rw) register accessor: FIFO status and interrupt register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2is::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2is::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2is`] module"]
#[doc(alias = "BK2IS")]
pub type Bk2is = crate::Reg<bk2is::Bk2isSpec>;
#[doc = "FIFO status and interrupt register 2"]
pub mod bk2is;
#[doc = "BK2TMGMEM (rw) register accessor: Regular memory space timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2tmgmem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2tmgmem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2tmgmem`] module"]
#[doc(alias = "BK2TMGMEM")]
pub type Bk2tmgmem = crate::Reg<bk2tmgmem::Bk2tmgmemSpec>;
#[doc = "Regular memory space timing register 2"]
pub mod bk2tmgmem;
#[doc = "BK2TMGATT (rw) register accessor: special memory space timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2tmgatt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2tmgatt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2tmgatt`] module"]
#[doc(alias = "BK2TMGATT")]
pub type Bk2tmgatt = crate::Reg<bk2tmgatt::Bk2tmgattSpec>;
#[doc = "special memory space timing register 2"]
pub mod bk2tmgatt;
#[doc = "BK2ECC (rw) register accessor: ECC result register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2ecc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2ecc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2ecc`] module"]
#[doc(alias = "BK2ECC")]
pub type Bk2ecc = crate::Reg<bk2ecc::Bk2eccSpec>;
#[doc = "ECC result register 2"]
pub mod bk2ecc;
#[doc = "BK1TMGWR1 (rw) register accessor: SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmgwr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmgwr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr1`] module"]
#[doc(alias = "BK1TMGWR1")]
pub type Bk1tmgwr1 = crate::Reg<bk1tmgwr1::Bk1tmgwr1Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bk1tmgwr1;
#[doc = "BK1TMGWR4 (rw) register accessor: SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::Reg::read) this register and get [`bk1tmgwr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk1tmgwr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr4`] module"]
#[doc(alias = "BK1TMGWR4")]
pub type Bk1tmgwr4 = crate::Reg<bk1tmgwr4::Bk1tmgwr4Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bk1tmgwr4;
#[doc = "EXT1 (rw) register accessor: externl timeing register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ext1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext1`] module"]
#[doc(alias = "EXT1")]
pub type Ext1 = crate::Reg<ext1::Ext1Spec>;
#[doc = "externl timeing register 1"]
pub mod ext1;
#[doc = "EXT4 (rw) register accessor: externl timeing register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ext4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext4`] module"]
#[doc(alias = "EXT4")]
pub type Ext4 = crate::Reg<ext4::Ext4Spec>;
#[doc = "externl timeing register 4"]
pub mod ext4;
