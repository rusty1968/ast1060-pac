#[doc = "Register `SECURE88C` reader"]
pub type R = crate::R<Secure88cSpec>;
#[doc = "Register `SECURE88C` writer"]
pub type W = crate::W<Secure88cSpec>;
#[doc = "Field `SecBootCryptoDataBuffer3Reg` reader - Secure Boot Crypto Data Buffer 3 Register"]
pub type SecBootCryptoDataBuffer3regR = crate::FieldReader<u32>;
#[doc = "Field `SecBootCryptoDataBuffer3Reg` writer - Secure Boot Crypto Data Buffer 3 Register"]
pub type SecBootCryptoDataBuffer3regW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Crypto Data Buffer 3 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_buffer3reg(&self) -> SecBootCryptoDataBuffer3regR {
        SecBootCryptoDataBuffer3regR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Crypto Data Buffer 3 Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_buffer3reg(
        &mut self,
    ) -> SecBootCryptoDataBuffer3regW<Secure88cSpec> {
        SecBootCryptoDataBuffer3regW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Data Buffer 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure88c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure88c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure88cSpec;
impl crate::RegisterSpec for Secure88cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure88c::R`](R) reader structure"]
impl crate::Readable for Secure88cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure88c::W`](W) writer structure"]
impl crate::Writable for Secure88cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE88C to value 0"]
impl crate::Resettable for Secure88cSpec {}
