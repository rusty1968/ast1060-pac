#[doc = "Register `I3CD054` reader"]
pub type R = crate::R<I3cd054Spec>;
#[doc = "Register `I3CD054` writer"]
pub type W = crate::W<I3cd054Spec>;
#[doc = "Field `SCLLINESIGNALLEVEL` reader - SCL_LINE_SIGNAL_LEVEL"]
pub type ScllinesignallevelR = crate::BitReader;
#[doc = "Field `SDALINESIGNALLEVEL` reader - SDA_LINE_SIGNAL_LEVEL"]
pub type SdalinesignallevelR = crate::BitReader;
#[doc = "Field `CURRENTMASTER` reader - CURRENT_MASTER"]
pub type CurrentmasterR = crate::BitReader;
#[doc = "Field `RSVD37` reader - These bits in Present State Register are reserved."]
pub type Rsvd37R = crate::FieldReader;
#[doc = "Field `CMTFRSTATUS` reader - CM_TFR_STATUS"]
pub type CmtfrstatusR = crate::FieldReader;
#[doc = "Field `RSVD1415` reader - These bits in Present State Register are reserved."]
pub type Rsvd1415R = crate::FieldReader;
#[doc = "Field `CMTFRSTSTATUS` reader - CM_TFR_ST_STATUS"]
pub type CmtfrststatusR = crate::FieldReader;
#[doc = "Field `RSVD2223` reader - RSVD_22_23"]
pub type Rsvd2223R = crate::FieldReader;
#[doc = "Field `CMDTID` reader - CMD_TID"]
pub type CmdtidR = crate::FieldReader;
#[doc = "Field `MASTERIDLE` reader - MASTER_IDLE"]
pub type MasteridleR = crate::BitReader;
#[doc = "Field `RSVD2831` reader - These bits in Present State Register are reserved."]
pub type Rsvd2831R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - SCL_LINE_SIGNAL_LEVEL"]
    #[inline(always)]
    pub fn scllinesignallevel(&self) -> ScllinesignallevelR {
        ScllinesignallevelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDA_LINE_SIGNAL_LEVEL"]
    #[inline(always)]
    pub fn sdalinesignallevel(&self) -> SdalinesignallevelR {
        SdalinesignallevelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CURRENT_MASTER"]
    #[inline(always)]
    pub fn currentmaster(&self) -> CurrentmasterR {
        CurrentmasterR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - These bits in Present State Register are reserved."]
    #[inline(always)]
    pub fn rsvd37(&self) -> Rsvd37R {
        Rsvd37R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - CM_TFR_STATUS"]
    #[inline(always)]
    pub fn cmtfrstatus(&self) -> CmtfrstatusR {
        CmtfrstatusR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - These bits in Present State Register are reserved."]
    #[inline(always)]
    pub fn rsvd1415(&self) -> Rsvd1415R {
        Rsvd1415R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:21 - CM_TFR_ST_STATUS"]
    #[inline(always)]
    pub fn cmtfrststatus(&self) -> CmtfrststatusR {
        CmtfrststatusR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:23 - RSVD_22_23"]
    #[inline(always)]
    pub fn rsvd2223(&self) -> Rsvd2223R {
        Rsvd2223R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - CMD_TID"]
    #[inline(always)]
    pub fn cmdtid(&self) -> CmdtidR {
        CmdtidR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - MASTER_IDLE"]
    #[inline(always)]
    pub fn masteridle(&self) -> MasteridleR {
        MasteridleR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - These bits in Present State Register are reserved."]
    #[inline(always)]
    pub fn rsvd2831(&self) -> Rsvd2831R {
        Rsvd2831R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {}
#[doc = "PRESENT STATE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd054Spec;
impl crate::RegisterSpec for I3cd054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd054::R`](R) reader structure"]
impl crate::Readable for I3cd054Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd054::W`](W) writer structure"]
impl crate::Writable for I3cd054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD054 to value 0"]
impl crate::Resettable for I3cd054Spec {}
