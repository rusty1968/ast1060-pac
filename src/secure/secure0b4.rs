#[doc = "Register `SECURE0B4` reader"]
pub type R = crate::R<Secure0b4Spec>;
#[doc = "Register `SECURE0B4` writer"]
pub type W = crate::W<Secure0b4Spec>;
#[doc = "Field `SecBootECCEngEnbl` reader - Secure Boot ECC Engine Enable"]
pub type SecBootEccengEnblR = crate::BitReader;
#[doc = "Field `SecBootECCEngEnbl` writer - Secure Boot ECC Engine Enable"]
pub type SecBootEccengEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SecBootECDSA384EngEnbl` reader - Secure Boot ECDSA384 Engine Enable"]
pub type SecBootEcdsa384engEnblR = crate::BitReader;
#[doc = "Field `SecBootECDSA384EngEnbl` writer - Secure Boot ECDSA384 Engine Enable"]
pub type SecBootEcdsa384engEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Secure Boot ECC Engine Enable"]
    #[inline(always)]
    pub fn sec_boot_ecceng_enbl(&self) -> SecBootEccengEnblR {
        SecBootEccengEnblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Secure Boot ECDSA384 Engine Enable"]
    #[inline(always)]
    pub fn sec_boot_ecdsa384eng_enbl(&self) -> SecBootEcdsa384engEnblR {
        SecBootEcdsa384engEnblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot ECC Engine Enable"]
    #[inline(always)]
    pub fn sec_boot_ecceng_enbl(&mut self) -> SecBootEccengEnblW<Secure0b4Spec> {
        SecBootEccengEnblW::new(self, 0)
    }
    #[doc = "Bit 1 - Secure Boot ECDSA384 Engine Enable"]
    #[inline(always)]
    pub fn sec_boot_ecdsa384eng_enbl(&mut self) -> SecBootEcdsa384engEnblW<Secure0b4Spec> {
        SecBootEcdsa384engEnblW::new(self, 1)
    }
}
#[doc = "Secure Boot ECC Engine Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure0b4Spec;
impl crate::RegisterSpec for Secure0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure0b4::R`](R) reader structure"]
impl crate::Readable for Secure0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`secure0b4::W`](W) writer structure"]
impl crate::Writable for Secure0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE0B4 to value 0"]
impl crate::Resettable for Secure0b4Spec {}
