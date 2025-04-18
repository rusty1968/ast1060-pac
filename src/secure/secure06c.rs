#[doc = "Register `SECURE06C` reader"]
pub type R = crate::R<Secure06cSpec>;
#[doc = "Register `SECURE06C` writer"]
pub type W = crate::W<Secure06cSpec>;
#[doc = "Field `SecBootSwRevReg2` reader - Secure Boot Software Revision Register 2"]
pub type SecBootSwRevReg2R = crate::FieldReader<u32>;
#[doc = "Field `SecBootSwRevReg2` writer - Secure Boot Software Revision Register 2"]
pub type SecBootSwRevReg2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Secure Boot Software Revision Register 2"]
    #[inline(always)]
    pub fn sec_boot_sw_rev_reg2(&self) -> SecBootSwRevReg2R {
        SecBootSwRevReg2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Secure Boot Software Revision Register 2"]
    #[inline(always)]
    pub fn sec_boot_sw_rev_reg2(&mut self) -> SecBootSwRevReg2W<Secure06cSpec> {
        SecBootSwRevReg2W::new(self, 0)
    }
}
#[doc = "Secure Boot Software Revision Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`secure06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure06cSpec;
impl crate::RegisterSpec for Secure06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure06c::R`](R) reader structure"]
impl crate::Readable for Secure06cSpec {}
#[doc = "`write(|w| ..)` method takes [`secure06c::W`](W) writer structure"]
impl crate::Writable for Secure06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE06C to value 0"]
impl crate::Resettable for Secure06cSpec {}
