#[doc = "Register `SECURE800` reader"]
pub type R = crate::R<Secure800Spec>;
#[doc = "Register `SECURE800` writer"]
pub type W = crate::W<Secure800Spec>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Secure Boot DMA Enable Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecBootDmaenblReg {
    #[doc = "1: trigger Secure Boot DMA"]
    TriggerSecureBootDma = 1,
}
impl From<SecBootDmaenblReg> for bool {
    #[inline(always)]
    fn from(variant: SecBootDmaenblReg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecBootDMAEnblReg` reader - Secure Boot DMA Enable Register"]
pub type SecBootDmaenblRegR = crate::BitReader<SecBootDmaenblReg>;
impl SecBootDmaenblRegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecBootDmaenblReg> {
        match self.bits {
            true => Some(SecBootDmaenblReg::TriggerSecureBootDma),
            _ => None,
        }
    }
    #[doc = "trigger Secure Boot DMA"]
    #[inline(always)]
    pub fn is_trigger_secure_boot_dma(&self) -> bool {
        *self == SecBootDmaenblReg::TriggerSecureBootDma
    }
}
#[doc = "Field `SecBootDMAEnblReg` writer - Secure Boot DMA Enable Register"]
pub type SecBootDmaenblRegW<'a, REG> = crate::BitWriter<'a, REG, SecBootDmaenblReg>;
impl<'a, REG> SecBootDmaenblRegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "trigger Secure Boot DMA"]
    #[inline(always)]
    pub fn trigger_secure_boot_dma(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootDmaenblReg::TriggerSecureBootDma)
    }
}
impl R {
    #[doc = "Bits 0:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
    #[doc = "Bit 0 - Secure Boot DMA Enable Register"]
    #[inline(always)]
    pub fn sec_boot_dmaenbl_reg(&self) -> SecBootDmaenblRegR {
        SecBootDmaenblRegR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot DMA Enable Register"]
    #[inline(always)]
    pub fn sec_boot_dmaenbl_reg(&mut self) -> SecBootDmaenblRegW<Secure800Spec> {
        SecBootDmaenblRegW::new(self, 0)
    }
}
#[doc = "Secure Boot DMA Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure800::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure800::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure800Spec;
impl crate::RegisterSpec for Secure800Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure800::R`](R) reader structure"]
impl crate::Readable for Secure800Spec {}
#[doc = "`write(|w| ..)` method takes [`secure800::W`](W) writer structure"]
impl crate::Writable for Secure800Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE800 to value 0"]
impl crate::Resettable for Secure800Spec {}
