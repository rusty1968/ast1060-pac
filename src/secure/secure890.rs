#[doc = "Register `SECURE890` reader"]
pub type R = crate::R<Secure890Spec>;
#[doc = "Register `SECURE890` writer"]
pub type W = crate::W<Secure890Spec>;
#[doc = "Field `SecBootCryptoAESGCMGHashKeyWrTriggerReg` reader - Secure Boot Crypto AES-GCM GHash Key Write Trigger Register"]
pub type SecBootCryptoAesgcmghashKeyWrTriggerRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoAESGCMGHashKeyWrTriggerReg` writer - Secure Boot Crypto AES-GCM GHash Key Write Trigger Register"]
pub type SecBootCryptoAesgcmghashKeyWrTriggerRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto AES-GCM GHash Key Write Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_aesgcmghash_key_wr_trigger_reg(
        &self,
    ) -> SecBootCryptoAesgcmghashKeyWrTriggerRegR {
        SecBootCryptoAesgcmghashKeyWrTriggerRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto AES-GCM GHash Key Write Trigger Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_aesgcmghash_key_wr_trigger_reg(
        &mut self,
    ) -> SecBootCryptoAesgcmghashKeyWrTriggerRegW<Secure890Spec> {
        SecBootCryptoAesgcmghashKeyWrTriggerRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto AES-GCM GHash Key Write Trigger Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure890::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure890::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure890Spec;
impl crate::RegisterSpec for Secure890Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure890::R`](R) reader structure"]
impl crate::Readable for Secure890Spec {}
#[doc = "`write(|w| ..)` method takes [`secure890::W`](W) writer structure"]
impl crate::Writable for Secure890Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE890 to value 0"]
impl crate::Resettable for Secure890Spec {}
