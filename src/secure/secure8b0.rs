#[doc = "Register `SECURE8B0` reader"]
pub type R = crate::R<Secure8b0Spec>;
#[doc = "Register `SECURE8B0` writer"]
pub type W = crate::W<Secure8b0Spec>;
#[doc = "Field `SecBootCryptoKeyBuffer4Reg` reader - Secure Boot Crypto Key Buffer 4 Register"]
pub type SecBootCryptoKeyBuffer4regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoKeyBuffer4Reg` writer - Secure Boot Crypto Key Buffer 4 Register"]
pub type SecBootCryptoKeyBuffer4regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 4 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer4reg(&self) -> SecBootCryptoKeyBuffer4regR {
        SecBootCryptoKeyBuffer4regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 4 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer4reg(&mut self) -> SecBootCryptoKeyBuffer4regW<Secure8b0Spec> {
        SecBootCryptoKeyBuffer4regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Key Buffer 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure8b0Spec;
impl crate::RegisterSpec for Secure8b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure8b0::R`](R) reader structure"]
impl crate::Readable for Secure8b0Spec {}
#[doc = "`write(|w| ..)` method takes [`secure8b0::W`](W) writer structure"]
impl crate::Writable for Secure8b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE8B0 to value 0"]
impl crate::Resettable for Secure8b0Spec {}
