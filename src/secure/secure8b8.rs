#[doc = "Register `SECURE8B8` reader"]
pub type R = crate::R<Secure8b8Spec>;
#[doc = "Register `SECURE8B8` writer"]
pub type W = crate::W<Secure8b8Spec>;
#[doc = "Field `SecBootCryptoKeyBuffer6Reg` reader - Secure Boot Crypto Key Buffer 6 Register"]
pub type SecBootCryptoKeyBuffer6regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoKeyBuffer6Reg` writer - Secure Boot Crypto Key Buffer 6 Register"]
pub type SecBootCryptoKeyBuffer6regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 6 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer6reg(&self) -> SecBootCryptoKeyBuffer6regR {
        SecBootCryptoKeyBuffer6regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 6 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer6reg(&mut self) -> SecBootCryptoKeyBuffer6regW<Secure8b8Spec> {
        SecBootCryptoKeyBuffer6regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Key Buffer 6 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure8b8Spec;
impl crate::RegisterSpec for Secure8b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure8b8::R`](R) reader structure"]
impl crate::Readable for Secure8b8Spec {}
#[doc = "`write(|w| ..)` method takes [`secure8b8::W`](W) writer structure"]
impl crate::Writable for Secure8b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE8B8 to value 0"]
impl crate::Resettable for Secure8b8Spec {}
