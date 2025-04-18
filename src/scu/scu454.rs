#[doc = "Register `SCU454` reader"]
pub type R = crate::R<Scu454Spec>;
#[doc = "Register `SCU454` writer"]
pub type W = crate::W<Scu454Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `EnblJTAGMasterRouting` reader - Enable JTAG master routing"]
pub type EnblJtagmasterRoutingR = crate::BitReader;
#[doc = "Field `EnblJTAGMasterRouting` writer - Enable JTAG master routing"]
pub type EnblJtagmasterRoutingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Enable JTAG master routing"]
    #[inline(always)]
    pub fn enbl_jtagmaster_routing(&self) -> EnblJtagmasterRoutingR {
        EnblJtagmasterRoutingR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu454Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 20 - Enable JTAG master routing"]
    #[inline(always)]
    pub fn enbl_jtagmaster_routing(&mut self) -> EnblJtagmasterRoutingW<Scu454Spec> {
        EnblJtagmasterRoutingW::new(self, 20)
    }
}
#[doc = "Multi-function Pin Control \\#10\n\nYou can [`read`](crate::Reg::read) this register and get [`scu454::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu454::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu454Spec;
impl crate::RegisterSpec for Scu454Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu454::R`](R) reader structure"]
impl crate::Readable for Scu454Spec {}
#[doc = "`write(|w| ..)` method takes [`scu454::W`](W) writer structure"]
impl crate::Writable for Scu454Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU454 to value 0"]
impl crate::Resettable for Scu454Spec {}
