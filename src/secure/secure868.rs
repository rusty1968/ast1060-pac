#[doc = "Register `SECURE868` reader"]
pub type R = crate::R<Secure868Spec>;
#[doc = "Register `SECURE868` writer"]
pub type W = crate::W<Secure868Spec>;
#[doc = "Field `SecBootCryptoDataTotalSizeReg` reader - Secure Boot Crypto Data Total Size Register"]
pub type SecBootCryptoDataTotalSizeRegR = crate::FieldReader<u16>;
#[doc = "Field `SecBootCryptoDataTotalSizeReg` writer - Secure Boot Crypto Data Total Size Register"]
pub type SecBootCryptoDataTotalSizeRegW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:12 - Secure Boot Crypto Data Total Size Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_total_size_reg(&self) -> SecBootCryptoDataTotalSizeRegR {
        SecBootCryptoDataTotalSizeRegR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:12 - Secure Boot Crypto Data Total Size Register"]
    #[inline(always)]
    pub fn sec_boot_crypto_data_total_size_reg(
        &mut self,
    ) -> SecBootCryptoDataTotalSizeRegW<Secure868Spec> {
        SecBootCryptoDataTotalSizeRegW::new(self, 0)
    }
}
#[doc = "Secure Boot Crypto Data Total Size Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure868::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure868::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure868Spec;
impl crate::RegisterSpec for Secure868Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure868::R`](R) reader structure"]
impl crate::Readable for Secure868Spec {}
#[doc = "`write(|w| ..)` method takes [`secure868::W`](W) writer structure"]
impl crate::Writable for Secure868Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE868 to value 0"]
impl crate::Resettable for Secure868Spec {}
