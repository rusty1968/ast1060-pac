#[doc = "Register `GPIO574` reader"]
pub type R = crate::R<Gpio574Spec>;
#[doc = "Register `GPIO574` writer"]
pub type W = crate::W<Gpio574Spec>;
#[doc = "Field `DataWrittenToGPIO51C` reader - Data written to GPIO51C"]
pub type DataWrittenToGpio51cR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data written to GPIO51C"]
    #[inline(always)]
    pub fn data_written_to_gpio51c(&self) -> DataWrittenToGpio51cR {
        DataWrittenToGpio51cR::new(self.bits)
    }
}
impl W {}
#[doc = "Serial GPIO\\_E/F/G/H 1 Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio574::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio574::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio574Spec;
impl crate::RegisterSpec for Gpio574Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio574::R`](R) reader structure"]
impl crate::Readable for Gpio574Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio574::W`](W) writer structure"]
impl crate::Writable for Gpio574Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO574 to value 0"]
impl crate::Resettable for Gpio574Spec {}
