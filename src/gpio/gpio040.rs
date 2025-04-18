#[doc = "Register `GPIO040` reader"]
pub type R = crate::R<Gpio040Spec>;
#[doc = "Register `GPIO040` writer"]
pub type W = crate::W<Gpio040Spec>;
#[doc = "Field `PortGPIOA70DebounceSettingReg1` reader - Port GPIOA\\[7:0\\] debounce setting register #1"]
pub type PortGpioa70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOA70DebounceSettingReg1` writer - Port GPIOA\\[7:0\\] debounce setting register #1"]
pub type PortGpioa70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70DebounceSettingReg1` reader - Port GPIOB\\[7:0\\] debounce setting register #1"]
pub type PortGpiob70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOB70DebounceSettingReg1` writer - Port GPIOB\\[7:0\\] debounce setting register #1"]
pub type PortGpiob70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70DebounceSettingReg1` reader - Port GPIOC\\[7:0\\] debounce setting register #1"]
pub type PortGpioc70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOC70DebounceSettingReg1` writer - Port GPIOC\\[7:0\\] debounce setting register #1"]
pub type PortGpioc70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70DebounceSettingReg1` reader - Port GPIOD\\[7:0\\] debounce setting register #1"]
pub type PortGpiod70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOD70DebounceSettingReg1` writer - Port GPIOD\\[7:0\\] debounce setting register #1"]
pub type PortGpiod70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioa70debounce_setting_reg1(&self) -> PortGpioa70debounceSettingReg1R {
        PortGpioa70debounceSettingReg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiob70debounce_setting_reg1(&self) -> PortGpiob70debounceSettingReg1R {
        PortGpiob70debounceSettingReg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioc70debounce_setting_reg1(&self) -> PortGpioc70debounceSettingReg1R {
        PortGpioc70debounceSettingReg1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiod70debounce_setting_reg1(&self) -> PortGpiod70debounceSettingReg1R {
        PortGpiod70debounceSettingReg1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioa70debounce_setting_reg1(
        &mut self,
    ) -> PortGpioa70debounceSettingReg1W<Gpio040Spec> {
        PortGpioa70debounceSettingReg1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiob70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiob70debounceSettingReg1W<Gpio040Spec> {
        PortGpiob70debounceSettingReg1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioc70debounce_setting_reg1(
        &mut self,
    ) -> PortGpioc70debounceSettingReg1W<Gpio040Spec> {
        PortGpioc70debounceSettingReg1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpiod70debounce_setting_reg1(
        &mut self,
    ) -> PortGpiod70debounceSettingReg1W<Gpio040Spec> {
        PortGpiod70debounceSettingReg1W::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio040Spec;
impl crate::RegisterSpec for Gpio040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio040::R`](R) reader structure"]
impl crate::Readable for Gpio040Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio040::W`](W) writer structure"]
impl crate::Writable for Gpio040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO040 to value 0"]
impl crate::Resettable for Gpio040Spec {}
