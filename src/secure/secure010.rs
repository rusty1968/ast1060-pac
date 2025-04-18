#[doc = "Register `SECURE010` reader"]
pub type R = crate::R<Secure010Spec>;
#[doc = "Register `SECURE010` writer"]
pub type W = crate::W<Secure010Spec>;
#[doc = "Field `OTPAddrRegiste` reader - OTP Address Registe"]
pub type OtpaddrRegisteR = crate::FieldReader<u16>;
#[doc = "Field `OTPAddrRegiste` writer - OTP Address Registe"]
pub type OtpaddrRegisteW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - OTP Address Registe"]
    #[inline(always)]
    pub fn otpaddr_registe(&self) -> OtpaddrRegisteR {
        OtpaddrRegisteR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - OTP Address Registe"]
    #[inline(always)]
    pub fn otpaddr_registe(&mut self) -> OtpaddrRegisteW<Secure010Spec> {
        OtpaddrRegisteW::new(self, 0)
    }
}
#[doc = "OTP Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure010Spec;
impl crate::RegisterSpec for Secure010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure010::R`](R) reader structure"]
impl crate::Readable for Secure010Spec {}
#[doc = "`write(|w| ..)` method takes [`secure010::W`](W) writer structure"]
impl crate::Writable for Secure010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE010 to value 0"]
impl crate::Resettable for Secure010Spec {}
