#[doc = "Register `SECURE0C0` reader"]
pub type R = crate::R<Secure0c0Spec>;
#[doc = "Register `SECURE0C0` writer"]
pub type W = crate::W<Secure0c0Spec>;
#[doc = "Field `SecBootRSAEngReadyINTEnanle` reader - Secure Boot RSA Engine Ready Interrupt Enanle"]
pub type SecBootRsaengReadyIntenanleR = crate::BitReader;
#[doc = "Field `SecBootRSAEngReadyINTEnanle` writer - Secure Boot RSA Engine Ready Interrupt Enanle"]
pub type SecBootRsaengReadyIntenanleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SecBootECCEngReadyINTEnanle` reader - Secure Boot ECC Engine Ready Interrupt Enanle"]
pub type SecBootEccengReadyIntenanleR = crate::BitReader;
#[doc = "Field `SecBootECCEngReadyINTEnanle` writer - Secure Boot ECC Engine Ready Interrupt Enanle"]
pub type SecBootEccengReadyIntenanleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Secure Boot RSA Engine Ready Interrupt Enanle"]
    #[inline(always)]
    pub fn sec_boot_rsaeng_ready_intenanle(&self) -> SecBootRsaengReadyIntenanleR {
        SecBootRsaengReadyIntenanleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure Boot ECC Engine Ready Interrupt Enanle"]
    #[inline(always)]
    pub fn sec_boot_ecceng_ready_intenanle(&self) -> SecBootEccengReadyIntenanleR {
        SecBootEccengReadyIntenanleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot RSA Engine Ready Interrupt Enanle"]
    #[inline(always)]
    pub fn sec_boot_rsaeng_ready_intenanle(
        &mut self,
    ) -> SecBootRsaengReadyIntenanleW<Secure0c0Spec> {
        SecBootRsaengReadyIntenanleW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure Boot ECC Engine Ready Interrupt Enanle"]
    #[inline(always)]
    pub fn sec_boot_ecceng_ready_intenanle(
        &mut self,
    ) -> SecBootEccengReadyIntenanleW<Secure0c0Spec> {
        SecBootEccengReadyIntenanleW::new(self, 1)
    }
}
#[doc = "Secure Boot Engine Interrupt Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure0c0Spec;
impl crate::RegisterSpec for Secure0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure0c0::R`](R) reader structure"]
impl crate::Readable for Secure0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`secure0c0::W`](W) writer structure"]
impl crate::Writable for Secure0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE0C0 to value 0"]
impl crate::Resettable for Secure0c0Spec {}
