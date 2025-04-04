#[doc = "Register `IIR` reader"]
pub type R = crate::R<IirSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Interrupt Identity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`reset()` method sets IIR to value 0"]
impl crate::Resettable for IirSpec {
    const RESET_VALUE: u32 = 0;
}
