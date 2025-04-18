#[doc = "Register `WDT034` reader"]
pub type R = crate::R<Wdt034Spec>;
#[doc = "Register `WDT034` writer"]
pub type W = crate::W<Wdt034Spec>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT04` reader - Eanble Write Protection of WDT04"]
pub type EanbleWrProtOfWdt04R = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT04` writer - Eanble Write Protection of WDT04"]
pub type EanbleWrProtOfWdt04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT08` reader - Eanble Write Protection of WDT08"]
pub type EanbleWrProtOfWdt08R = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT08` writer - Eanble Write Protection of WDT08"]
pub type EanbleWrProtOfWdt08W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT0C` reader - Eanble Write Protection of WDT0C"]
pub type EanbleWrProtOfWdt0cR = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT0C` writer - Eanble Write Protection of WDT0C"]
pub type EanbleWrProtOfWdt0cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT14` reader - Eanble Write Protection of WDT14"]
pub type EanbleWrProtOfWdt14R = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT14` writer - Eanble Write Protection of WDT14"]
pub type EanbleWrProtOfWdt14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT18` reader - Eanble Write Protection of WDT18"]
pub type EanbleWrProtOfWdt18R = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT18` writer - Eanble Write Protection of WDT18"]
pub type EanbleWrProtOfWdt18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT1C` reader - Eanble Write Protection of WDT1C"]
pub type EanbleWrProtOfWdt1cR = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT1C` writer - Eanble Write Protection of WDT1C"]
pub type EanbleWrProtOfWdt1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT20` reader - Eanble Write Protection of WDT20"]
pub type EanbleWrProtOfWdt20R = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT20` writer - Eanble Write Protection of WDT20"]
pub type EanbleWrProtOfWdt20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT24` reader - Eanble Write Protection of WDT24"]
pub type EanbleWrProtOfWdt24R = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT24` writer - Eanble Write Protection of WDT24"]
pub type EanbleWrProtOfWdt24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT28` reader - Eanble Write Protection of WDT28"]
pub type EanbleWrProtOfWdt28R = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT28` writer - Eanble Write Protection of WDT28"]
pub type EanbleWrProtOfWdt28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EanbleWrProtOfWDT2C` reader - Eanble Write Protection of WDT2C"]
pub type EanbleWrProtOfWdt2cR = crate::BitReader;
#[doc = "Field `EanbleWrProtOfWDT2C` writer - Eanble Write Protection of WDT2C"]
pub type EanbleWrProtOfWdt2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblClearBit12To0WhenCPURst` reader - Enable clear bit 12 to 0 when CPU reset"]
pub type EnblClearBit12to0whenCpurstR = crate::BitReader;
#[doc = "Field `EnblClearBit12To0WhenCPURst` writer - Enable clear bit 12 to 0 when CPU reset"]
pub type EnblClearBit12to0whenCpurstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Eanble Write Protection of WDT04"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt04(&self) -> EanbleWrProtOfWdt04R {
        EanbleWrProtOfWdt04R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Eanble Write Protection of WDT08"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt08(&self) -> EanbleWrProtOfWdt08R {
        EanbleWrProtOfWdt08R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Eanble Write Protection of WDT0C"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt0c(&self) -> EanbleWrProtOfWdt0cR {
        EanbleWrProtOfWdt0cR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Eanble Write Protection of WDT14"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt14(&self) -> EanbleWrProtOfWdt14R {
        EanbleWrProtOfWdt14R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Eanble Write Protection of WDT18"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt18(&self) -> EanbleWrProtOfWdt18R {
        EanbleWrProtOfWdt18R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Eanble Write Protection of WDT1C"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt1c(&self) -> EanbleWrProtOfWdt1cR {
        EanbleWrProtOfWdt1cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Eanble Write Protection of WDT20"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt20(&self) -> EanbleWrProtOfWdt20R {
        EanbleWrProtOfWdt20R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Eanble Write Protection of WDT24"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt24(&self) -> EanbleWrProtOfWdt24R {
        EanbleWrProtOfWdt24R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Eanble Write Protection of WDT28"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt28(&self) -> EanbleWrProtOfWdt28R {
        EanbleWrProtOfWdt28R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Eanble Write Protection of WDT2C"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt2c(&self) -> EanbleWrProtOfWdt2cR {
        EanbleWrProtOfWdt2cR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable clear bit 12 to 0 when CPU reset"]
    #[inline(always)]
    pub fn enbl_clear_bit12to0when_cpurst(&self) -> EnblClearBit12to0whenCpurstR {
        EnblClearBit12to0whenCpurstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Wdt034Spec> {
        Reserved3W::new(self, 0)
    }
    #[doc = "Bit 1 - Eanble Write Protection of WDT04"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt04(&mut self) -> EanbleWrProtOfWdt04W<Wdt034Spec> {
        EanbleWrProtOfWdt04W::new(self, 1)
    }
    #[doc = "Bit 2 - Eanble Write Protection of WDT08"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt08(&mut self) -> EanbleWrProtOfWdt08W<Wdt034Spec> {
        EanbleWrProtOfWdt08W::new(self, 2)
    }
    #[doc = "Bit 3 - Eanble Write Protection of WDT0C"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt0c(&mut self) -> EanbleWrProtOfWdt0cW<Wdt034Spec> {
        EanbleWrProtOfWdt0cW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Wdt034Spec> {
        Reserved2W::new(self, 4)
    }
    #[doc = "Bit 5 - Eanble Write Protection of WDT14"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt14(&mut self) -> EanbleWrProtOfWdt14W<Wdt034Spec> {
        EanbleWrProtOfWdt14W::new(self, 5)
    }
    #[doc = "Bit 6 - Eanble Write Protection of WDT18"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt18(&mut self) -> EanbleWrProtOfWdt18W<Wdt034Spec> {
        EanbleWrProtOfWdt18W::new(self, 6)
    }
    #[doc = "Bit 7 - Eanble Write Protection of WDT1C"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt1c(&mut self) -> EanbleWrProtOfWdt1cW<Wdt034Spec> {
        EanbleWrProtOfWdt1cW::new(self, 7)
    }
    #[doc = "Bit 8 - Eanble Write Protection of WDT20"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt20(&mut self) -> EanbleWrProtOfWdt20W<Wdt034Spec> {
        EanbleWrProtOfWdt20W::new(self, 8)
    }
    #[doc = "Bit 9 - Eanble Write Protection of WDT24"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt24(&mut self) -> EanbleWrProtOfWdt24W<Wdt034Spec> {
        EanbleWrProtOfWdt24W::new(self, 9)
    }
    #[doc = "Bit 10 - Eanble Write Protection of WDT28"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt28(&mut self) -> EanbleWrProtOfWdt28W<Wdt034Spec> {
        EanbleWrProtOfWdt28W::new(self, 10)
    }
    #[doc = "Bit 11 - Eanble Write Protection of WDT2C"]
    #[inline(always)]
    pub fn eanble_wr_prot_of_wdt2c(&mut self) -> EanbleWrProtOfWdt2cW<Wdt034Spec> {
        EanbleWrProtOfWdt2cW::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Wdt034Spec> {
        Reserved1W::new(self, 12)
    }
    #[doc = "Bit 31 - Enable clear bit 12 to 0 when CPU reset"]
    #[inline(always)]
    pub fn enbl_clear_bit12to0when_cpurst(&mut self) -> EnblClearBit12to0whenCpurstW<Wdt034Spec> {
        EnblClearBit12to0whenCpurstW::new(self, 31)
    }
}
#[doc = "WDTn Write Protection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdt034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdt034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wdt034Spec;
impl crate::RegisterSpec for Wdt034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdt034::R`](R) reader structure"]
impl crate::Readable for Wdt034Spec {}
#[doc = "`write(|w| ..)` method takes [`wdt034::W`](W) writer structure"]
impl crate::Writable for Wdt034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDT034 to value 0"]
impl crate::Resettable for Wdt034Spec {}
