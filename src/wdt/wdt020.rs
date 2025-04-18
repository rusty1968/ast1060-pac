#[doc = "Register `WDT020` reader"]
pub type R = crate::R<Wdt020Spec>;
#[doc = "Register `WDT020` writer"]
pub type W = crate::W<Wdt020Spec>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::FieldReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EnblRstSOCCtrl` reader - Enable reset SOC controller"]
pub type EnblRstSocctrlR = crate::BitReader;
#[doc = "Field `EnblRstSOCCtrl` writer - Enable reset SOC controller"]
pub type EnblRstSocctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstMACCtrl` reader - Enable reset MAC controller"]
pub type EnblRstMacctrlR = crate::BitReader;
#[doc = "Field `EnblRstMACCtrl` writer - Enable reset MAC controller"]
pub type EnblRstMacctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstJTAG1MasterCtrl` reader - Enable reset JTAG #1 master controller"]
pub type EnblRstJtag1masterCtrlR = crate::BitReader;
#[doc = "Field `EnblRstJTAG1MasterCtrl` writer - Enable reset JTAG #1 master controller"]
pub type EnblRstJtag1masterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstJTAG2MasterCtrl` reader - Enable reset JTAG #2 master controller"]
pub type EnblRstJtag2masterCtrlR = crate::BitReader;
#[doc = "Field `EnblRstJTAG2MasterCtrl` writer - Enable reset JTAG #2 master controller"]
pub type EnblRstJtag2masterCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstGPIOCtrl` reader - Enable reset GPIO controller"]
pub type EnblRstGpioctrlR = crate::BitReader;
#[doc = "Field `EnblRstGPIOCtrl` writer - Enable reset GPIO controller"]
pub type EnblRstGpioctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstMDCMDIOCtrl` reader - Enable reset MDC/MDIO controller"]
pub type EnblRstMdcmdioctrlR = crate::BitReader;
#[doc = "Field `EnblRstMDCMDIOCtrl` writer - Enable reset MDC/MDIO controller"]
pub type EnblRstMdcmdioctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EnblRstLPCCtrl` reader - Enable reset LPC controller"]
pub type EnblRstLpcctrlR = crate::BitReader;
#[doc = "Field `EnblRstLPCCtrl` writer - Enable reset LPC controller"]
pub type EnblRstLpcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstPECICtrl` reader - Enable reset PECI controller"]
pub type EnblRstPecictrlR = crate::BitReader;
#[doc = "Field `EnblRstPECICtrl` writer - Enable reset PECI controller"]
pub type EnblRstPecictrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstPWMCtrl` reader - Enable reset PWM controller"]
pub type EnblRstPwmctrlR = crate::BitReader;
#[doc = "Field `EnblRstPWMCtrl` writer - Enable reset PWM controller"]
pub type EnblRstPwmctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstADCCtrl` reader - Enable reset ADC controller"]
pub type EnblRstAdcctrlR = crate::BitReader;
#[doc = "Field `EnblRstADCCtrl` writer - Enable reset ADC controller"]
pub type EnblRstAdcctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI2CI2CFLTCtrl` reader - Enable reset I2C/I2CFLT controller"]
pub type EnblRstI2ci2cfltctrlR = crate::BitReader;
#[doc = "Field `EnblRstI2CI2CFLTCtrl` writer - Enable reset I2C/I2CFLT controller"]
pub type EnblRstI2ci2cfltctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CGlobalCtrl` reader - Enable reset I3C Global controller"]
pub type EnblRstI3cglobalCtrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CGlobalCtrl` writer - Enable reset I3C Global controller"]
pub type EnblRstI3cglobalCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus1Ctrl` reader - Enable reset I3C bus1 controller"]
pub type EnblRstI3cbus1ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus1Ctrl` writer - Enable reset I3C bus1 controller"]
pub type EnblRstI3cbus1ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus2Ctrl` reader - Enable reset I3C bus2 controller"]
pub type EnblRstI3cbus2ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus2Ctrl` writer - Enable reset I3C bus2 controller"]
pub type EnblRstI3cbus2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus3Ctrl` reader - Enable reset I3C bus3 controller"]
pub type EnblRstI3cbus3ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus3Ctrl` writer - Enable reset I3C bus3 controller"]
pub type EnblRstI3cbus3ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstI3CBus4Ctrl` reader - Enable reset I3C bus4 controller"]
pub type EnblRstI3cbus4ctrlR = crate::BitReader;
#[doc = "Field `EnblRstI3CBus4Ctrl` writer - Enable reset I3C bus4 controller"]
pub type EnblRstI3cbus4ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRstSPI1SPI2Ctrl` reader - Enable reset SPI1/SPI2 controller"]
pub type EnblRstSpi1spi2ctrlR = crate::BitReader;
#[doc = "Field `EnblRstSPI1SPI2Ctrl` writer - Enable reset SPI1/SPI2 controller"]
pub type EnblRstSpi1spi2ctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `EnblRstESPICtrl` reader - Enable reset eSPI controller"]
pub type EnblRstEspictrlR = crate::BitReader;
#[doc = "Field `EnblRstESPICtrl` writer - Enable reset eSPI controller"]
pub type EnblRstEspictrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&self) -> EnblRstSocctrlR {
        EnblRstSocctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable reset MAC controller"]
    #[inline(always)]
    pub fn enbl_rst_macctrl(&self) -> EnblRstMacctrlR {
        EnblRstMacctrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable reset JTAG #1 master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtag1master_ctrl(&self) -> EnblRstJtag1masterCtrlR {
        EnblRstJtag1masterCtrlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable reset JTAG #2 master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtag2master_ctrl(&self) -> EnblRstJtag2masterCtrlR {
        EnblRstJtag2masterCtrlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable reset GPIO controller"]
    #[inline(always)]
    pub fn enbl_rst_gpioctrl(&self) -> EnblRstGpioctrlR {
        EnblRstGpioctrlR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable reset MDC/MDIO controller"]
    #[inline(always)]
    pub fn enbl_rst_mdcmdioctrl(&self) -> EnblRstMdcmdioctrlR {
        EnblRstMdcmdioctrlR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Enable reset LPC controller"]
    #[inline(always)]
    pub fn enbl_rst_lpcctrl(&self) -> EnblRstLpcctrlR {
        EnblRstLpcctrlR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable reset PECI controller"]
    #[inline(always)]
    pub fn enbl_rst_pecictrl(&self) -> EnblRstPecictrlR {
        EnblRstPecictrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable reset PWM controller"]
    #[inline(always)]
    pub fn enbl_rst_pwmctrl(&self) -> EnblRstPwmctrlR {
        EnblRstPwmctrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable reset ADC controller"]
    #[inline(always)]
    pub fn enbl_rst_adcctrl(&self) -> EnblRstAdcctrlR {
        EnblRstAdcctrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable reset I2C/I2CFLT controller"]
    #[inline(always)]
    pub fn enbl_rst_i2ci2cfltctrl(&self) -> EnblRstI2ci2cfltctrlR {
        EnblRstI2ci2cfltctrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable reset I3C Global controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cglobal_ctrl(&self) -> EnblRstI3cglobalCtrlR {
        EnblRstI3cglobalCtrlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable reset I3C bus1 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus1ctrl(&self) -> EnblRstI3cbus1ctrlR {
        EnblRstI3cbus1ctrlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable reset I3C bus2 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus2ctrl(&self) -> EnblRstI3cbus2ctrlR {
        EnblRstI3cbus2ctrlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable reset I3C bus3 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus3ctrl(&self) -> EnblRstI3cbus3ctrlR {
        EnblRstI3cbus3ctrlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable reset I3C bus4 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus4ctrl(&self) -> EnblRstI3cbus4ctrlR {
        EnblRstI3cbus4ctrlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable reset SPI1/SPI2 controller"]
    #[inline(always)]
    pub fn enbl_rst_spi1spi2ctrl(&self) -> EnblRstSpi1spi2ctrlR {
        EnblRstSpi1spi2ctrlR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 23:31 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
    #[doc = "Bit 26 - Enable reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_rst_espictrl(&self) -> EnblRstEspictrlR {
        EnblRstEspictrlR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<Wdt020Spec> {
        Reserved7W::new(self, 0)
    }
    #[doc = "Bit 4 - Enable reset SOC controller"]
    #[inline(always)]
    pub fn enbl_rst_socctrl(&mut self) -> EnblRstSocctrlW<Wdt020Spec> {
        EnblRstSocctrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable reset MAC controller"]
    #[inline(always)]
    pub fn enbl_rst_macctrl(&mut self) -> EnblRstMacctrlW<Wdt020Spec> {
        EnblRstMacctrlW::new(self, 5)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<Wdt020Spec> {
        Reserved6W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<Wdt020Spec> {
        Reserved5W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable reset JTAG #1 master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtag1master_ctrl(&mut self) -> EnblRstJtag1masterCtrlW<Wdt020Spec> {
        EnblRstJtag1masterCtrlW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable reset JTAG #2 master controller"]
    #[inline(always)]
    pub fn enbl_rst_jtag2master_ctrl(&mut self) -> EnblRstJtag2masterCtrlW<Wdt020Spec> {
        EnblRstJtag2masterCtrlW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable reset GPIO controller"]
    #[inline(always)]
    pub fn enbl_rst_gpioctrl(&mut self) -> EnblRstGpioctrlW<Wdt020Spec> {
        EnblRstGpioctrlW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable reset MDC/MDIO controller"]
    #[inline(always)]
    pub fn enbl_rst_mdcmdioctrl(&mut self) -> EnblRstMdcmdioctrlW<Wdt020Spec> {
        EnblRstMdcmdioctrlW::new(self, 10)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Wdt020Spec> {
        Reserved4W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable reset LPC controller"]
    #[inline(always)]
    pub fn enbl_rst_lpcctrl(&mut self) -> EnblRstLpcctrlW<Wdt020Spec> {
        EnblRstLpcctrlW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable reset PECI controller"]
    #[inline(always)]
    pub fn enbl_rst_pecictrl(&mut self) -> EnblRstPecictrlW<Wdt020Spec> {
        EnblRstPecictrlW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable reset PWM controller"]
    #[inline(always)]
    pub fn enbl_rst_pwmctrl(&mut self) -> EnblRstPwmctrlW<Wdt020Spec> {
        EnblRstPwmctrlW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable reset ADC controller"]
    #[inline(always)]
    pub fn enbl_rst_adcctrl(&mut self) -> EnblRstAdcctrlW<Wdt020Spec> {
        EnblRstAdcctrlW::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt020Spec> {
        Reserved3W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable reset I2C/I2CFLT controller"]
    #[inline(always)]
    pub fn enbl_rst_i2ci2cfltctrl(&mut self) -> EnblRstI2ci2cfltctrlW<Wdt020Spec> {
        EnblRstI2ci2cfltctrlW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable reset I3C Global controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cglobal_ctrl(&mut self) -> EnblRstI3cglobalCtrlW<Wdt020Spec> {
        EnblRstI3cglobalCtrlW::new(self, 17)
    }
    #[doc = "Bit 18 - Enable reset I3C bus1 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus1ctrl(&mut self) -> EnblRstI3cbus1ctrlW<Wdt020Spec> {
        EnblRstI3cbus1ctrlW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable reset I3C bus2 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus2ctrl(&mut self) -> EnblRstI3cbus2ctrlW<Wdt020Spec> {
        EnblRstI3cbus2ctrlW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable reset I3C bus3 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus3ctrl(&mut self) -> EnblRstI3cbus3ctrlW<Wdt020Spec> {
        EnblRstI3cbus3ctrlW::new(self, 20)
    }
    #[doc = "Bit 21 - Enable reset I3C bus4 controller"]
    #[inline(always)]
    pub fn enbl_rst_i3cbus4ctrl(&mut self) -> EnblRstI3cbus4ctrlW<Wdt020Spec> {
        EnblRstI3cbus4ctrlW::new(self, 21)
    }
    #[doc = "Bit 22 - Enable reset SPI1/SPI2 controller"]
    #[inline(always)]
    pub fn enbl_rst_spi1spi2ctrl(&mut self) -> EnblRstSpi1spi2ctrlW<Wdt020Spec> {
        EnblRstSpi1spi2ctrlW::new(self, 22)
    }
    #[doc = "Bits 23:25 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt020Spec> {
        Reserved1W::new(self, 23)
    }
    #[doc = "Bits 23:31 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt020Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bit 26 - Enable reset eSPI controller"]
    #[inline(always)]
    pub fn enbl_rst_espictrl(&mut self) -> EnblRstEspictrlW<Wdt020Spec> {
        EnblRstEspictrlW::new(self, 26)
    }
}
#[doc = "WDTn Reset Mask Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt020Spec;
impl crate::RegisterSpec for Wdt020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt020::R`](R) reader structure"]
impl crate::Readable for Wdt020Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt020::W`](W) writer structure"]
impl crate::Writable for Wdt020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT020 to value 0x03ff_fff1"]
impl crate::Resettable for Wdt020Spec {
    const RESET_VALUE: u32 = 0x03ff_fff1;
}
