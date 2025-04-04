#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hace00: Hace00,
}
impl RegisterBlock {
    #[doc = "0x00 - Crypto Data Source Base Address Register"]
    #[inline(always)]
    pub const fn hace00(&self) -> &Hace00 {
        &self.hace00
    }
}
#[doc = "HACE00 (rw) register accessor: Crypto Data Source Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace00`]
module"]
#[doc(alias = "HACE00")]
pub type Hace00 = crate::Reg<hace00::Hace00Spec>;
#[doc = "Crypto Data Source Base Address Register"]
pub mod hace00;
