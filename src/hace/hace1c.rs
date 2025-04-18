#[doc = "Register `HACE1C` reader"]
pub type R = crate::R<Hace1cSpec>;
#[doc = "Register `HACE1C` writer"]
pub type W = crate::W<Hace1cSpec>;
#[doc = "Field `HashEngStsFlag` reader - Hash engine status flag"]
pub type HashEngStsFlagR = crate::BitReader;
#[doc = "Field `CryptoEngStsFlag` reader - Crypto engine status flag"]
pub type CryptoEngStsFlagR = crate::BitReader;
#[doc = "Field `Reserved04` reader - Reserved (0)"]
pub type Reserved04R = crate::BitReader;
#[doc = "Field `CmdQueueStsFlag` reader - Command queue status flag"]
pub type CmdQueueStsFlagR = crate::BitReader;
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::FieldReader;
#[doc = "Field `HashINTFlag` reader - Hash interrupt flag"]
pub type HashIntflagR = crate::BitReader;
#[doc = "Field `HashINTFlag` writer - Hash interrupt flag"]
pub type HashIntflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `CryptoINTFlag` reader - Crypto interrupt flag"]
pub type CryptoIntflagR = crate::BitReader;
#[doc = "Field `CryptoINTFlag` writer - Crypto interrupt flag"]
pub type CryptoIntflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `WrSwTagINTFlag` reader - Write software tag interrupt flag"]
pub type WrSwTagIntflagR = crate::BitReader;
#[doc = "Field `WrSwTagINTFlag` writer - Write software tag interrupt flag"]
pub type WrSwTagIntflagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `HashInputDataSgListTotalLenMismatchRegdebug` reader - Hash input data scatter and gather list total length mismatch regdebug"]
pub type HashInputDataSgListTotalLenMismatchRegdebugR = crate::BitReader;
#[doc = "Field `CryptoInputDataSgListTotalLenMismatchRegdebug` reader - Crypto input data scatter and gather list total length mismatch regdebug"]
pub type CryptoInputDataSgListTotalLenMismatchRegdebugR = crate::BitReader;
#[doc = "Field `CryptoOutputDataSgListTotalLenMismatchRegdebug` reader - Crypto output data scatter and gather list total length mismatch regdebug"]
pub type CryptoOutputDataSgListTotalLenMismatchRegdebugR = crate::BitReader;
#[doc = "Field `HashInputRorateDataOvflRegdebug` reader - Hash input rorate data overflow regdebug"]
pub type HashInputRorateDataOvflRegdebugR = crate::BitReader;
#[doc = "Field `HashInputDataBufOvflRegdebug` reader - Hash input data buffer overflow regdebug"]
pub type HashInputDataBufOvflRegdebugR = crate::BitReader;
#[doc = "Field `CryptoInputRorateDataOvflRegdebug` reader - Crypto input rorate data overflow regdebug"]
pub type CryptoInputRorateDataOvflRegdebugR = crate::BitReader;
#[doc = "Field `CryptoInputDataBufOvflRegdebug` reader - Crypto input data buffer overflow regdebug"]
pub type CryptoInputDataBufOvflRegdebugR = crate::BitReader;
#[doc = "Field `CQHighDataBufOvflRegdebug` reader - CQ high data buffer overflow regdebug"]
pub type CqhighDataBufOvflRegdebugR = crate::BitReader;
#[doc = "Field `CQLowDataBufOvflRegdebug` reader - CQ low data buffer overflow regdebug"]
pub type CqlowDataBufOvflRegdebugR = crate::BitReader;
#[doc = "Field `HashInputDataSgListBufOvflRegdebug` reader - Hash input data scatter and gather list buffer overflow regdebug"]
pub type HashInputDataSgListBufOvflRegdebugR = crate::BitReader;
#[doc = "Field `CryptoInputDataSgListBufOvflRegdebug` reader - Crypto input data scatter and gather list buffer overflow regdebug"]
pub type CryptoInputDataSgListBufOvflRegdebugR = crate::BitReader;
#[doc = "Field `CryptoOutputDataSgListBufOvflRegdebug` reader - Crypto output data scatter and gather list buffer overflow regdebug"]
pub type CryptoOutputDataSgListBufOvflRegdebugR = crate::BitReader;
#[doc = "Field `CQDataBufFormatErrorRegdebug` reader - CQ data buffer format error regdebug"]
pub type CqdataBufFormatErrorRegdebugR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Hash engine status flag"]
    #[inline(always)]
    pub fn hash_eng_sts_flag(&self) -> HashEngStsFlagR {
        HashEngStsFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Crypto engine status flag"]
    #[inline(always)]
    pub fn crypto_eng_sts_flag(&self) -> CryptoEngStsFlagR {
        CryptoEngStsFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved04(&self) -> Reserved04R {
        Reserved04R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command queue status flag"]
    #[inline(always)]
    pub fn cmd_queue_sts_flag(&self) -> CmdQueueStsFlagR {
        CmdQueueStsFlagR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - Hash interrupt flag"]
    #[inline(always)]
    pub fn hash_intflag(&self) -> HashIntflagR {
        HashIntflagR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Crypto interrupt flag"]
    #[inline(always)]
    pub fn crypto_intflag(&self) -> CryptoIntflagR {
        CryptoIntflagR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Write software tag interrupt flag"]
    #[inline(always)]
    pub fn wr_sw_tag_intflag(&self) -> WrSwTagIntflagR {
        WrSwTagIntflagR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Hash input data scatter and gather list total length mismatch regdebug"]
    #[inline(always)]
    pub fn hash_input_data_sg_list_total_len_mismatch_regdebug(
        &self,
    ) -> HashInputDataSgListTotalLenMismatchRegdebugR {
        HashInputDataSgListTotalLenMismatchRegdebugR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Crypto input data scatter and gather list total length mismatch regdebug"]
    #[inline(always)]
    pub fn crypto_input_data_sg_list_total_len_mismatch_regdebug(
        &self,
    ) -> CryptoInputDataSgListTotalLenMismatchRegdebugR {
        CryptoInputDataSgListTotalLenMismatchRegdebugR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Crypto output data scatter and gather list total length mismatch regdebug"]
    #[inline(always)]
    pub fn crypto_output_data_sg_list_total_len_mismatch_regdebug(
        &self,
    ) -> CryptoOutputDataSgListTotalLenMismatchRegdebugR {
        CryptoOutputDataSgListTotalLenMismatchRegdebugR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Hash input rorate data overflow regdebug"]
    #[inline(always)]
    pub fn hash_input_rorate_data_ovfl_regdebug(&self) -> HashInputRorateDataOvflRegdebugR {
        HashInputRorateDataOvflRegdebugR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Hash input data buffer overflow regdebug"]
    #[inline(always)]
    pub fn hash_input_data_buf_ovfl_regdebug(&self) -> HashInputDataBufOvflRegdebugR {
        HashInputDataBufOvflRegdebugR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Crypto input rorate data overflow regdebug"]
    #[inline(always)]
    pub fn crypto_input_rorate_data_ovfl_regdebug(&self) -> CryptoInputRorateDataOvflRegdebugR {
        CryptoInputRorateDataOvflRegdebugR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Crypto input data buffer overflow regdebug"]
    #[inline(always)]
    pub fn crypto_input_data_buf_ovfl_regdebug(&self) -> CryptoInputDataBufOvflRegdebugR {
        CryptoInputDataBufOvflRegdebugR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CQ high data buffer overflow regdebug"]
    #[inline(always)]
    pub fn cqhigh_data_buf_ovfl_regdebug(&self) -> CqhighDataBufOvflRegdebugR {
        CqhighDataBufOvflRegdebugR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CQ low data buffer overflow regdebug"]
    #[inline(always)]
    pub fn cqlow_data_buf_ovfl_regdebug(&self) -> CqlowDataBufOvflRegdebugR {
        CqlowDataBufOvflRegdebugR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Hash input data scatter and gather list buffer overflow regdebug"]
    #[inline(always)]
    pub fn hash_input_data_sg_list_buf_ovfl_regdebug(&self) -> HashInputDataSgListBufOvflRegdebugR {
        HashInputDataSgListBufOvflRegdebugR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Crypto input data scatter and gather list buffer overflow regdebug"]
    #[inline(always)]
    pub fn crypto_input_data_sg_list_buf_ovfl_regdebug(
        &self,
    ) -> CryptoInputDataSgListBufOvflRegdebugR {
        CryptoInputDataSgListBufOvflRegdebugR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Crypto output data scatter and gather list buffer overflow regdebug"]
    #[inline(always)]
    pub fn crypto_output_data_sg_list_buf_ovfl_regdebug(
        &self,
    ) -> CryptoOutputDataSgListBufOvflRegdebugR {
        CryptoOutputDataSgListBufOvflRegdebugR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CQ data buffer format error regdebug"]
    #[inline(always)]
    pub fn cqdata_buf_format_error_regdebug(&self) -> CqdataBufFormatErrorRegdebugR {
        CqdataBufFormatErrorRegdebugR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Hash interrupt flag"]
    #[inline(always)]
    pub fn hash_intflag(&mut self) -> HashIntflagW<Hace1cSpec> {
        HashIntflagW::new(self, 9)
    }
    #[doc = "Bit 12 - Crypto interrupt flag"]
    #[inline(always)]
    pub fn crypto_intflag(&mut self) -> CryptoIntflagW<Hace1cSpec> {
        CryptoIntflagW::new(self, 12)
    }
    #[doc = "Bit 15 - Write software tag interrupt flag"]
    #[inline(always)]
    pub fn wr_sw_tag_intflag(&mut self) -> WrSwTagIntflagW<Hace1cSpec> {
        WrSwTagIntflagW::new(self, 15)
    }
}
#[doc = "HAC Engine Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace1cSpec;
impl crate::RegisterSpec for Hace1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace1c::R`](R) reader structure"]
impl crate::Readable for Hace1cSpec {}
#[doc = "`write(|w| ..)` method takes [`hace1c::W`](W) writer structure"]
impl crate::Writable for Hace1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE1C to value 0"]
impl crate::Resettable for Hace1cSpec {}
