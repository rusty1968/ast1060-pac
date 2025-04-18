#[doc = "Register `I3CD0D0` reader"]
pub type R = crate::R<I3cd0d0Spec>;
#[doc = "Register `I3CD0D0` writer"]
pub type W = crate::W<I3cd0d0Spec>;
#[doc = "Field `SDAODPPSWITCHDLY` reader - SDA_OD_PP_SWITCH_DLY"]
pub type SdaodppswitchdlyR = crate::FieldReader;
#[doc = "Field `SDAODPPSWITCHDLY` writer - SDA_OD_PP_SWITCH_DLY"]
pub type SdaodppswitchdlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD2` reader - reserved"]
pub type Rsvd2R = crate::FieldReader;
#[doc = "Field `RSVD2` writer - reserved"]
pub type Rsvd2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SDAPPODSWITCHDLY` reader - SDA_PP_OD_SWITCH_DLY"]
pub type SdappodswitchdlyR = crate::FieldReader;
#[doc = "Field `SDAPPODSWITCHDLY` writer - SDA_PP_OD_SWITCH_DLY"]
pub type SdappodswitchdlyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::FieldReader;
#[doc = "Field `RSVD1` writer - reserved"]
pub type Rsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SDATXHOLD` reader - SDA_TX_HOLD"]
pub type SdatxholdR = crate::FieldReader;
#[doc = "Field `SDATXHOLD` writer - SDA_TX_HOLD"]
pub type SdatxholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `RSVD` writer - reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:2 - SDA_OD_PP_SWITCH_DLY"]
    #[inline(always)]
    pub fn sdaodppswitchdly(&self) -> SdaodppswitchdlyR {
        SdaodppswitchdlyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - SDA_PP_OD_SWITCH_DLY"]
    #[inline(always)]
    pub fn sdappodswitchdly(&self) -> SdappodswitchdlyR {
        SdappodswitchdlyR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - SDA_TX_HOLD"]
    #[inline(always)]
    pub fn sdatxhold(&self) -> SdatxholdR {
        SdatxholdR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDA_OD_PP_SWITCH_DLY"]
    #[inline(always)]
    pub fn sdaodppswitchdly(&mut self) -> SdaodppswitchdlyW<I3cd0d0Spec> {
        SdaodppswitchdlyW::new(self, 0)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn rsvd2(&mut self) -> Rsvd2W<I3cd0d0Spec> {
        Rsvd2W::new(self, 3)
    }
    #[doc = "Bits 8:10 - SDA_PP_OD_SWITCH_DLY"]
    #[inline(always)]
    pub fn sdappodswitchdly(&mut self) -> SdappodswitchdlyW<I3cd0d0Spec> {
        SdappodswitchdlyW::new(self, 8)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&mut self) -> Rsvd1W<I3cd0d0Spec> {
        Rsvd1W::new(self, 11)
    }
    #[doc = "Bits 16:18 - SDA_TX_HOLD"]
    #[inline(always)]
    pub fn sdatxhold(&mut self) -> SdatxholdW<I3cd0d0Spec> {
        SdatxholdW::new(self, 16)
    }
    #[doc = "Bits 19:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<I3cd0d0Spec> {
        RsvdW::new(self, 19)
    }
}
#[doc = "SDA Hold and Mode Switch Delay Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0d0Spec;
impl crate::RegisterSpec for I3cd0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0d0::R`](R) reader structure"]
impl crate::Readable for I3cd0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0d0::W`](W) writer structure"]
impl crate::Writable for I3cd0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0D0 to value 0"]
impl crate::Resettable for I3cd0d0Spec {}
