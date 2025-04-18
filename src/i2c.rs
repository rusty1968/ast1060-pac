#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2cc00: I2cc00,
    i2cc04: I2cc04,
    i2cc08: I2cc08,
    i2cc0c: I2cc0c,
    i2cm10: I2cm10,
    i2cm14: I2cm14,
    i2cm18: I2cm18,
    i2cm1c: I2cm1c,
    i2cs20: I2cs20,
    i2cs24: I2cs24,
    i2cs28: I2cs28,
    i2cs2c: I2cs2c,
    i2cm30: I2cm30,
    i2cm34: I2cm34,
    i2cs38: I2cs38,
    i2cs3c: I2cs3c,
    i2cs40: I2cs40,
    _reserved17: [u8; 0x04],
    i2cm48: I2cm48,
    i2cs4c: I2cs4c,
    i2cc50: I2cc50,
    i2cc54: I2cc54,
}
impl RegisterBlock {
    #[doc = "0x00 - Master/Slave Function Control Register"]
    #[inline(always)]
    pub const fn i2cc00(&self) -> &I2cc00 {
        &self.i2cc00
    }
    #[doc = "0x04 - Master/Slave Clock and AC Timing Control Register \\#1"]
    #[inline(always)]
    pub const fn i2cc04(&self) -> &I2cc04 {
        &self.i2cc04
    }
    #[doc = "0x08 - Master/Slave Transmit/Receive Byte Buffer Register"]
    #[inline(always)]
    pub const fn i2cc08(&self) -> &I2cc08 {
        &self.i2cc08
    }
    #[doc = "0x0c - Master/Slave Pool Buffer Control Register"]
    #[inline(always)]
    pub const fn i2cc0c(&self) -> &I2cc0c {
        &self.i2cc0c
    }
    #[doc = "0x10 - Master Interrupt Control Register"]
    #[inline(always)]
    pub const fn i2cm10(&self) -> &I2cm10 {
        &self.i2cm10
    }
    #[doc = "0x14 - Master Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2cm14(&self) -> &I2cm14 {
        &self.i2cm14
    }
    #[doc = "0x18 - Master Command/Status Register"]
    #[inline(always)]
    pub const fn i2cm18(&self) -> &I2cm18 {
        &self.i2cm18
    }
    #[doc = "0x1c - \newver{Master DMA Transfer Length Register"]
    #[inline(always)]
    pub const fn i2cm1c(&self) -> &I2cm1c {
        &self.i2cm1c
    }
    #[doc = "0x20 - Slave Interrupt Control Register"]
    #[inline(always)]
    pub const fn i2cs20(&self) -> &I2cs20 {
        &self.i2cs20
    }
    #[doc = "0x24 - Slave Interrupt Status Register"]
    #[inline(always)]
    pub const fn i2cs24(&self) -> &I2cs24 {
        &self.i2cs24
    }
    #[doc = "0x28 - Slave Command/Status Register"]
    #[inline(always)]
    pub const fn i2cs28(&self) -> &I2cs28 {
        &self.i2cs28
    }
    #[doc = "0x2c - \newver{Slave DMA Transfer Length Register"]
    #[inline(always)]
    pub const fn i2cs2c(&self) -> &I2cs2c {
        &self.i2cs2c
    }
    #[doc = "0x30 - \newver{Master DMA Mode Tx Buffer Base Address"]
    #[inline(always)]
    pub const fn i2cm30(&self) -> &I2cm30 {
        &self.i2cm30
    }
    #[doc = "0x34 - \newver{Master DMA Mode Rx Buffer Base Address"]
    #[inline(always)]
    pub const fn i2cm34(&self) -> &I2cm34 {
        &self.i2cm34
    }
    #[doc = "0x38 - \newver{Slave~ DMA Mode Tx Buffer Base Address"]
    #[inline(always)]
    pub const fn i2cs38(&self) -> &I2cs38 {
        &self.i2cs38
    }
    #[doc = "0x3c - \newver{Slave~ DMA Mode Rx Buffer Base Address"]
    #[inline(always)]
    pub const fn i2cs3c(&self) -> &I2cs3c {
        &self.i2cs3c
    }
    #[doc = "0x40 - Slave Device Address Register"]
    #[inline(always)]
    pub const fn i2cs40(&self) -> &I2cs40 {
        &self.i2cs40
    }
    #[doc = "0x48 - Master DMA Length Status Register"]
    #[inline(always)]
    pub const fn i2cm48(&self) -> &I2cm48 {
        &self.i2cm48
    }
    #[doc = "0x4c - Slave DMA Length Status Register"]
    #[inline(always)]
    pub const fn i2cs4c(&self) -> &I2cs4c {
        &self.i2cs4c
    }
    #[doc = "0x50 - Current DMA Operating Address Status"]
    #[inline(always)]
    pub const fn i2cc50(&self) -> &I2cc50 {
        &self.i2cc50
    }
    #[doc = "0x54 - Current DMA Operating Length Status"]
    #[inline(always)]
    pub const fn i2cc54(&self) -> &I2cc54 {
        &self.i2cc54
    }
}
#[doc = "I2CC00 (rw) register accessor: Master/Slave Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc00`] module"]
#[doc(alias = "I2CC00")]
pub type I2cc00 = crate::Reg<i2cc00::I2cc00Spec>;
#[doc = "Master/Slave Function Control Register"]
pub mod i2cc00;
#[doc = "I2CC04 (rw) register accessor: Master/Slave Clock and AC Timing Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc04`] module"]
#[doc(alias = "I2CC04")]
pub type I2cc04 = crate::Reg<i2cc04::I2cc04Spec>;
#[doc = "Master/Slave Clock and AC Timing Control Register \\#1"]
pub mod i2cc04;
#[doc = "I2CC08 (rw) register accessor: Master/Slave Transmit/Receive Byte Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc08`] module"]
#[doc(alias = "I2CC08")]
pub type I2cc08 = crate::Reg<i2cc08::I2cc08Spec>;
#[doc = "Master/Slave Transmit/Receive Byte Buffer Register"]
pub mod i2cc08;
#[doc = "I2CC0C (rw) register accessor: Master/Slave Pool Buffer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc0c`] module"]
#[doc(alias = "I2CC0C")]
pub type I2cc0c = crate::Reg<i2cc0c::I2cc0cSpec>;
#[doc = "Master/Slave Pool Buffer Control Register"]
pub mod i2cc0c;
#[doc = "I2CM10 (rw) register accessor: Master Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm10`] module"]
#[doc(alias = "I2CM10")]
pub type I2cm10 = crate::Reg<i2cm10::I2cm10Spec>;
#[doc = "Master Interrupt Control Register"]
pub mod i2cm10;
#[doc = "I2CM14 (rw) register accessor: Master Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm14`] module"]
#[doc(alias = "I2CM14")]
pub type I2cm14 = crate::Reg<i2cm14::I2cm14Spec>;
#[doc = "Master Interrupt Status Register"]
pub mod i2cm14;
#[doc = "I2CM18 (rw) register accessor: Master Command/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm18`] module"]
#[doc(alias = "I2CM18")]
pub type I2cm18 = crate::Reg<i2cm18::I2cm18Spec>;
#[doc = "Master Command/Status Register"]
pub mod i2cm18;
#[doc = "I2CM1C (rw) register accessor: \newver{Master DMA Transfer Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm1c`] module"]
#[doc(alias = "I2CM1C")]
pub type I2cm1c = crate::Reg<i2cm1c::I2cm1cSpec>;
#[doc = "\newver{Master DMA Transfer Length Register"]
pub mod i2cm1c;
#[doc = "I2CS20 (rw) register accessor: Slave Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cs20`] module"]
#[doc(alias = "I2CS20")]
pub type I2cs20 = crate::Reg<i2cs20::I2cs20Spec>;
#[doc = "Slave Interrupt Control Register"]
pub mod i2cs20;
#[doc = "I2CS24 (rw) register accessor: Slave Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cs24`] module"]
#[doc(alias = "I2CS24")]
pub type I2cs24 = crate::Reg<i2cs24::I2cs24Spec>;
#[doc = "Slave Interrupt Status Register"]
pub mod i2cs24;
#[doc = "I2CS28 (rw) register accessor: Slave Command/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cs28`] module"]
#[doc(alias = "I2CS28")]
pub type I2cs28 = crate::Reg<i2cs28::I2cs28Spec>;
#[doc = "Slave Command/Status Register"]
pub mod i2cs28;
#[doc = "I2CS2C (rw) register accessor: \newver{Slave DMA Transfer Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cs2c`] module"]
#[doc(alias = "I2CS2C")]
pub type I2cs2c = crate::Reg<i2cs2c::I2cs2cSpec>;
#[doc = "\newver{Slave DMA Transfer Length Register"]
pub mod i2cs2c;
#[doc = "I2CM30 (rw) register accessor: \newver{Master DMA Mode Tx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm30`] module"]
#[doc(alias = "I2CM30")]
pub type I2cm30 = crate::Reg<i2cm30::I2cm30Spec>;
#[doc = "\newver{Master DMA Mode Tx Buffer Base Address"]
pub mod i2cm30;
#[doc = "I2CM34 (rw) register accessor: \newver{Master DMA Mode Rx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm34`] module"]
#[doc(alias = "I2CM34")]
pub type I2cm34 = crate::Reg<i2cm34::I2cm34Spec>;
#[doc = "\newver{Master DMA Mode Rx Buffer Base Address"]
pub mod i2cm34;
#[doc = "I2CS38 (rw) register accessor: \newver{Slave~ DMA Mode Tx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cs38`] module"]
#[doc(alias = "I2CS38")]
pub type I2cs38 = crate::Reg<i2cs38::I2cs38Spec>;
#[doc = "\newver{Slave~ DMA Mode Tx Buffer Base Address"]
pub mod i2cs38;
#[doc = "I2CS3C (rw) register accessor: \newver{Slave~ DMA Mode Rx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs3c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs3c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cs3c`] module"]
#[doc(alias = "I2CS3C")]
pub type I2cs3c = crate::Reg<i2cs3c::I2cs3cSpec>;
#[doc = "\newver{Slave~ DMA Mode Rx Buffer Base Address"]
pub mod i2cs3c;
#[doc = "I2CS40 (rw) register accessor: Slave Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cs40`] module"]
#[doc(alias = "I2CS40")]
pub type I2cs40 = crate::Reg<i2cs40::I2cs40Spec>;
#[doc = "Slave Device Address Register"]
pub mod i2cs40;
#[doc = "I2CM48 (rw) register accessor: Master DMA Length Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm48`] module"]
#[doc(alias = "I2CM48")]
pub type I2cm48 = crate::Reg<i2cm48::I2cm48Spec>;
#[doc = "Master DMA Length Status Register"]
pub mod i2cm48;
#[doc = "I2CS4C (rw) register accessor: Slave DMA Length Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cs4c`] module"]
#[doc(alias = "I2CS4C")]
pub type I2cs4c = crate::Reg<i2cs4c::I2cs4cSpec>;
#[doc = "Slave DMA Length Status Register"]
pub mod i2cs4c;
#[doc = "I2CC50 (rw) register accessor: Current DMA Operating Address Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc50`] module"]
#[doc(alias = "I2CC50")]
pub type I2cc50 = crate::Reg<i2cc50::I2cc50Spec>;
#[doc = "Current DMA Operating Address Status"]
pub mod i2cc50;
#[doc = "I2CC54 (rw) register accessor: Current DMA Operating Length Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc54`] module"]
#[doc(alias = "I2CC54")]
pub type I2cc54 = crate::Reg<i2cc54::I2cc54Spec>;
#[doc = "Current DMA Operating Length Status"]
pub mod i2cc54;
