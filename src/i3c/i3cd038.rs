#[doc = "Register `I3CD038` reader"]
pub type R = crate::R<I3cd038Spec>;
#[doc = "Register `I3CD038` writer"]
pub type W = crate::W<I3cd038Spec>;
#[doc = "Field `SlaveINTReq` reader - Slave Interrupt Request."]
pub type SlaveIntreqR = crate::BitReader;
#[doc = "Field `MasterINTReq` reader - Master Interrupt Request."]
pub type MasterIntreqR = crate::BitReader;
#[doc = "Field `RSVD2` reader - These bits in Slave Event Control Register are reserved."]
pub type Rsvd2R = crate::BitReader;
#[doc = "Field `HotJoinINTReq` reader - Hot-Join Interrupt Request."]
pub type HotJoinIntreqR = crate::BitReader;
#[doc = "Field `ActivityStateStatus` reader - Activity State Status."]
pub type ActivityStateStatusR = crate::FieldReader;
#[doc = "Field `MRLUpdatedStatus` reader - MRL Updated Status."]
pub type MrlupdatedStatusR = crate::BitReader;
#[doc = "Field `MRLUpdatedStatus` writer - MRL Updated Status."]
pub type MrlupdatedStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MWLUpdatedStatus` reader - MWL Updated Status."]
pub type MwlupdatedStatusR = crate::BitReader;
#[doc = "Field `MWLUpdatedStatus` writer - MWL Updated Status."]
pub type MwlupdatedStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD316` reader - These bits in Slave Event Control Register are reserved."]
pub type Rsvd316R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Slave Interrupt Request."]
    #[inline(always)]
    pub fn slave_intreq(&self) -> SlaveIntreqR {
        SlaveIntreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Interrupt Request."]
    #[inline(always)]
    pub fn master_intreq(&self) -> MasterIntreqR {
        MasterIntreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - These bits in Slave Event Control Register are reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Hot-Join Interrupt Request."]
    #[inline(always)]
    pub fn hot_join_intreq(&self) -> HotJoinIntreqR {
        HotJoinIntreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Activity State Status."]
    #[inline(always)]
    pub fn activity_state_status(&self) -> ActivityStateStatusR {
        ActivityStateStatusR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - MRL Updated Status."]
    #[inline(always)]
    pub fn mrlupdated_status(&self) -> MrlupdatedStatusR {
        MrlupdatedStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MWL Updated Status."]
    #[inline(always)]
    pub fn mwlupdated_status(&self) -> MwlupdatedStatusR {
        MwlupdatedStatusR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - These bits in Slave Event Control Register are reserved."]
    #[inline(always)]
    pub fn rsvd316(&self) -> Rsvd316R {
        Rsvd316R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 6 - MRL Updated Status."]
    #[inline(always)]
    pub fn mrlupdated_status(&mut self) -> MrlupdatedStatusW<I3cd038Spec> {
        MrlupdatedStatusW::new(self, 6)
    }
    #[doc = "Bit 7 - MWL Updated Status."]
    #[inline(always)]
    pub fn mwlupdated_status(&mut self) -> MwlupdatedStatusW<I3cd038Spec> {
        MwlupdatedStatusW::new(self, 7)
    }
}
#[doc = "Slave Event Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd038Spec;
impl crate::RegisterSpec for I3cd038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd038::R`](R) reader structure"]
impl crate::Readable for I3cd038Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd038::W`](W) writer structure"]
impl crate::Writable for I3cd038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD038 to value 0"]
impl crate::Resettable for I3cd038Spec {}
