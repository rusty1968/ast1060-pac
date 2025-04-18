#[doc = "Register `SECURE044` reader"]
pub type R = crate::R<Secure044Spec>;
#[doc = "Register `SECURE044` writer"]
pub type W = crate::W<Secure044Spec>;
#[doc = "Field `OTPQMRDataReadBack` reader - OTP QMR data read back"]
pub type OtpqmrdataReadBackR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - OTP QMR data read back"]
    #[inline(always)]
    pub fn otpqmrdata_read_back(&self) -> OtpqmrdataReadBackR {
        OtpqmrdataReadBackR::new(self.bits)
    }
}
impl W {}
#[doc = "OTP QMR data read back\n\nYou can [`read`](crate::Reg::read) this register and get [`secure044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure044Spec;
impl crate::RegisterSpec for Secure044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure044::R`](R) reader structure"]
impl crate::Readable for Secure044Spec {}
#[doc = "`write(|w| ..)` method takes [`secure044::W`](W) writer structure"]
impl crate::Writable for Secure044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE044 to value 0"]
impl crate::Resettable for Secure044Spec {}
