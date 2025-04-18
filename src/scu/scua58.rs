#[doc = "Register `SCUA58` reader"]
pub type R = crate::R<Scua58Spec>;
#[doc = "Register `SCUA58` writer"]
pub type W = crate::W<Scua58Spec>;
#[doc = "Field `CacheEnbl` reader - Cache Enable"]
pub type CacheEnblR = crate::BitReader;
#[doc = "Field `CacheEnbl` writer - Cache Enable"]
pub type CacheEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClearDataCacheData` reader - Clear data cache data"]
pub type ClearDataCacheDataR = crate::BitReader;
#[doc = "Field `ClearDataCacheData` writer - Clear data cache data"]
pub type ClearDataCacheDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClearInstructionCache` reader - Clear instruction cache"]
pub type ClearInstructionCacheR = crate::BitReader;
#[doc = "Field `ClearInstructionCache` writer - Clear instruction cache"]
pub type ClearInstructionCacheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEnblForCM4FInstructiondataSysBusAddrMonitoring` reader - Interrupt enable for CM4F instruction/data/system bus address monitoring"]
pub type IntenblForCm4finstructiondataSysBusAddrMonitoringR = crate::BitReader;
#[doc = "Field `INTEnblForCM4FInstructiondataSysBusAddrMonitoring` writer - Interrupt enable for CM4F instruction/data/system bus address monitoring"]
pub type IntenblForCm4finstructiondataSysBusAddrMonitoringW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMaskForCM4FInstructiondataSysBusAddrMonitoring` reader - Interrupt mask for CM4F instruction/data/system bus address monitoring"]
pub type IntmaskForCm4finstructiondataSysBusAddrMonitoringR = crate::BitReader;
#[doc = "Field `INTMaskForCM4FInstructiondataSysBusAddrMonitoring` writer - Interrupt mask for CM4F instruction/data/system bus address monitoring"]
pub type IntmaskForCm4finstructiondataSysBusAddrMonitoringW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTClearForCM4FInstructiondataSysBusAddrMonitoring` reader - Interrupt clear for CM4F instruction/data/system bus address monitoring"]
pub type IntclearForCm4finstructiondataSysBusAddrMonitoringR = crate::BitReader;
#[doc = "Field `INTClearForCM4FInstructiondataSysBusAddrMonitoring` writer - Interrupt clear for CM4F instruction/data/system bus address monitoring"]
pub type IntclearForCm4finstructiondataSysBusAddrMonitoringW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RawINTStsForCM4FInstructionBusAddrMonitoring` reader - Raw interrupt status for CM4F instruction bus address monitoring"]
pub type RawIntstsForCm4finstructionBusAddrMonitoringR = crate::BitReader;
#[doc = "Field `RawINTStsForCM4FDataBusAddrMonitoring` reader - Raw interrupt status for CM4F data bus address monitoring"]
pub type RawIntstsForCm4fdataBusAddrMonitoringR = crate::BitReader;
#[doc = "Field `RawINTStsForCM4FSysBusAddrMonitoring` reader - Raw interrupt status for CM4F system bus address monitoring"]
pub type RawIntstsForCm4fsysBusAddrMonitoringR = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `MaskedINTStsForCM4FInstructionBusAddrMonitoring` reader - Masked interrupt status for CM4F instruction bus address monitoring"]
pub type MaskedIntstsForCm4finstructionBusAddrMonitoringR = crate::BitReader;
#[doc = "Field `MaskedINTStsForCM4FDataBusAddrMonitoring` reader - Masked interrupt status for CM4F data bus address monitoring"]
pub type MaskedIntstsForCm4fdataBusAddrMonitoringR = crate::BitReader;
#[doc = "Field `MaskedINTStsForCM4FSysBusAddrMonitoring` reader - Masked interrupt status for CM4F system bus address monitoring"]
pub type MaskedIntstsForCm4fsysBusAddrMonitoringR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    pub fn cache_enbl(&self) -> CacheEnblR {
        CacheEnblR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear data cache data"]
    #[inline(always)]
    pub fn clear_data_cache_data(&self) -> ClearDataCacheDataR {
        ClearDataCacheDataR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear instruction cache"]
    #[inline(always)]
    pub fn clear_instruction_cache(&self) -> ClearInstructionCacheR {
        ClearInstructionCacheR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 3) & 0x7f) as u8)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt enable for CM4F instruction/data/system bus address monitoring"]
    #[inline(always)]
    pub fn intenbl_for_cm4finstructiondata_sys_bus_addr_monitoring(
        &self,
    ) -> IntenblForCm4finstructiondataSysBusAddrMonitoringR {
        IntenblForCm4finstructiondataSysBusAddrMonitoringR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt mask for CM4F instruction/data/system bus address monitoring"]
    #[inline(always)]
    pub fn intmask_for_cm4finstructiondata_sys_bus_addr_monitoring(
        &self,
    ) -> IntmaskForCm4finstructiondataSysBusAddrMonitoringR {
        IntmaskForCm4finstructiondataSysBusAddrMonitoringR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt clear for CM4F instruction/data/system bus address monitoring"]
    #[inline(always)]
    pub fn intclear_for_cm4finstructiondata_sys_bus_addr_monitoring(
        &self,
    ) -> IntclearForCm4finstructiondataSysBusAddrMonitoringR {
        IntclearForCm4finstructiondataSysBusAddrMonitoringR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Raw interrupt status for CM4F instruction bus address monitoring"]
    #[inline(always)]
    pub fn raw_intsts_for_cm4finstruction_bus_addr_monitoring(
        &self,
    ) -> RawIntstsForCm4finstructionBusAddrMonitoringR {
        RawIntstsForCm4finstructionBusAddrMonitoringR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Raw interrupt status for CM4F data bus address monitoring"]
    #[inline(always)]
    pub fn raw_intsts_for_cm4fdata_bus_addr_monitoring(
        &self,
    ) -> RawIntstsForCm4fdataBusAddrMonitoringR {
        RawIntstsForCm4fdataBusAddrMonitoringR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Raw interrupt status for CM4F system bus address monitoring"]
    #[inline(always)]
    pub fn raw_intsts_for_cm4fsys_bus_addr_monitoring(
        &self,
    ) -> RawIntstsForCm4fsysBusAddrMonitoringR {
        RawIntstsForCm4fsysBusAddrMonitoringR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Masked interrupt status for CM4F instruction bus address monitoring"]
    #[inline(always)]
    pub fn masked_intsts_for_cm4finstruction_bus_addr_monitoring(
        &self,
    ) -> MaskedIntstsForCm4finstructionBusAddrMonitoringR {
        MaskedIntstsForCm4finstructionBusAddrMonitoringR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Masked interrupt status for CM4F data bus address monitoring"]
    #[inline(always)]
    pub fn masked_intsts_for_cm4fdata_bus_addr_monitoring(
        &self,
    ) -> MaskedIntstsForCm4fdataBusAddrMonitoringR {
        MaskedIntstsForCm4fdataBusAddrMonitoringR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Masked interrupt status for CM4F system bus address monitoring"]
    #[inline(always)]
    pub fn masked_intsts_for_cm4fsys_bus_addr_monitoring(
        &self,
    ) -> MaskedIntstsForCm4fsysBusAddrMonitoringR {
        MaskedIntstsForCm4fsysBusAddrMonitoringR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    pub fn cache_enbl(&mut self) -> CacheEnblW<Scua58Spec> {
        CacheEnblW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear data cache data"]
    #[inline(always)]
    pub fn clear_data_cache_data(&mut self) -> ClearDataCacheDataW<Scua58Spec> {
        ClearDataCacheDataW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear instruction cache"]
    #[inline(always)]
    pub fn clear_instruction_cache(&mut self) -> ClearInstructionCacheW<Scua58Spec> {
        ClearInstructionCacheW::new(self, 2)
    }
    #[doc = "Bits 3:9 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Scua58Spec> {
        Reserved6W::new(self, 3)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Scua58Spec> {
        Reserved5W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scua58Spec> {
        Reserved4W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt enable for CM4F instruction/data/system bus address monitoring"]
    #[inline(always)]
    pub fn intenbl_for_cm4finstructiondata_sys_bus_addr_monitoring(
        &mut self,
    ) -> IntenblForCm4finstructiondataSysBusAddrMonitoringW<Scua58Spec> {
        IntenblForCm4finstructiondataSysBusAddrMonitoringW::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt mask for CM4F instruction/data/system bus address monitoring"]
    #[inline(always)]
    pub fn intmask_for_cm4finstructiondata_sys_bus_addr_monitoring(
        &mut self,
    ) -> IntmaskForCm4finstructiondataSysBusAddrMonitoringW<Scua58Spec> {
        IntmaskForCm4finstructiondataSysBusAddrMonitoringW::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt clear for CM4F instruction/data/system bus address monitoring"]
    #[inline(always)]
    pub fn intclear_for_cm4finstructiondata_sys_bus_addr_monitoring(
        &mut self,
    ) -> IntclearForCm4finstructiondataSysBusAddrMonitoringW<Scua58Spec> {
        IntclearForCm4finstructiondataSysBusAddrMonitoringW::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scua58Spec> {
        Reserved3W::new(self, 15)
    }
}
#[doc = "CM4F Cache Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scua58Spec;
impl crate::RegisterSpec for Scua58Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scua58::R`](R) reader structure"]
impl crate::Readable for Scua58Spec {}
#[doc = "`write(|w| ..)` method takes [`scua58::W`](W) writer structure"]
impl crate::Writable for Scua58Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUA58 to value 0x01"]
impl crate::Resettable for Scua58Spec {
    const RESET_VALUE: u32 = 0x01;
}
