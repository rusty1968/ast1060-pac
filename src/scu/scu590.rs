#[doc = "Register `SCU590` reader"]
pub type R = crate::R<Scu590Spec>;
#[doc = "Register `SCU590` writer"]
pub type W = crate::W<Scu590Spec>;
#[doc = "Field `EFUSECtrls1` reader - EFUSE Controls"]
pub type Efusectrls1R = crate::FieldReader<u32>;
#[doc = "Field `EFUSECtrls1` writer - EFUSE Controls"]
pub type Efusectrls1W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `EFUSECtrls` reader - EFUSE Controls"]
pub type EfusectrlsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - EFUSE Controls"]
    #[inline(always)]
    pub fn efusectrls1(&self) -> Efusectrls1R {
        Efusectrls1R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:29 - EFUSE Controls"]
    #[inline(always)]
    pub fn efusectrls(&self) -> EfusectrlsR {
        EfusectrlsR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - EFUSE Controls"]
    #[inline(always)]
    pub fn efusectrls1(&mut self) -> Efusectrls1W<Scu590Spec> {
        Efusectrls1W::new(self, 0)
    }
}
#[doc = "EFUSE Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scu590::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu590::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu590Spec;
impl crate::RegisterSpec for Scu590Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu590::R`](R) reader structure"]
impl crate::Readable for Scu590Spec {}
#[doc = "`write(|w| ..)` method takes [`scu590::W`](W) writer structure"]
impl crate::Writable for Scu590Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU590 to value 0x000f_f3fa"]
impl crate::Resettable for Scu590Spec {
    const RESET_VALUE: u32 = 0x000f_f3fa;
}
