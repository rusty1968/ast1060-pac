#[doc = "Register `SECURE884` reader"]
pub type R = crate::R<Secure884Spec>;
#[doc = "Register `SECURE884` writer"]
pub type W = crate::W<Secure884Spec>;
#[doc = "Field `SecBootCryptoDataBuffer1Reg` reader - Secure Boot Crypto Data Buffer 1 Register"]
pub type SecBootCryptoDataBuffer1regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoDataBuffer1Reg` writer - Secure Boot Crypto Data Buffer 1 Register"]
pub type SecBootCryptoDataBuffer1regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Data Buffer 1 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_buffer1reg(&self) -> SecBootCryptoDataBuffer1regR {
        SecBootCryptoDataBuffer1regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Data Buffer 1 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_buffer1reg(
        &mut self,
    ) -> SecBootCryptoDataBuffer1regW<Secure884Spec> {
        SecBootCryptoDataBuffer1regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Data Buffer 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure884::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure884::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure884Spec;
impl crate::RegisterSpec for Secure884Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure884::R`](R) reader structure"]
impl crate::Readable for Secure884Spec {}
#[doc = "`write(|w| ..)` method takes [`secure884::W`](W) writer structure"]
impl crate::Writable for Secure884Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE884 to value 0"]
impl crate::Resettable for Secure884Spec {}
