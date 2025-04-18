#[doc = "Register `GPIO134` reader"]
pub type R = crate::R<Gpio134Spec>;
#[doc = "Register `GPIO134` writer"]
pub type W = crate::W<Gpio134Spec>;
#[doc = "Field `PortGPIOQ70DebounceSettingReg2` reader - Port GPIOQ\\[7:0\\] debounce setting register #2"]
pub type PortGpioq70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOQ70DebounceSettingReg2` writer - Port GPIOQ\\[7:0\\] debounce setting register #2"]
pub type PortGpioq70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70DebounceSettingReg2` reader - Port GPIOR\\[7:0\\] debounce setting register #2"]
pub type PortGpior70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOR70DebounceSettingReg2` writer - Port GPIOR\\[7:0\\] debounce setting register #2"]
pub type PortGpior70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70DebounceSettingReg2` reader - Port GPIOS\\[7:0\\] debounce setting register #2"]
pub type PortGpios70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOS70DebounceSettingReg2` writer - Port GPIOS\\[7:0\\] debounce setting register #2"]
pub type PortGpios70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70DebounceSettingReg2` reader - Port GPIT\\[7:0\\] debounce setting register #2"]
pub type PortGpit70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIT70DebounceSettingReg2` writer - Port GPIT\\[7:0\\] debounce setting register #2"]
pub type PortGpit70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioq70debounce_setting_reg2(&self) -> PortGpioq70debounceSettingReg2R {
        PortGpioq70debounceSettingReg2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpior70debounce_setting_reg2(&self) -> PortGpior70debounceSettingReg2R {
        PortGpior70debounceSettingReg2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpios70debounce_setting_reg2(&self) -> PortGpios70debounceSettingReg2R {
        PortGpios70debounceSettingReg2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpit70debounce_setting_reg2(&self) -> PortGpit70debounceSettingReg2R {
        PortGpit70debounceSettingReg2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioq70debounce_setting_reg2(
        &mut self,
    ) -> PortGpioq70debounceSettingReg2W<Gpio134Spec> {
        PortGpioq70debounceSettingReg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpior70debounce_setting_reg2(
        &mut self,
    ) -> PortGpior70debounceSettingReg2W<Gpio134Spec> {
        PortGpior70debounceSettingReg2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpios70debounce_setting_reg2(
        &mut self,
    ) -> PortGpios70debounceSettingReg2W<Gpio134Spec> {
        PortGpios70debounceSettingReg2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpit70debounce_setting_reg2(
        &mut self,
    ) -> PortGpit70debounceSettingReg2W<Gpio134Spec> {
        PortGpit70debounceSettingReg2W::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio134::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio134::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio134Spec;
impl crate::RegisterSpec for Gpio134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio134::R`](R) reader structure"]
impl crate::Readable for Gpio134Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio134::W`](W) writer structure"]
impl crate::Writable for Gpio134Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO134 to value 0"]
impl crate::Resettable for Gpio134Spec {}
