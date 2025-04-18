#[doc = "Register `SCU310` reader"]
pub type R = crate::R<Scu310Spec>;
#[doc = "Register `SCU310` writer"]
pub type W = crate::W<Scu310Spec>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SelectUART5ClkSrc` reader - Select UART5 clock source"]
pub type SelectUart5clkSrcR = crate::BitReader;
#[doc = "Field `SelectUART5ClkSrc` writer - Select UART5 clock source"]
pub type SelectUart5clkSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SelectUARTDebugPortClkSrc` reader - Select UART debug port clock source"]
pub type SelectUartdebugPortClkSrcR = crate::BitReader;
#[doc = "Field `SelectUARTDebugPortClkSrc` writer - Select UART debug port clock source"]
pub type SelectUartdebugPortClkSrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APBBusPCLKDividerSel` reader - APB Bus PCLK divider selection"]
pub type ApbbusPclkdividerSelR = crate::FieldReader;
#[doc = "Field `APBBusPCLKDividerSel` writer - APB Bus PCLK divider selection"]
pub type ApbbusPclkdividerSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I3CClkDividerSel` reader - I3C Clock divider selection"]
pub type I3cclkDividerSelR = crate::FieldReader;
#[doc = "Field `I3CClkDividerSel` writer - I3C Clock divider selection"]
pub type I3cclkDividerSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I3CClkSourceSel` reader - I3C Clock Source selection"]
pub type I3cclkSourceSelR = crate::BitReader;
#[doc = "Field `I3CClkSourceSel` writer - I3C Clock Source selection"]
pub type I3cclkSourceSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Select UART5 clock source"]
    #[inline(always)]
    pub fn select_uart5clk_src(&self) -> SelectUart5clkSrcR {
        SelectUart5clkSrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Select UART debug port clock source"]
    #[inline(always)]
    pub fn select_uartdebug_port_clk_src(&self) -> SelectUartdebugPortClkSrcR {
        SelectUartdebugPortClkSrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - APB Bus PCLK divider selection"]
    #[inline(always)]
    pub fn apbbus_pclkdivider_sel(&self) -> ApbbusPclkdividerSelR {
        ApbbusPclkdividerSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - I3C Clock divider selection"]
    #[inline(always)]
    pub fn i3cclk_divider_sel(&self) -> I3cclkDividerSelR {
        I3cclkDividerSelR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - I3C Clock Source selection"]
    #[inline(always)]
    pub fn i3cclk_source_sel(&self) -> I3cclkSourceSelR {
        I3cclkSourceSelR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu310Spec> {
        Reserved3W::new(self, 0)
    }
    #[doc = "Bit 4 - Select UART5 clock source"]
    #[inline(always)]
    pub fn select_uart5clk_src(&mut self) -> SelectUart5clkSrcW<Scu310Spec> {
        SelectUart5clkSrcW::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu310Spec> {
        Reserved2W::new(self, 5)
    }
    #[doc = "Bit 6 - Select UART debug port clock source"]
    #[inline(always)]
    pub fn select_uartdebug_port_clk_src(&mut self) -> SelectUartdebugPortClkSrcW<Scu310Spec> {
        SelectUartdebugPortClkSrcW::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu310Spec> {
        Reserved1W::new(self, 7)
    }
    #[doc = "Bits 8:11 - APB Bus PCLK divider selection"]
    #[inline(always)]
    pub fn apbbus_pclkdivider_sel(&mut self) -> ApbbusPclkdividerSelW<Scu310Spec> {
        ApbbusPclkdividerSelW::new(self, 8)
    }
    #[doc = "Bits 28:30 - I3C Clock divider selection"]
    #[inline(always)]
    pub fn i3cclk_divider_sel(&mut self) -> I3cclkDividerSelW<Scu310Spec> {
        I3cclkDividerSelW::new(self, 28)
    }
    #[doc = "Bit 31 - I3C Clock Source selection"]
    #[inline(always)]
    pub fn i3cclk_source_sel(&mut self) -> I3cclkSourceSelW<Scu310Spec> {
        I3cclkSourceSelW::new(self, 31)
    }
}
#[doc = "Clock Selection Register Set 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scu310::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu310::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu310Spec;
impl crate::RegisterSpec for Scu310Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu310::R`](R) reader structure"]
impl crate::Readable for Scu310Spec {}
#[doc = "`write(|w| ..)` method takes [`scu310::W`](W) writer structure"]
impl crate::Writable for Scu310Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU310 to value 0x43f9_0000"]
impl crate::Resettable for Scu310Spec {
    const RESET_VALUE: u32 = 0x43f9_0000;
}
