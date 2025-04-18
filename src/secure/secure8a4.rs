#[doc = "Register `SECURE8A4` reader"]
pub type R = crate::R<Secure8a4Spec>;
#[doc = "Register `SECURE8A4` writer"]
pub type W = crate::W<Secure8a4Spec>;
#[doc = "Field `SecBootCryptoKeyBuffer1Reg` reader - Secure Boot Crypto Key Buffer 1 Register"]
pub type SecBootCryptoKeyBuffer1regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoKeyBuffer1Reg` writer - Secure Boot Crypto Key Buffer 1 Register"]
pub type SecBootCryptoKeyBuffer1regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 1 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer1reg(&self) -> SecBootCryptoKeyBuffer1regR {
        SecBootCryptoKeyBuffer1regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 1 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer1reg(&mut self) -> SecBootCryptoKeyBuffer1regW<Secure8a4Spec> {
        SecBootCryptoKeyBuffer1regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Key Buffer 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8a4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8a4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure8a4Spec;
impl crate::RegisterSpec for Secure8a4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure8a4::R`](R) reader structure"]
impl crate::Readable for Secure8a4Spec {}
#[doc = "`write(|w| ..)` method takes [`secure8a4::W`](W) writer structure"]
impl crate::Writable for Secure8a4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE8A4 to value 0"]
impl crate::Resettable for Secure8a4Spec {}
