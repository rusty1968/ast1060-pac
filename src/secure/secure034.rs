#[doc = "Register `SECURE034` reader"]
pub type R = crate::R<Secure034Spec>;
#[doc = "Register `SECURE034` writer"]
pub type W = crate::W<Secure034Spec>;
#[doc = "Field `OTPTRAPDataReadBack2` reader - OTPTRAP data read back 2"]
pub type OtptrapdataReadBack2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OTPTRAP data read back 2"]
    #[inline(always)]
    pub fn otptrapdata_read_back2(&self) -> OtptrapdataReadBack2R {
        OtptrapdataReadBack2R::new(self.bits)
    }
}
impl W {}
#[doc = "OTPTRAP data read back 2\n\nYou can [`read`](crate::Reg::read) this register and get [`secure034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure034Spec;
impl crate::RegisterSpec for Secure034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure034::R`](R) reader structure"]
impl crate::Readable for Secure034Spec {}
#[doc = "`write(|w| ..)` method takes [`secure034::W`](W) writer structure"]
impl crate::Writable for Secure034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE034 to value 0"]
impl crate::Resettable for Secure034Spec {}
