#[doc = "Register `SCU514` reader"]
pub type R = crate::R<Scu514Spec>;
#[doc = "Register `SCU514` writer"]
pub type W = crate::W<Scu514Spec>;
#[doc = "Field `SCU510HwStrap2ClearReg` reader - SCU510 Hardware Strap2 Clear Register"]
pub type Scu510hwStrap2clearRegR = crate::FieldReader<u32>;
#[doc = "Field `SCU510HwStrap2ClearReg` writer - SCU510 Hardware Strap2 Clear Register"]
pub type Scu510hwStrap2clearRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU510 Hardware Strap2 Clear Register"]
    #[inline(always)]
    pub fn scu510hw_strap2clear_reg(&self) -> Scu510hwStrap2clearRegR {
        Scu510hwStrap2clearRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU510 Hardware Strap2 Clear Register"]
    #[inline(always)]
    pub fn scu510hw_strap2clear_reg(&mut self) -> Scu510hwStrap2clearRegW<Scu514Spec> {
        Scu510hwStrap2clearRegW::new(self, 0)
    }
}
#[doc = "Hardware Strap2 Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu514::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu514::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu514Spec;
impl crate::RegisterSpec for Scu514Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu514::R`](R) reader structure"]
impl crate::Readable for Scu514Spec {}
#[doc = "`write(|w| ..)` method takes [`scu514::W`](W) writer structure"]
impl crate::Writable for Scu514Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU514 to value 0"]
impl crate::Resettable for Scu514Spec {}
