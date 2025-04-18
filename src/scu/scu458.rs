#[doc = "Register `SCU458` reader"]
pub type R = crate::R<Scu458Spec>;
#[doc = "Register `SCU458` writer"]
pub type W = crate::W<Scu458Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `VoltageOfGPION4N5N6N7O0O1O2O3` reader - Voltage of GPION4/N5/N6/N7/O0/O1/O2/O3."]
pub type VoltageOfGpion4n5n6n7o0o1o2o3R = crate::BitReader;
#[doc = "Field `VoltageOfGPION4N5N6N7O0O1O2O3` writer - Voltage of GPION4/N5/N6/N7/O0/O1/O2/O3."]
pub type VoltageOfGpion4n5n6n7o0o1o2o3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:19 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 20 - Voltage of GPION4/N5/N6/N7/O0/O1/O2/O3."]
    #[inline(always)]
    pub fn voltage_of_gpion4n5n6n7o0o1o2o3(&self) -> VoltageOfGpion4n5n6n7o0o1o2o3R {
        VoltageOfGpion4n5n6n7o0o1o2o3R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu458Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 20 - Voltage of GPION4/N5/N6/N7/O0/O1/O2/O3."]
    #[inline(always)]
    pub fn voltage_of_gpion4n5n6n7o0o1o2o3(
        &mut self,
    ) -> VoltageOfGpion4n5n6n7o0o1o2o3W<Scu458Spec> {
        VoltageOfGpion4n5n6n7o0o1o2o3W::new(self, 20)
    }
}
#[doc = "Multi-function Pin Control \\#11\n\nYou can [`read`](crate::Reg::read) this register and get [`scu458::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu458::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu458Spec;
impl crate::RegisterSpec for Scu458Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu458::R`](R) reader structure"]
impl crate::Readable for Scu458Spec {}
#[doc = "`write(|w| ..)` method takes [`scu458::W`](W) writer structure"]
impl crate::Writable for Scu458Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU458 to value 0x0500"]
impl crate::Resettable for Scu458Spec {
    const RESET_VALUE: u32 = 0x0500;
}
