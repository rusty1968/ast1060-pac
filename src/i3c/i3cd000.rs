#[doc = "Register `I3CD000` reader"]
pub type R = crate::R<I3cd000Spec>;
#[doc = "Register `I3CD000` writer"]
pub type W = crate::W<I3cd000Spec>;
#[doc = "Field `I3CBroadcastAddrInclude` reader - I3C Broadcast Address include."]
pub type I3cbroadcastAddrIncludeR = crate::BitReader;
#[doc = "Field `I3CBroadcastAddrInclude` writer - I3C Broadcast Address include."]
pub type I3cbroadcastAddrIncludeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD` writer - reserved"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `I2CSlavePresent` reader - I2C Slave Present"]
pub type I2cslavePresentR = crate::BitReader;
#[doc = "Field `I2CSlavePresent` writer - I2C Slave Present"]
pub type I2cslavePresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HotJoinAckNackCtrl` reader - Hot-Join Ack/Nack Control"]
pub type HotJoinAckNackCtrlR = crate::BitReader;
#[doc = "Field `HotJoinAckNackCtrl` writer - Hot-Join Ack/Nack Control"]
pub type HotJoinAckNackCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIPAYLOADEN` reader - Enable IBI with Data in Slave Mode"]
pub type IbipayloadenR = crate::BitReader;
#[doc = "Field `IBIPAYLOADEN` writer - Enable IBI with Data in Slave Mode"]
pub type IbipayloadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD1115` reader - These bits in Device Control Register is reserved. It will always return 0. These bits are reserved in slave mode of operation"]
pub type Rsvd1115R = crate::FieldReader;
#[doc = "Field `MDB` reader - Mandatory Byte in Slave Mode"]
pub type MdbR = crate::FieldReader;
#[doc = "Field `MDB` writer - Mandatory Byte in Slave Mode"]
pub type MdbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IdleCountMultiplier` reader - Idle count multiplier."]
pub type IdleCountMultiplierR = crate::FieldReader;
#[doc = "Field `IdleCountMultiplier` writer - Idle count multiplier."]
pub type IdleCountMultiplierW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD26` reader - These bits in Device Control Register is reserved. It will always return 0."]
pub type Rsvd26R = crate::BitReader;
#[doc = "Field `EnblAdaptionOfI2CI3CMode` reader - Control bit to enable the controller to adapt to I2C/I3C mode of operation."]
pub type EnblAdaptionOfI2ci3cmodeR = crate::BitReader;
#[doc = "Field `EnblAdaptionOfI2CI3CMode` writer - Control bit to enable the controller to adapt to I2C/I3C mode of operation."]
pub type EnblAdaptionOfI2ci3cmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAHandshakeInterfaceEnbl` reader - DMA Handshake Interface Enable."]
pub type DmahandshakeInterfaceEnblR = crate::BitReader;
#[doc = "Field `DMAHandshakeInterfaceEnbl` writer - DMA Handshake Interface Enable."]
pub type DmahandshakeInterfaceEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3CAbort` reader - I3C Abort."]
pub type I3cabortR = crate::BitReader;
#[doc = "Field `I3CAbort` writer - I3C Abort."]
pub type I3cabortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3CResume` reader - I3C Resume"]
pub type I3cresumeR = crate::BitReader;
#[doc = "Field `I3CResume` writer - I3C Resume"]
pub type I3cresumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblI3CCtrl` reader - Controls whether I3C is enabled in master mode of operation"]
pub type EnblI3cctrlR = crate::BitReader;
#[doc = "Field `EnblI3CCtrl` writer - Controls whether I3C is enabled in master mode of operation"]
pub type EnblI3cctrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I3C Broadcast Address include."]
    #[inline(always)]
    pub fn i3cbroadcast_addr_include(&self) -> I3cbroadcastAddrIncludeR {
        I3cbroadcastAddrIncludeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - I2C Slave Present"]
    #[inline(always)]
    pub fn i2cslave_present(&self) -> I2cslavePresentR {
        I2cslavePresentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Hot-Join Ack/Nack Control"]
    #[inline(always)]
    pub fn hot_join_ack_nack_ctrl(&self) -> HotJoinAckNackCtrlR {
        HotJoinAckNackCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable IBI with Data in Slave Mode"]
    #[inline(always)]
    pub fn ibipayloaden(&self) -> IbipayloadenR {
        IbipayloadenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - These bits in Device Control Register is reserved. It will always return 0. These bits are reserved in slave mode of operation"]
    #[inline(always)]
    pub fn rsvd1115(&self) -> Rsvd1115R {
        Rsvd1115R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Mandatory Byte in Slave Mode"]
    #[inline(always)]
    pub fn mdb(&self) -> MdbR {
        MdbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Idle count multiplier."]
    #[inline(always)]
    pub fn idle_count_multiplier(&self) -> IdleCountMultiplierR {
        IdleCountMultiplierR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - These bits in Device Control Register is reserved. It will always return 0."]
    #[inline(always)]
    pub fn rsvd26(&self) -> Rsvd26R {
        Rsvd26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Control bit to enable the controller to adapt to I2C/I3C mode of operation."]
    #[inline(always)]
    pub fn enbl_adaption_of_i2ci3cmode(&self) -> EnblAdaptionOfI2ci3cmodeR {
        EnblAdaptionOfI2ci3cmodeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Handshake Interface Enable."]
    #[inline(always)]
    pub fn dmahandshake_interface_enbl(&self) -> DmahandshakeInterfaceEnblR {
        DmahandshakeInterfaceEnblR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - I3C Abort."]
    #[inline(always)]
    pub fn i3cabort(&self) -> I3cabortR {
        I3cabortR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - I3C Resume"]
    #[inline(always)]
    pub fn i3cresume(&self) -> I3cresumeR {
        I3cresumeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Controls whether I3C is enabled in master mode of operation"]
    #[inline(always)]
    pub fn enbl_i3cctrl(&self) -> EnblI3cctrlR {
        EnblI3cctrlR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I3C Broadcast Address include."]
    #[inline(always)]
    pub fn i3cbroadcast_addr_include(&mut self) -> I3cbroadcastAddrIncludeW<I3cd000Spec> {
        I3cbroadcastAddrIncludeW::new(self, 0)
    }
    #[doc = "Bits 1:6 - reserved"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<I3cd000Spec> {
        RsvdW::new(self, 1)
    }
    #[doc = "Bit 7 - I2C Slave Present"]
    #[inline(always)]
    pub fn i2cslave_present(&mut self) -> I2cslavePresentW<I3cd000Spec> {
        I2cslavePresentW::new(self, 7)
    }
    #[doc = "Bit 8 - Hot-Join Ack/Nack Control"]
    #[inline(always)]
    pub fn hot_join_ack_nack_ctrl(&mut self) -> HotJoinAckNackCtrlW<I3cd000Spec> {
        HotJoinAckNackCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable IBI with Data in Slave Mode"]
    #[inline(always)]
    pub fn ibipayloaden(&mut self) -> IbipayloadenW<I3cd000Spec> {
        IbipayloadenW::new(self, 9)
    }
    #[doc = "Bits 16:23 - Mandatory Byte in Slave Mode"]
    #[inline(always)]
    pub fn mdb(&mut self) -> MdbW<I3cd000Spec> {
        MdbW::new(self, 16)
    }
    #[doc = "Bits 24:25 - Idle count multiplier."]
    #[inline(always)]
    pub fn idle_count_multiplier(&mut self) -> IdleCountMultiplierW<I3cd000Spec> {
        IdleCountMultiplierW::new(self, 24)
    }
    #[doc = "Bit 27 - Control bit to enable the controller to adapt to I2C/I3C mode of operation."]
    #[inline(always)]
    pub fn enbl_adaption_of_i2ci3cmode(&mut self) -> EnblAdaptionOfI2ci3cmodeW<I3cd000Spec> {
        EnblAdaptionOfI2ci3cmodeW::new(self, 27)
    }
    #[doc = "Bit 28 - DMA Handshake Interface Enable."]
    #[inline(always)]
    pub fn dmahandshake_interface_enbl(&mut self) -> DmahandshakeInterfaceEnblW<I3cd000Spec> {
        DmahandshakeInterfaceEnblW::new(self, 28)
    }
    #[doc = "Bit 29 - I3C Abort."]
    #[inline(always)]
    pub fn i3cabort(&mut self) -> I3cabortW<I3cd000Spec> {
        I3cabortW::new(self, 29)
    }
    #[doc = "Bit 30 - I3C Resume"]
    #[inline(always)]
    pub fn i3cresume(&mut self) -> I3cresumeW<I3cd000Spec> {
        I3cresumeW::new(self, 30)
    }
    #[doc = "Bit 31 - Controls whether I3C is enabled in master mode of operation"]
    #[inline(always)]
    pub fn enbl_i3cctrl(&mut self) -> EnblI3cctrlW<I3cd000Spec> {
        EnblI3cctrlW::new(self, 31)
    }
}
#[doc = "Devicee Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd000Spec;
impl crate::RegisterSpec for I3cd000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd000::R`](R) reader structure"]
impl crate::Readable for I3cd000Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd000::W`](W) writer structure"]
impl crate::Writable for I3cd000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD000 to value 0"]
impl crate::Resettable for I3cd000Spec {}
