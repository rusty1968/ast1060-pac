#[doc = "Register `I3CD050` reader"]
pub type R = crate::R<I3cd050Spec>;
#[doc = "Register `I3CD050` writer"]
pub type W = crate::W<I3cd050Spec>;
#[doc = "Field `TXBUFEMPTYLOC` reader - TX_BUF_EMPTY_LOC"]
pub type TxbufemptylocR = crate::FieldReader;
#[doc = "Field `RSVD158` reader - These bits in Data Buffer Level Register are reserved."]
pub type Rsvd158R = crate::FieldReader;
#[doc = "Field `RXBUFBLR` reader - RX_BUF_BLR"]
pub type RxbufblrR = crate::FieldReader;
#[doc = "Field `RSVD3124` reader - These bits in Data Buffer Level Register are reserved."]
pub type Rsvd3124R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - TX_BUF_EMPTY_LOC"]
    #[inline(always)]
    pub fn txbufemptyloc(&self) -> TxbufemptylocR {
        TxbufemptylocR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - These bits in Data Buffer Level Register are reserved."]
    #[inline(always)]
    pub fn rsvd158(&self) -> Rsvd158R {
        Rsvd158R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RX_BUF_BLR"]
    #[inline(always)]
    pub fn rxbufblr(&self) -> RxbufblrR {
        RxbufblrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - These bits in Data Buffer Level Register are reserved."]
    #[inline(always)]
    pub fn rsvd3124(&self) -> Rsvd3124R {
        Rsvd3124R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "DATA BUFFER STATUS LEVEL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd050Spec;
impl crate::RegisterSpec for I3cd050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd050::R`](R) reader structure"]
impl crate::Readable for I3cd050Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd050::W`](W) writer structure"]
impl crate::Writable for I3cd050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD050 to value 0"]
impl crate::Resettable for I3cd050Spec {}
