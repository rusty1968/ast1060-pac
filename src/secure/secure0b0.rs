#[doc = "Register `SECURE0B0` reader"]
pub type R = crate::R<Secure0b0Spec>;
#[doc = "Register `SECURE0B0` writer"]
pub type W = crate::W<Secure0b0Spec>;
#[doc = "Field `SecBootRSAModularBitNumberRegs` reader - Secure Boot RSA Modular Bit Number Registers"]
pub type SecBootRsamodularBitNumberRegsR = crate::FieldReader<u16>;
#[doc = "Field `SecBootRSAModularBitNumberRegs` writer - Secure Boot RSA Modular Bit Number Registers"]
pub type SecBootRsamodularBitNumberRegsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `Reserved01` reader - Reserved(0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `SecBootRSAExponentBitNumberRegs` reader - Secure Boot RSA Exponent Bit Number Registers"]
pub type SecBootRsaexponentBitNumberRegsR = crate::FieldReader<u16>;
#[doc = "Field `SecBootRSAExponentBitNumberRegs` writer - Secure Boot RSA Exponent Bit Number Registers"]
pub type SecBootRsaexponentBitNumberRegsW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:13 - Secure Boot RSA Modular Bit Number Registers"]
    #[inline(always)]
    pub fn sec_boot_rsamodular_bit_number_regs(&self) -> SecBootRsamodularBitNumberRegsR {
        SecBootRsamodularBitNumberRegsR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:29 - Secure Boot RSA Exponent Bit Number Registers"]
    #[inline(always)]
    pub fn sec_boot_rsaexponent_bit_number_regs(&self) -> SecBootRsaexponentBitNumberRegsR {
        SecBootRsaexponentBitNumberRegsR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 30:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Secure Boot RSA Modular Bit Number Registers"]
    #[inline(always)]
    pub fn sec_boot_rsamodular_bit_number_regs(
        &mut self,
    ) -> SecBootRsamodularBitNumberRegsW<Secure0b0Spec> {
        SecBootRsamodularBitNumberRegsW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Secure Boot RSA Exponent Bit Number Registers"]
    #[inline(always)]
    pub fn sec_boot_rsaexponent_bit_number_regs(
        &mut self,
    ) -> SecBootRsaexponentBitNumberRegsW<Secure0b0Spec> {
        SecBootRsaexponentBitNumberRegsW::new(self, 16)
    }
}
#[doc = "Secure Boot RSA Engine Controller Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure0b0Spec;
impl crate::RegisterSpec for Secure0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure0b0::R`](R) reader structure"]
impl crate::Readable for Secure0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`secure0b0::W`](W) writer structure"]
impl crate::Writable for Secure0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE0B0 to value 0"]
impl crate::Resettable for Secure0b0Spec {}
