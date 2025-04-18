#[doc = "Register `SECURE880` reader"]
pub type R = crate::R<Secure880Spec>;
#[doc = "Register `SECURE880` writer"]
pub type W = crate::W<Secure880Spec>;
#[doc = "Field `SecBootCryptoDataBuffer0Reg` reader - Secure Boot Crypto Data Buffer 0 Register"]
pub type SecBootCryptoDataBuffer0regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoDataBuffer0Reg` writer - Secure Boot Crypto Data Buffer 0 Register"]
pub type SecBootCryptoDataBuffer0regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Data Buffer 0 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_buffer0reg(&self) -> SecBootCryptoDataBuffer0regR {
        SecBootCryptoDataBuffer0regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Data Buffer 0 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_buffer0reg(
        &mut self,
    ) -> SecBootCryptoDataBuffer0regW<Secure880Spec> {
        SecBootCryptoDataBuffer0regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Data Buffer 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure880::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure880::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure880Spec;
impl crate::RegisterSpec for Secure880Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure880::R`](R) reader structure"]
impl crate::Readable for Secure880Spec {}
#[doc = "`write(|w| ..)` method takes [`secure880::W`](W) writer structure"]
impl crate::Writable for Secure880Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE880 to value 0"]
impl crate::Resettable for Secure880Spec {}
