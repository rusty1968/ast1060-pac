#[doc = "Register `SECURE058` reader"]
pub type R = crate::R<Secure058Spec>;
#[doc = "Register `SECURE058` writer"]
pub type W = crate::W<Secure058Spec>;
#[doc = "Field `SecBootEngIntCtrlReg` reader - Secure Boot Engine Internal Controller Register"]
pub type SecBootEngIntCtrlRegR = crate::BitReader;
#[doc = "Field `SecBootEngIntCtrlReg` writer - Secure Boot Engine Internal Controller Register"]
pub type SecBootEngIntCtrlRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WrProtOfThisRegSEC58` reader - Write Protection of this register SEC58"]
pub type WrProtOfThisRegSec58R = crate::BitReader;
#[doc = "Field `WrProtOfThisRegSEC58` writer - Write Protection of this register SEC58"]
pub type WrProtOfThisRegSec58W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Secure Boot Engine Internal Controller Register"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_reg(&self) -> SecBootEngIntCtrlRegR {
        SecBootEngIntCtrlRegR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection of this register SEC58"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec58(&self) -> WrProtOfThisRegSec58R {
        WrProtOfThisRegSec58R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Secure Boot Engine Internal Controller Register"]
    #[inline(always)]
    pub fn sec_boot_eng_int_ctrl_reg(&mut self) -> SecBootEngIntCtrlRegW<Secure058Spec> {
        SecBootEngIntCtrlRegW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protection of this register SEC58"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec58(&mut self) -> WrProtOfThisRegSec58W<Secure058Spec> {
        WrProtOfThisRegSec58W::new(self, 1)
    }
}
#[doc = "Secure Boot Engine Internal Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure058Spec;
impl crate::RegisterSpec for Secure058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure058::R`](R) reader structure"]
impl crate::Readable for Secure058Spec {}
#[doc = "`write(|w| ..)` method takes [`secure058::W`](W) writer structure"]
impl crate::Writable for Secure058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE058 to value 0x03"]
impl crate::Resettable for Secure058Spec {
    const RESET_VALUE: u32 = 0x03;
}
