#[doc = "Register `SECURE8BC` reader"]
pub type R = crate::R<Secure8bcSpec>;
#[doc = "Register `SECURE8BC` writer"]
pub type W = crate::W<Secure8bcSpec>;
#[doc = "Field `SecBootCryptoKeyBuffer7Reg` reader - Secure Boot Crypto Key Buffer 7 Register"]
pub type SecBootCryptoKeyBuffer7regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoKeyBuffer7Reg` writer - Secure Boot Crypto Key Buffer 7 Register"]
pub type SecBootCryptoKeyBuffer7regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 7 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer7reg(&self) -> SecBootCryptoKeyBuffer7regR {
        SecBootCryptoKeyBuffer7regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Key Buffer 7 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_key_buffer7reg(&mut self) -> SecBootCryptoKeyBuffer7regW<Secure8bcSpec> {
        SecBootCryptoKeyBuffer7regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Key Buffer 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure8bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure8bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure8bcSpec;
impl crate::RegisterSpec for Secure8bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure8bc::R`](R) reader structure"]
impl crate::Readable for Secure8bcSpec {}
#[doc = "`write(|w| ..)` method takes [`secure8bc::W`](W) writer structure"]
impl crate::Writable for Secure8bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE8BC to value 0"]
impl crate::Resettable for Secure8bcSpec {}
