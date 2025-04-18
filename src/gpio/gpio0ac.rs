#[doc = "Register `GPIO0AC` reader"]
pub type R = crate::R<Gpio0acSpec>;
#[doc = "Register `GPIO0AC` writer"]
pub type W = crate::W<Gpio0acSpec>;
#[doc = "Field `PortGPIOI70WDTRstToleranceEnbl` reader - Port GPIOI\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioi70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOI70WDTRstToleranceEnbl` writer - Port GPIOI\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioi70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70WDTRstToleranceEnbl` reader - Port GPIOJ\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioj70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOJ70WDTRstToleranceEnbl` writer - Port GPIOJ\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioj70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70WDTRstToleranceEnbl` reader - Port GPIOK\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiok70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOK70WDTRstToleranceEnbl` writer - Port GPIOK\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiok70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70WDTRstToleranceEnbl` reader - Port GPIOL\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiol70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOL70WDTRstToleranceEnbl` writer - Port GPIOL\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiol70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioi70wdtrst_tolerance_enbl(&self) -> PortGpioi70wdtrstToleranceEnblR {
        PortGpioi70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioj70wdtrst_tolerance_enbl(&self) -> PortGpioj70wdtrstToleranceEnblR {
        PortGpioj70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiok70wdtrst_tolerance_enbl(&self) -> PortGpiok70wdtrstToleranceEnblR {
        PortGpiok70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiol70wdtrst_tolerance_enbl(&self) -> PortGpiol70wdtrstToleranceEnblR {
        PortGpiol70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioi70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpioi70wdtrstToleranceEnblW<Gpio0acSpec> {
        PortGpioi70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioj70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpioj70wdtrstToleranceEnblW<Gpio0acSpec> {
        PortGpioj70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiok70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiok70wdtrstToleranceEnblW<Gpio0acSpec> {
        PortGpiok70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiol70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiol70wdtrstToleranceEnblW<Gpio0acSpec> {
        PortGpiol70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0acSpec;
impl crate::RegisterSpec for Gpio0acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0ac::R`](R) reader structure"]
impl crate::Readable for Gpio0acSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0ac::W`](W) writer structure"]
impl crate::Writable for Gpio0acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0AC to value 0"]
impl crate::Resettable for Gpio0acSpec {}
