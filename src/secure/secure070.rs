#[doc = "Register `SECURE070` reader"]
pub type R = crate::R<Secure070Spec>;
#[doc = "Register `SECURE070` writer"]
pub type W = crate::W<Secure070Spec>;
#[doc = "Secure Boot from SPI Status Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecBootFromSpistatusRegs {
    #[doc = "0: Reset initial value and during secure boot period. The value will keep 00 when secure boot is not enabled or Boot from eMMC is enabled."]
    ResetInitialValueAndDuringSecureBootPeriodTheValueWillKeep00WhenSecureBootIsNotEnabledOrBootFromEmmcIsEnabled =
        0,
    #[doc = "1: Secure Boot check failed."]
    SecureBootCheckFailed = 1,
    #[doc = "3: Secure Boot check passed."]
    SecureBootCheckPassed = 3,
}
impl From<SecBootFromSpistatusRegs> for u8 {
    #[inline(always)]
    fn from(variant: SecBootFromSpistatusRegs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecBootFromSpistatusRegs {
    type Ux = u8;
}
impl crate::IsEnum for SecBootFromSpistatusRegs {}
#[doc = "Field `SecBootFromSPIStatusRegs` reader - Secure Boot from SPI Status Registers"]
pub type SecBootFromSpistatusRegsR = crate::FieldReader<SecBootFromSpistatusRegs>;
impl SecBootFromSpistatusRegsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecBootFromSpistatusRegs {
        match self . bits { 0 => SecBootFromSpistatusRegs :: ResetInitialValueAndDuringSecureBootPeriodTheValueWillKeep00WhenSecureBootIsNotEnabledOrBootFromEmmcIsEnabled , 1 => SecBootFromSpistatusRegs :: SecureBootCheckFailed , 3 => SecBootFromSpistatusRegs :: SecureBootCheckPassed , _ => unreachable ! () , }
    }
    #[doc = "Reset initial value and during secure boot period. The value will keep 00 when secure boot is not enabled or Boot from eMMC is enabled."]
    #[inline(always)]
    pub fn is_reset_initial_value_and_during_secure_boot_period_the_value_will_keep_00_when_secure_boot_is_not_enabled_or_boot_from_emmc_is_enabled(
        &self,
    ) -> bool {
        * self == SecBootFromSpistatusRegs :: ResetInitialValueAndDuringSecureBootPeriodTheValueWillKeep00WhenSecureBootIsNotEnabledOrBootFromEmmcIsEnabled
    }
    #[doc = "Secure Boot check failed."]
    #[inline(always)]
    pub fn is_secure_boot_check_failed(&self) -> bool {
        *self == SecBootFromSpistatusRegs::SecureBootCheckFailed
    }
    #[doc = "Secure Boot check passed."]
    #[inline(always)]
    pub fn is_secure_boot_check_passed(&self) -> bool {
        *self == SecBootFromSpistatusRegs::SecureBootCheckPassed
    }
}
#[doc = "Field `SecBootFromSPIStatusRegs` writer - Secure Boot from SPI Status Registers"]
pub type SecBootFromSpistatusRegsW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SecBootFromSpistatusRegs>;
impl<'a, REG> SecBootFromSpistatusRegsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset initial value and during secure boot period. The value will keep 00 when secure boot is not enabled or Boot from eMMC is enabled."]
    #[inline(always)]
    pub fn reset_initial_value_and_during_secure_boot_period_the_value_will_keep_00_when_secure_boot_is_not_enabled_or_boot_from_emmc_is_enabled(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (SecBootFromSpistatusRegs :: ResetInitialValueAndDuringSecureBootPeriodTheValueWillKeep00WhenSecureBootIsNotEnabledOrBootFromEmmcIsEnabled)
    }
    #[doc = "Secure Boot check failed."]
    #[inline(always)]
    pub fn secure_boot_check_failed(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootFromSpistatusRegs::SecureBootCheckFailed)
    }
    #[doc = "Secure Boot check passed."]
    #[inline(always)]
    pub fn secure_boot_check_passed(self) -> &'a mut crate::W<REG> {
        self.variant(SecBootFromSpistatusRegs::SecureBootCheckPassed)
    }
}
#[doc = "Field `WrProtOfThisRegSEC70` reader - Write Protection of this register SEC70"]
pub type WrProtOfThisRegSec70R = crate::BitReader;
#[doc = "Field `WrProtOfThisRegSEC70` writer - Write Protection of this register SEC70"]
pub type WrProtOfThisRegSec70W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - Secure Boot from SPI Status Registers"]
    #[inline(always)]
    pub fn sec_boot_from_spistatus_regs(&self) -> SecBootFromSpistatusRegsR {
        SecBootFromSpistatusRegsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Write Protection of this register SEC70"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec70(&self) -> WrProtOfThisRegSec70R {
        WrProtOfThisRegSec70R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Secure Boot from SPI Status Registers"]
    #[inline(always)]
    pub fn sec_boot_from_spistatus_regs(&mut self) -> SecBootFromSpistatusRegsW<Secure070Spec> {
        SecBootFromSpistatusRegsW::new(self, 0)
    }
    #[doc = "Bit 2 - Write Protection of this register SEC70"]
    #[inline(always)]
    pub fn wr_prot_of_this_reg_sec70(&mut self) -> WrProtOfThisRegSec70W<Secure070Spec> {
        WrProtOfThisRegSec70W::new(self, 2)
    }
}
#[doc = "Secure Boot from SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure070Spec;
impl crate::RegisterSpec for Secure070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure070::R`](R) reader structure"]
impl crate::Readable for Secure070Spec {}
#[doc = "`write(|w| ..)` method takes [`secure070::W`](W) writer structure"]
impl crate::Writable for Secure070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE070 to value 0"]
impl crate::Resettable for Secure070Spec {}
