#[doc = "Register `I3CD01C` reader"]
pub type R = crate::R<I3cd01cSpec>;
#[doc = "Register `I3CD01C` writer"]
pub type W = crate::W<I3cd01cSpec>;
#[doc = "Field `CmdBufferEmptyThresholdValue` reader - Command Buffer Empty Threshold Value."]
pub type CmdBufferEmptyThresholdValueR = crate::FieldReader;
#[doc = "Field `CmdBufferEmptyThresholdValue` writer - Command Buffer Empty Threshold Value."]
pub type CmdBufferEmptyThresholdValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ResponseBufferThresholdValue` reader - Response Buffer Threshold Value."]
pub type ResponseBufferThresholdValueR = crate::FieldReader;
#[doc = "Field `ResponseBufferThresholdValue` writer - Response Buffer Threshold Value."]
pub type ResponseBufferThresholdValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IBIDataThresholdValue` reader - IBI Data Threshold Value"]
pub type IbidataThresholdValueR = crate::FieldReader;
#[doc = "Field `IBIDataThresholdValue` writer - IBI Data Threshold Value"]
pub type IbidataThresholdValueW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `InBandINTStatusThresholdValue` reader - In-Band Interrupt Status Threshold Value."]
pub type InBandIntstatusThresholdValueR = crate::FieldReader;
#[doc = "Field `InBandINTStatusThresholdValue` writer - In-Band Interrupt Status Threshold Value."]
pub type InBandIntstatusThresholdValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Command Buffer Empty Threshold Value."]
    #[inline(always)]
    pub fn cmd_buffer_empty_threshold_value(&self) -> CmdBufferEmptyThresholdValueR {
        CmdBufferEmptyThresholdValueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Response Buffer Threshold Value."]
    #[inline(always)]
    pub fn response_buffer_threshold_value(&self) -> ResponseBufferThresholdValueR {
        ResponseBufferThresholdValueR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - IBI Data Threshold Value"]
    #[inline(always)]
    pub fn ibidata_threshold_value(&self) -> IbidataThresholdValueR {
        IbidataThresholdValueR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - In-Band Interrupt Status Threshold Value."]
    #[inline(always)]
    pub fn in_band_intstatus_threshold_value(&self) -> InBandIntstatusThresholdValueR {
        InBandIntstatusThresholdValueR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command Buffer Empty Threshold Value."]
    #[inline(always)]
    pub fn cmd_buffer_empty_threshold_value(
        &mut self,
    ) -> CmdBufferEmptyThresholdValueW<I3cd01cSpec> {
        CmdBufferEmptyThresholdValueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Response Buffer Threshold Value."]
    #[inline(always)]
    pub fn response_buffer_threshold_value(
        &mut self,
    ) -> ResponseBufferThresholdValueW<I3cd01cSpec> {
        ResponseBufferThresholdValueW::new(self, 8)
    }
    #[doc = "Bits 16:20 - IBI Data Threshold Value"]
    #[inline(always)]
    pub fn ibidata_threshold_value(&mut self) -> IbidataThresholdValueW<I3cd01cSpec> {
        IbidataThresholdValueW::new(self, 16)
    }
    #[doc = "Bits 24:31 - In-Band Interrupt Status Threshold Value."]
    #[inline(always)]
    pub fn in_band_intstatus_threshold_value(
        &mut self,
    ) -> InBandIntstatusThresholdValueW<I3cd01cSpec> {
        InBandIntstatusThresholdValueW::new(self, 24)
    }
}
#[doc = "Queue Threshold Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd01cSpec;
impl crate::RegisterSpec for I3cd01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd01c::R`](R) reader structure"]
impl crate::Readable for I3cd01cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd01c::W`](W) writer structure"]
impl crate::Writable for I3cd01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD01C to value 0"]
impl crate::Resettable for I3cd01cSpec {}
