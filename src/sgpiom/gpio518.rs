#[doc = "Register `GPIO518` reader"]
pub type R = crate::R<Gpio518Spec>;
#[doc = "Register `GPIO518` writer"]
pub type W = crate::W<Gpio518Spec>;
#[doc = "Field `PortSerialGPIOA70WDTRstToleranceEnbl` reader - Port Serial GPIOA\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioa70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOA70WDTRstToleranceEnbl` writer - Port Serial GPIOA\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioa70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOB70WDTRstToleranceEnbl` reader - Port Serial GPIOB\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiob70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOB70WDTRstToleranceEnbl` writer - Port Serial GPIOB\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiob70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOC70WDTRstToleranceEnbl` reader - Port Serial GPIOC\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioc70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOC70WDTRstToleranceEnbl` writer - Port Serial GPIOC\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioc70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOD70WDTRstToleranceEnbl` reader - Port Serial GPIOD\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiod70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOD70WDTRstToleranceEnbl` writer - Port Serial GPIOD\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiod70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioa70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpioa70wdtrstToleranceEnblR {
        PortSerialGpioa70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiob70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpiob70wdtrstToleranceEnblR {
        PortSerialGpiob70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioc70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpioc70wdtrstToleranceEnblR {
        PortSerialGpioc70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiod70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpiod70wdtrstToleranceEnblR {
        PortSerialGpiod70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOA\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioa70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpioa70wdtrstToleranceEnblW<Gpio518Spec> {
        PortSerialGpioa70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOB\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiob70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpiob70wdtrstToleranceEnblW<Gpio518Spec> {
        PortSerialGpiob70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOC\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioc70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpioc70wdtrstToleranceEnblW<Gpio518Spec> {
        PortSerialGpioc70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOD\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiod70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpiod70wdtrstToleranceEnblW<Gpio518Spec> {
        PortSerialGpiod70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_A/B/C/D 1 Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio518::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio518::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio518Spec;
impl crate::RegisterSpec for Gpio518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio518::R`](R) reader structure"]
impl crate::Readable for Gpio518Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio518::W`](W) writer structure"]
impl crate::Writable for Gpio518Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO518 to value 0"]
impl crate::Resettable for Gpio518Spec {}
