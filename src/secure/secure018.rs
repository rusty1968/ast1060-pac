#[doc = "Register `SECURE018` reader"]
pub type R = crate::R<Secure018Spec>;
#[doc = "Register `SECURE018` writer"]
pub type W = crate::W<Secure018Spec>;
#[doc = "OTP Program Protect (same as SEC10\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpprogramProtectSameAsSec108 {
    #[doc = "0: SEC10 OTP address is not protected"]
    Sec10OtpAddressIsNotProtected = 0,
    #[doc = "1: SEC10 OTP address is write protected"]
    Sec10OtpAddressIsWriteProtected = 1,
}
impl From<OtpprogramProtectSameAsSec108> for bool {
    #[inline(always)]
    fn from(variant: OtpprogramProtectSameAsSec108) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPProgramProtectSameAsSEC108` reader - OTP Program Protect (same as SEC10\\[8\\]"]
pub type OtpprogramProtectSameAsSec108R = crate::BitReader<OtpprogramProtectSameAsSec108>;
impl OtpprogramProtectSameAsSec108R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpprogramProtectSameAsSec108 {
        match self.bits {
            false => OtpprogramProtectSameAsSec108::Sec10OtpAddressIsNotProtected,
            true => OtpprogramProtectSameAsSec108::Sec10OtpAddressIsWriteProtected,
        }
    }
    #[doc = "SEC10 OTP address is not protected"]
    #[inline(always)]
    pub fn is_sec10_otp_address_is_not_protected(&self) -> bool {
        *self == OtpprogramProtectSameAsSec108::Sec10OtpAddressIsNotProtected
    }
    #[doc = "SEC10 OTP address is write protected"]
    #[inline(always)]
    pub fn is_sec10_otp_address_is_write_protected(&self) -> bool {
        *self == OtpprogramProtectSameAsSec108::Sec10OtpAddressIsWriteProtected
    }
}
#[doc = "OTP Program Protected (same as SEC10\\[9\\])\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpprogramProtectedSameAsSec109 {
    #[doc = "0: Last OTP program command is completed"]
    LastOtpProgramCommandIsCompleted = 0,
    #[doc = "1: Last OTP program command is write protected"]
    LastOtpProgramCommandIsWriteProtected = 1,
}
impl From<OtpprogramProtectedSameAsSec109> for bool {
    #[inline(always)]
    fn from(variant: OtpprogramProtectedSameAsSec109) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPProgramProtectedSameAsSEC109` reader - OTP Program Protected (same as SEC10\\[9\\])"]
pub type OtpprogramProtectedSameAsSec109R = crate::BitReader<OtpprogramProtectedSameAsSec109>;
impl OtpprogramProtectedSameAsSec109R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpprogramProtectedSameAsSec109 {
        match self.bits {
            false => OtpprogramProtectedSameAsSec109::LastOtpProgramCommandIsCompleted,
            true => OtpprogramProtectedSameAsSec109::LastOtpProgramCommandIsWriteProtected,
        }
    }
    #[doc = "Last OTP program command is completed"]
    #[inline(always)]
    pub fn is_last_otp_program_command_is_completed(&self) -> bool {
        *self == OtpprogramProtectedSameAsSec109::LastOtpProgramCommandIsCompleted
    }
    #[doc = "Last OTP program command is write protected"]
    #[inline(always)]
    pub fn is_last_otp_program_command_is_write_protected(&self) -> bool {
        *self == OtpprogramProtectedSameAsSec109::LastOtpProgramCommandIsWriteProtected
    }
}
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - OTP Program Protect (same as SEC10\\[8\\]"]
    #[inline(always)]
    pub fn otpprogram_protect_same_as_sec108(&self) -> OtpprogramProtectSameAsSec108R {
        OtpprogramProtectSameAsSec108R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OTP Program Protected (same as SEC10\\[9\\])"]
    #[inline(always)]
    pub fn otpprogram_protected_same_as_sec109(&self) -> OtpprogramProtectedSameAsSec109R {
        OtpprogramProtectedSameAsSec109R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {}
#[doc = "OTP Programming Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure018Spec;
impl crate::RegisterSpec for Secure018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure018::R`](R) reader structure"]
impl crate::Readable for Secure018Spec {}
#[doc = "`write(|w| ..)` method takes [`secure018::W`](W) writer structure"]
impl crate::Writable for Secure018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE018 to value 0"]
impl crate::Resettable for Secure018Spec {}
