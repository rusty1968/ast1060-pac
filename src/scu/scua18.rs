#[doc = "Register `SCUA18` reader"]
pub type R = crate::R<Scua18Spec>;
#[doc = "Register `SCUA18` writer"]
pub type W = crate::W<Scua18Spec>;
#[doc = "Field `TheUpperBoundSRAMBaseAddrForCM4FInstruction` reader - The upper bound SRAM base address for CM4F instruction"]
pub type TheUpperBoundSrambaseAddrForCm4finstructionR = crate::FieldReader<u32>;
#[doc = "Field `TheUpperBoundSRAMBaseAddrForCM4FInstruction` writer - The upper bound SRAM base address for CM4F instruction"]
pub type TheUpperBoundSrambaseAddrForCm4finstructionW<'a, REG> =
    crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The upper bound SRAM base address for CM4F instruction"]
    #[inline(always)]
    pub fn the_upper_bound_srambase_addr_for_cm4finstruction(
        &self,
    ) -> TheUpperBoundSrambaseAddrForCm4finstructionR {
        TheUpperBoundSrambaseAddrForCm4finstructionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The upper bound SRAM base address for CM4F instruction"]
    #[inline(always)]
    pub fn the_upper_bound_srambase_addr_for_cm4finstruction(
        &mut self,
    ) -> TheUpperBoundSrambaseAddrForCm4finstructionW<Scua18Spec> {
        TheUpperBoundSrambaseAddrForCm4finstructionW::new(self, 0)
    }
}
#[doc = "CM4F Instruction Memory Address Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scua18Spec;
impl crate::RegisterSpec for Scua18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scua18::R`](R) reader structure"]
impl crate::Readable for Scua18Spec {}
#[doc = "`write(|w| ..)` method takes [`scua18::W`](W) writer structure"]
impl crate::Writable for Scua18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUA18 to value 0x2000_0000"]
impl crate::Resettable for Scua18Spec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
