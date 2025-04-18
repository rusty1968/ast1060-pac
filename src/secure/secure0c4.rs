#[doc = "Register `SECURE0C4` reader"]
pub type R = crate::R<Secure0c4Spec>;
#[doc = "Register `SECURE0C4` writer"]
pub type W = crate::W<Secure0c4Spec>;
#[doc = "Field `SecBootRSAEngReadyINTStatus` reader - Secure Boot RSA Engine Ready Interrupt Status"]
pub type SecBootRsaengReadyIntstatusR = crate::BitReader;
#[doc = "Field `SecBootRSAEngReadyINTStatus` writer - Secure Boot RSA Engine Ready Interrupt Status"]
pub type SecBootRsaengReadyIntstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SecBootECCEngReadyINTStatus` reader - Secure Boot ECC Engine Ready Interrupt Status"]
pub type SecBootEccengReadyIntstatusR = crate::BitReader;
#[doc = "Field `SecBootECCEngReadyINTStatus` writer - Secure Boot ECC Engine Ready Interrupt Status"]
pub type SecBootEccengReadyIntstatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Secure Boot RSA Engine Ready Interrupt Status"]
    #[inline(always)]
    pub fn sec_boot_rsaeng_ready_intstatus(&self) -> SecBootRsaengReadyIntstatusR {
        SecBootRsaengReadyIntstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure Boot ECC Engine Ready Interrupt Status"]
    #[inline(always)]
    pub fn sec_boot_ecceng_ready_intstatus(&self) -> SecBootEccengReadyIntstatusR {
        SecBootEccengReadyIntstatusR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot RSA Engine Ready Interrupt Status"]
    #[inline(always)]
    pub fn sec_boot_rsaeng_ready_intstatus(
        &mut self,
    ) -> SecBootRsaengReadyIntstatusW<Secure0c4Spec> {
        SecBootRsaengReadyIntstatusW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure Boot ECC Engine Ready Interrupt Status"]
    #[inline(always)]
    pub fn sec_boot_ecceng_ready_intstatus(
        &mut self,
    ) -> SecBootEccengReadyIntstatusW<Secure0c4Spec> {
        SecBootEccengReadyIntstatusW::new(self, 1)
    }
}
#[doc = "Secure Boot Engine Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure0c4Spec;
impl crate::RegisterSpec for Secure0c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure0c4::R`](R) reader structure"]
impl crate::Readable for Secure0c4Spec {}
#[doc = "`write(|w| ..)` method takes [`secure0c4::W`](W) writer structure"]
impl crate::Writable for Secure0c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE0C4 to value 0"]
impl crate::Resettable for Secure0c4Spec {}
