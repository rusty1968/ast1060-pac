#[doc = "Register `MI3C18` reader"]
pub type R = crate::R<Mi3c18Spec>;
#[doc = "Register `MI3C18` writer"]
pub type W = crate::W<Mi3c18Spec>;
#[doc = "Field `MasterModeDebugSigs` reader - - Master Mode Debug Signals"]
pub type MasterModeDebugSigsR = crate::FieldReader<u32>;
#[doc = "Field `SlaveModeDebugSigs` reader - - Slave Mode Debug Signals"]
pub type SlaveModeDebugSigsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - - Master Mode Debug Signals"]
    #[inline(always)]
    pub fn master_mode_debug_sigs(&self) -> MasterModeDebugSigsR {
        MasterModeDebugSigsR::new(self.bits)
    }
    #[doc = "Bits 0:31 - - Slave Mode Debug Signals"]
    #[inline(always)]
    pub fn slave_mode_debug_sigs(&self) -> SlaveModeDebugSigsR {
        SlaveModeDebugSigsR::new(self.bits)
    }
}
impl W {}
#[doc = "I3C1Dbg1 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mi3c18Spec;
impl crate::RegisterSpec for Mi3c18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi3c18::R`](R) reader structure"]
impl crate::Readable for Mi3c18Spec {}
#[doc = "`write(|w| ..)` method takes [`mi3c18::W`](W) writer structure"]
impl crate::Writable for Mi3c18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MI3C18 to value 0"]
impl crate::Resettable for Mi3c18Spec {}
