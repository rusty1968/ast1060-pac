#[doc = "Register `SECURE874` reader"]
pub type R = crate::R<Secure874Spec>;
#[doc = "Register `SECURE874` writer"]
pub type W = crate::W<Secure874Spec>;
#[doc = "Field `SecBootCryptoHighKeyWrTriggerReg` reader - Secure Boot Crypto High Key Write Trigger Register"]
pub type SecBootCryptoHighKeyWrTriggerRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoHighKeyWrTriggerReg` writer - Secure Boot Crypto High Key Write Trigger Register"]
pub type SecBootCryptoHighKeyWrTriggerRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto High Key Write Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_high_key_wr_trigger_reg(&self) -> SecBootCryptoHighKeyWrTriggerRegR {
        SecBootCryptoHighKeyWrTriggerRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto High Key Write Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_high_key_wr_trigger_reg(
        &mut self,
    ) -> SecBootCryptoHighKeyWrTriggerRegW<Secure874Spec> {
        SecBootCryptoHighKeyWrTriggerRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto High Key Write Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure874::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure874::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure874Spec;
impl crate::RegisterSpec for Secure874Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure874::R`](R) reader structure"]
impl crate::Readable for Secure874Spec {}
#[doc = "`write(|w| ..)` method takes [`secure874::W`](W) writer structure"]
impl crate::Writable for Secure874Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE874 to value 0"]
impl crate::Resettable for Secure874Spec {}
