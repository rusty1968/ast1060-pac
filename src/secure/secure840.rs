#[doc = "Register `SECURE840` reader"]
pub type R = crate::R<Secure840Spec>;
#[doc = "Register `SECURE840` writer"]
pub type W = crate::W<Secure840Spec>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `SecBootDMASourceAddrReg` reader - Secure Boot DMA Source Address Register"]
pub type SecBootDmasourceAddrRegR = crate::FieldReader<u32>;
#[doc = "Field `SecBootDMASourceAddrReg` writer - Secure Boot DMA Source Address Register"]
pub type SecBootDmasourceAddrRegW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Secure Boot DMA Source Address Register"]
    #[inline(always)]
    pub fn sec_boot_dmasource_addr_reg(&self) -> SecBootDmasourceAddrRegR {
        SecBootDmasourceAddrRegR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Secure Boot DMA Source Address Register"]
    #[inline(always)]
    pub fn sec_boot_dmasource_addr_reg(&mut self) -> SecBootDmasourceAddrRegW<Secure840Spec> {
        SecBootDmasourceAddrRegW::new(self, 2)
    }
}
#[doc = "Secure Boot DMA Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure840::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure840::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure840Spec;
impl crate::RegisterSpec for Secure840Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure840::R`](R) reader structure"]
impl crate::Readable for Secure840Spec {}
#[doc = "`write(|w| ..)` method takes [`secure840::W`](W) writer structure"]
impl crate::Writable for Secure840Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE840 to value 0"]
impl crate::Resettable for Secure840Spec {}
