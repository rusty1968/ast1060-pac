#[doc = "Register `HACE00` reader"]
pub type R = crate::R<Hace00Spec>;
#[doc = "Register `HACE00` writer"]
pub type W = crate::W<Hace00Spec>;
#[doc = "Field `BASE_ADDRESS` reader - Base address of crypto source data"]
pub type BaseAddressR = crate::FieldReader<u32>;
#[doc = "Field `BASE_ADDRESS` writer - Base address of crypto source data"]
pub type BaseAddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Base address of crypto source data"]
    #[inline(always)]
    pub fn base_address(&self) -> BaseAddressR {
        BaseAddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Base address of crypto source data"]
    #[inline(always)]
    pub fn base_address(&mut self) -> BaseAddressW<Hace00Spec> {
        BaseAddressW::new(self, 0)
    }
}
#[doc = "Crypto Data Source Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace00Spec;
impl crate::RegisterSpec for Hace00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace00::R`](R) reader structure"]
impl crate::Readable for Hace00Spec {}
#[doc = "`write(|w| ..)` method takes [`hace00::W`](W) writer structure"]
impl crate::Writable for Hace00Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HACE00 to value 0"]
impl crate::Resettable for Hace00Spec {
    const RESET_VALUE: u32 = 0;
}
