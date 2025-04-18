#[doc = "Register `GPIO0B0` reader"]
pub type R = crate::R<Gpio0b0Spec>;
#[doc = "Register `GPIO0B0` writer"]
pub type W = crate::W<Gpio0b0Spec>;
#[doc = "Field `PortGPIOI70DebounceSettingReg1` reader - Port GPIOI\\[7:0\\] debounce setting register #1"]
pub type PortGpioi70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOI70DebounceSettingReg1` writer - Port GPIOI\\[7:0\\] debounce setting register #1"]
pub type PortGpioi70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70DebounceSettingReg1` reader - Port GPIOJ\\[7:0\\] debounce setting register #1"]
pub type PortGpioj70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOJ70DebounceSettingReg1` writer - Port GPIOJ\\[7:0\\] debounce setting register #1"]
pub type PortGpioj70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70DebounceSettingReg1` reader - Port GPIOK\\[7:0\\] debounce setting register #1"]
pub type PortGpiok70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOK70DebounceSettingReg1` writer - Port GPIOK\\[7:0\\] debounce setting register #1"]
pub type PortGpiok70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70DebounceSettingReg1` reader - Port GPIOL\\[7:0\\] debounce setting register #1"]
pub type PortGpiol70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOL70DebounceSettingReg1` writer - Port GPIOL\\[7:0\\] debounce setting register #1"]
pub type PortGpiol70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioi70debounce_setting_reg1(&self) -> PortGpioi70debounceSettingReg1R {
        PortGpioi70debounceSettingReg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioj70debounce_setting_reg1(&self) -> PortGpioj70debounceSettingReg1R {
        PortGpioj70debounceSettingReg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiok70debounce_setting_reg1(&self) -> PortGpiok70debounceSettingReg1R {
        PortGpiok70debounceSettingReg1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiol70debounce_setting_reg1(&self) -> PortGpiol70debounceSettingReg1R {
        PortGpiol70debounceSettingReg1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioi70debounce_setting_reg1(
        &mut self,
    ) -> PortGpioi70debounceSettingReg1W<Gpio0b0Spec> {
        PortGpioi70debounceSettingReg1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioj70debounce_setting_reg1(
        &mut self,
    ) -> PortGpioj70debounceSettingReg1W<Gpio0b0Spec> {
        PortGpioj70debounceSettingReg1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiok70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiok70debounceSettingReg1W<Gpio0b0Spec> {
        PortGpiok70debounceSettingReg1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiol70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiol70debounceSettingReg1W<Gpio0b0Spec> {
        PortGpiol70debounceSettingReg1W::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0b0Spec;
impl crate::RegisterSpec for Gpio0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0b0::R`](R) reader structure"]
impl crate::Readable for Gpio0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0b0::W`](W) writer structure"]
impl crate::Writable for Gpio0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0B0 to value 0"]
impl crate::Resettable for Gpio0b0Spec {}
