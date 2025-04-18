#[doc = "Register `I3CD058` reader"]
pub type R = crate::R<I3cd058Spec>;
#[doc = "Register `I3CD058` writer"]
pub type W = crate::W<I3cd058Spec>;
#[doc = "Field `PENDINGINTR` reader - PENDING_INTR"]
pub type PendingintrR = crate::FieldReader;
#[doc = "Field `RSVD4` reader - This bit in Device Operating Status register is reserved."]
pub type Rsvd4R = crate::BitReader;
#[doc = "Field `ACTIVITYMODE` reader - ACTIVITY_MODE"]
pub type ActivitymodeR = crate::FieldReader;
#[doc = "Field `PROTOCOLERR` reader - PROTOCOL_ERR"]
pub type ProtocolerrR = crate::FieldReader;
#[doc = "Field `SLAVEBUSY` reader - SLAVE_BUSY"]
pub type SlavebusyR = crate::BitReader;
#[doc = "Field `OVERFLOWERR` reader - OVERFLOW_ERR"]
pub type OverflowerrR = crate::BitReader;
#[doc = "Field `UNDERERR` reader - UNDER_ERR"]
pub type UndererrR = crate::BitReader;
#[doc = "Field `DATANOTREADY` reader - DATA_NOT_READY"]
pub type DatanotreadyR = crate::BitReader;
#[doc = "Field `BUFFERNOTAVAIL` reader - BUFFER_NOT_AVAIL"]
pub type BuffernotavailR = crate::BitReader;
#[doc = "Field `FRAMEERROR` reader - FRAME_ERROR"]
pub type FrameerrorR = crate::BitReader;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - PENDING_INTR"]
    #[inline(always)]
    pub fn pendingintr(&self) -> PendingintrR {
        PendingintrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - This bit in Device Operating Status register is reserved."]
    #[inline(always)]
    pub fn rsvd4(&self) -> Rsvd4R {
        Rsvd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 6:7 - ACTIVITY_MODE"]
    #[inline(always)]
    pub fn activitymode(&self) -> ActivitymodeR {
        ActivitymodeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PROTOCOL_ERR"]
    #[inline(always)]
    pub fn protocolerr(&self) -> ProtocolerrR {
        ProtocolerrR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 9 - SLAVE_BUSY"]
    #[inline(always)]
    pub fn slavebusy(&self) -> SlavebusyR {
        SlavebusyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OVERFLOW_ERR"]
    #[inline(always)]
    pub fn overflowerr(&self) -> OverflowerrR {
        OverflowerrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 10 - UNDER_ERR"]
    #[inline(always)]
    pub fn undererr(&self) -> UndererrR {
        UndererrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DATA_NOT_READY"]
    #[inline(always)]
    pub fn datanotready(&self) -> DatanotreadyR {
        DatanotreadyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BUFFER_NOT_AVAIL"]
    #[inline(always)]
    pub fn buffernotavail(&self) -> BuffernotavailR {
        BuffernotavailR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FRAME_ERROR"]
    #[inline(always)]
    pub fn frameerror(&self) -> FrameerrorR {
        FrameerrorR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {}
#[doc = "Device Operating Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd058Spec;
impl crate::RegisterSpec for I3cd058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd058::R`](R) reader structure"]
impl crate::Readable for I3cd058Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd058::W`](W) writer structure"]
impl crate::Writable for I3cd058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD058 to value 0"]
impl crate::Resettable for I3cd058Spec {}
