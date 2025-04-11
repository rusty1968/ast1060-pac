#[doc = "Register `SPI010` reader"]
pub type R = crate::R<Spi010Spec>;
#[doc = "Register `SPI010` writer"]
pub type W = crate::W<Spi010Spec>;
#[doc = "SPICMDMODECommand Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpicmdmodecmdMode {
    #[doc = "0: Auto-Read (0x03/0x13 + Address + Read data \\[1/2/3/4 bytes\\])"]
    AutoRead0x030x13_Address_ReadData1234Bytes = 0,
    #[doc = "1: Normal-Read (CMD + Address + Read data \\[1/2/3/4 bytes\\])"]
    NormalReadCmd_Address_ReadData1234Bytes = 1,
    #[doc = "2: Normal-Write (CMD + Address + Write data \\[1/2/3/4 bytes\\])"]
    NormalWriteCmd_Address_WriteData1234Bytes = 2,
    #[doc = "3: User-Mode (Read/write data \\[1/2/3/4 bytes\\])"]
    UserModeReadwriteData1234Bytes = 3,
}
impl From<SpicmdmodecmdMode> for u8 {
    #[inline(always)]
    fn from(variant: SpicmdmodecmdMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpicmdmodecmdMode {
    type Ux = u8;
}
impl crate::IsEnum for SpicmdmodecmdMode {}
#[doc = "Field `SPICMDMODECmdMode` reader - SPICMDMODECommand Mode"]
pub type SpicmdmodecmdModeR = crate::FieldReader<SpicmdmodecmdMode>;
impl SpicmdmodecmdModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpicmdmodecmdMode {
        match self.bits {
            0 => SpicmdmodecmdMode::AutoRead0x030x13_Address_ReadData1234Bytes,
            1 => SpicmdmodecmdMode::NormalReadCmd_Address_ReadData1234Bytes,
            2 => SpicmdmodecmdMode::NormalWriteCmd_Address_WriteData1234Bytes,
            3 => SpicmdmodecmdMode::UserModeReadwriteData1234Bytes,
            _ => unreachable!(),
        }
    }
    #[doc = "Auto-Read (0x03/0x13 + Address + Read data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn is_auto_read_0x030x13__address__read_data_1234_bytes(&self) -> bool {
        *self == SpicmdmodecmdMode::AutoRead0x030x13_Address_ReadData1234Bytes
    }
    #[doc = "Normal-Read (CMD + Address + Read data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn is_normal_read_cmd__address__read_data_1234_bytes(&self) -> bool {
        *self == SpicmdmodecmdMode::NormalReadCmd_Address_ReadData1234Bytes
    }
    #[doc = "Normal-Write (CMD + Address + Write data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn is_normal_write_cmd__address__write_data_1234_bytes(&self) -> bool {
        *self == SpicmdmodecmdMode::NormalWriteCmd_Address_WriteData1234Bytes
    }
    #[doc = "User-Mode (Read/write data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn is_user_mode_readwrite_data_1234_bytes(&self) -> bool {
        *self == SpicmdmodecmdMode::UserModeReadwriteData1234Bytes
    }
}
#[doc = "Field `SPICMDMODECmdMode` writer - SPICMDMODECommand Mode"]
pub type SpicmdmodecmdModeW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SpicmdmodecmdMode, crate::Safe>;
impl<'a, REG> SpicmdmodecmdModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto-Read (0x03/0x13 + Address + Read data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn auto_read_0x030x13__address__read_data_1234_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SpicmdmodecmdMode::AutoRead0x030x13_Address_ReadData1234Bytes)
    }
    #[doc = "Normal-Read (CMD + Address + Read data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn normal_read_cmd__address__read_data_1234_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SpicmdmodecmdMode::NormalReadCmd_Address_ReadData1234Bytes)
    }
    #[doc = "Normal-Write (CMD + Address + Write data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn normal_write_cmd__address__write_data_1234_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SpicmdmodecmdMode::NormalWriteCmd_Address_WriteData1234Bytes)
    }
    #[doc = "User-Mode (Read/write data \\[1/2/3/4 bytes\\])"]
    #[inline(always)]
    pub fn user_mode_readwrite_data_1234_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(SpicmdmodecmdMode::UserModeReadwriteData1234Bytes)
    }
}
#[doc = "Field `CEStopActiveCtrl` reader - CE# Stop Active Control"]
pub type CestopActiveCtrlR = crate::BitReader;
#[doc = "Field `CEStopActiveCtrl` writer - CE# Stop Active Control"]
pub type CestopActiveCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Reserved Enable dual data input mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReservedEnblDualDataInputMode {
    #[doc = "0: 28] = \"010\" mode."]
    _28_010Mode = 0,
}
impl From<ReservedEnblDualDataInputMode> for bool {
    #[inline(always)]
    fn from(variant: ReservedEnblDualDataInputMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ReservedEnblDualDataInputMode` reader - Reserved Enable dual data input mode"]
pub type ReservedEnblDualDataInputModeR = crate::BitReader<ReservedEnblDualDataInputMode>;
impl ReservedEnblDualDataInputModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ReservedEnblDualDataInputMode> {
        match self.bits {
            false => Some(ReservedEnblDualDataInputMode::_28_010Mode),
            _ => None,
        }
    }
    #[doc = "28] = \"010\" mode."]
    #[inline(always)]
    pub fn is_28__010_mode(&self) -> bool {
        *self == ReservedEnblDualDataInputMode::_28_010Mode
    }
}
#[doc = "Field `ReservedEnblDualDataInputMode` writer - Reserved Enable dual data input mode"]
pub type ReservedEnblDualDataInputModeW<'a, REG> =
    crate::BitWriter<'a, REG, ReservedEnblDualDataInputMode>;
impl<'a, REG> ReservedEnblDualDataInputModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "28] = \"010\" mode."]
    #[inline(always)]
    pub fn _28__010_mode(self) -> &'a mut crate::W<REG> {
        self.variant(ReservedEnblDualDataInputMode::_28_010Mode)
    }
}
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "MSB/LSB first control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsblsbfirstCtrl {
    #[doc = "0: MSB First \\htextA{(default for boot code)"]
    MsbFirstHtextAdefaultForBootCode = 0,
    #[doc = "1: LSB First"]
    LsbFirst = 1,
}
impl From<MsblsbfirstCtrl> for bool {
    #[inline(always)]
    fn from(variant: MsblsbfirstCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBLSBFirstCtrl` reader - MSB/LSB first control"]
pub type MsblsbfirstCtrlR = crate::BitReader<MsblsbfirstCtrl>;
impl MsblsbfirstCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsblsbfirstCtrl {
        match self.bits {
            false => MsblsbfirstCtrl::MsbFirstHtextAdefaultForBootCode,
            true => MsblsbfirstCtrl::LsbFirst,
        }
    }
    #[doc = "MSB First \\htextA{(default for boot code)"]
    #[inline(always)]
    pub fn is_msb_first_htext_adefault_for_boot_code(&self) -> bool {
        *self == MsblsbfirstCtrl::MsbFirstHtextAdefaultForBootCode
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == MsblsbfirstCtrl::LsbFirst
    }
}
#[doc = "Field `MSBLSBFirstCtrl` writer - MSB/LSB first control"]
pub type MsblsbfirstCtrlW<'a, REG> = crate::BitWriter<'a, REG, MsblsbfirstCtrl>;
impl<'a, REG> MsblsbfirstCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB First \\htextA{(default for boot code)"]
    #[inline(always)]
    pub fn msb_first_htext_adefault_for_boot_code(self) -> &'a mut crate::W<REG> {
        self.variant(MsblsbfirstCtrl::MsbFirstHtextAdefaultForBootCode)
    }
    #[doc = "LSB First"]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut crate::W<REG> {
        self.variant(MsblsbfirstCtrl::LsbFirst)
    }
}
#[doc = "Field `DummyCyclesBeforeDataForNormalReadCmdLowBits` reader - Dummy cycles before data for Normal-Read command (low bits)"]
pub type DummyCyclesBeforeDataForNormalReadCmdLowBitsR = crate::FieldReader;
#[doc = "Field `DummyCyclesBeforeDataForNormalReadCmdLowBits` writer - Dummy cycles before data for Normal-Read command (low bits)"]
pub type DummyCyclesBeforeDataForNormalReadCmdLowBitsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "SPI clock frequency selection (t-CK)\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpiclkFrequencySelTck {
    #[doc = "0: baseclk + (16 * t-HCLK)"]
    Baseclk_16_Thclk = 0,
    #[doc = "1: baseclk + (14 * t-HCLK)"]
    Baseclk_14_Thclk = 1,
    #[doc = "2: baseclk + (12 * t-HCLK)"]
    Baseclk_12_Thclk = 2,
    #[doc = "3: baseclk + (10 * t-HCLK)"]
    Baseclk_10_Thclk = 3,
    #[doc = "4: baseclk + (~8 * t-HCLK)\\regdefmark"]
    Baseclk_8_Thclkregdefmark = 4,
    #[doc = "5: baseclk + (~6 * t-HCLK)"]
    Baseclk_6_Thclk = 5,
    #[doc = "6: baseclk + (~4 * t-HCLK)"]
    Baseclk_4_Thclk = 6,
    #[doc = "7: baseclk + (~2 * t-HCLK)"]
    Baseclk_2_Thclk = 7,
    #[doc = "8: baseclk + (15 * t-HCLK)"]
    Baseclk_15_Thclk = 8,
    #[doc = "9: baseclk + (13 * t-HCLK)"]
    Baseclk_13_Thclk = 9,
    #[doc = "10: baseclk + (11 * t-HCLK)"]
    Baseclk_11_Thclk = 10,
    #[doc = "11: baseclk + (~9 * t-HCLK)"]
    Baseclk_9_Thclk = 11,
    #[doc = "12: baseclk + (~7 * t-HCLK)"]
    Baseclk_7_Thclk = 12,
    #[doc = "13: baseclk + (~5 * t-HCLK)"]
    Baseclk_5_Thclk = 13,
    #[doc = "14: baseclk + (~3 * t-HCLK)"]
    Baseclk_3_Thclk = 14,
    #[doc = "15: baseclk + (~1 * t-HCLK) (only valid for baseclk selection not equal to 0)"]
    Baseclk_1_ThclkOnlyValidForBaseclkSelectionNotEqualTo0 = 15,
}
impl From<SpiclkFrequencySelTck> for u8 {
    #[inline(always)]
    fn from(variant: SpiclkFrequencySelTck) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpiclkFrequencySelTck {
    type Ux = u8;
}
impl crate::IsEnum for SpiclkFrequencySelTck {}
#[doc = "Field `SPIClkFrequencySelTCK` reader - SPI clock frequency selection (t-CK)"]
pub type SpiclkFrequencySelTckR = crate::FieldReader<SpiclkFrequencySelTck>;
impl SpiclkFrequencySelTckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpiclkFrequencySelTck {
        match self.bits {
            0 => SpiclkFrequencySelTck::Baseclk_16_Thclk,
            1 => SpiclkFrequencySelTck::Baseclk_14_Thclk,
            2 => SpiclkFrequencySelTck::Baseclk_12_Thclk,
            3 => SpiclkFrequencySelTck::Baseclk_10_Thclk,
            4 => SpiclkFrequencySelTck::Baseclk_8_Thclkregdefmark,
            5 => SpiclkFrequencySelTck::Baseclk_6_Thclk,
            6 => SpiclkFrequencySelTck::Baseclk_4_Thclk,
            7 => SpiclkFrequencySelTck::Baseclk_2_Thclk,
            8 => SpiclkFrequencySelTck::Baseclk_15_Thclk,
            9 => SpiclkFrequencySelTck::Baseclk_13_Thclk,
            10 => SpiclkFrequencySelTck::Baseclk_11_Thclk,
            11 => SpiclkFrequencySelTck::Baseclk_9_Thclk,
            12 => SpiclkFrequencySelTck::Baseclk_7_Thclk,
            13 => SpiclkFrequencySelTck::Baseclk_5_Thclk,
            14 => SpiclkFrequencySelTck::Baseclk_3_Thclk,
            15 => SpiclkFrequencySelTck::Baseclk_1_ThclkOnlyValidForBaseclkSelectionNotEqualTo0,
            _ => unreachable!(),
        }
    }
    #[doc = "baseclk + (16 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__16__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_16_Thclk
    }
    #[doc = "baseclk + (14 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__14__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_14_Thclk
    }
    #[doc = "baseclk + (12 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__12__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_12_Thclk
    }
    #[doc = "baseclk + (10 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__10__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_10_Thclk
    }
    #[doc = "baseclk + (~8 * t-HCLK)\\regdefmark"]
    #[inline(always)]
    pub fn is_baseclk__8__thclkregdefmark(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_8_Thclkregdefmark
    }
    #[doc = "baseclk + (~6 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__6__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_6_Thclk
    }
    #[doc = "baseclk + (~4 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__4__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_4_Thclk
    }
    #[doc = "baseclk + (~2 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__2__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_2_Thclk
    }
    #[doc = "baseclk + (15 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__15__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_15_Thclk
    }
    #[doc = "baseclk + (13 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__13__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_13_Thclk
    }
    #[doc = "baseclk + (11 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__11__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_11_Thclk
    }
    #[doc = "baseclk + (~9 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__9__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_9_Thclk
    }
    #[doc = "baseclk + (~7 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__7__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_7_Thclk
    }
    #[doc = "baseclk + (~5 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__5__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_5_Thclk
    }
    #[doc = "baseclk + (~3 * t-HCLK)"]
    #[inline(always)]
    pub fn is_baseclk__3__thclk(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_3_Thclk
    }
    #[doc = "baseclk + (~1 * t-HCLK) (only valid for baseclk selection not equal to 0)"]
    #[inline(always)]
    pub fn is_baseclk__1__thclk_only_valid_for_baseclk_selection_not_equal_to_0(&self) -> bool {
        *self == SpiclkFrequencySelTck::Baseclk_1_ThclkOnlyValidForBaseclkSelectionNotEqualTo0
    }
}
#[doc = "Field `SPIClkFrequencySelTCK` writer - SPI clock frequency selection (t-CK)"]
pub type SpiclkFrequencySelTckW<'a, REG> =
    crate::FieldWriter<'a, REG, 4, SpiclkFrequencySelTck, crate::Safe>;
impl<'a, REG> SpiclkFrequencySelTckW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "baseclk + (16 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__16__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_16_Thclk)
    }
    #[doc = "baseclk + (14 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__14__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_14_Thclk)
    }
    #[doc = "baseclk + (12 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__12__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_12_Thclk)
    }
    #[doc = "baseclk + (10 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__10__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_10_Thclk)
    }
    #[doc = "baseclk + (~8 * t-HCLK)\\regdefmark"]
    #[inline(always)]
    pub fn baseclk__8__thclkregdefmark(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_8_Thclkregdefmark)
    }
    #[doc = "baseclk + (~6 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__6__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_6_Thclk)
    }
    #[doc = "baseclk + (~4 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__4__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_4_Thclk)
    }
    #[doc = "baseclk + (~2 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__2__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_2_Thclk)
    }
    #[doc = "baseclk + (15 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__15__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_15_Thclk)
    }
    #[doc = "baseclk + (13 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__13__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_13_Thclk)
    }
    #[doc = "baseclk + (11 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__11__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_11_Thclk)
    }
    #[doc = "baseclk + (~9 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__9__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_9_Thclk)
    }
    #[doc = "baseclk + (~7 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__7__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_7_Thclk)
    }
    #[doc = "baseclk + (~5 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__5__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_5_Thclk)
    }
    #[doc = "baseclk + (~3 * t-HCLK)"]
    #[inline(always)]
    pub fn baseclk__3__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_3_Thclk)
    }
    #[doc = "baseclk + (~1 * t-HCLK) (only valid for baseclk selection not equal to 0)"]
    #[inline(always)]
    pub fn baseclk__1__thclk_only_valid_for_baseclk_selection_not_equal_to_0(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(SpiclkFrequencySelTck::Baseclk_1_ThclkOnlyValidForBaseclkSelectionNotEqualTo0)
    }
}
#[doc = "Disable SPI flash read/write command merge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisSpiflashReadwrCmdMerge {
    #[doc = "0: Enable"]
    Enable = 0,
    #[doc = "1: Disable (with performance penalty)"]
    DisableWithPerformancePenalty = 1,
}
impl From<DisSpiflashReadwrCmdMerge> for bool {
    #[inline(always)]
    fn from(variant: DisSpiflashReadwrCmdMerge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DisSPIFlashReadwrCmdMerge` reader - Disable SPI flash read/write command merge"]
pub type DisSpiflashReadwrCmdMergeR = crate::BitReader<DisSpiflashReadwrCmdMerge>;
impl DisSpiflashReadwrCmdMergeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisSpiflashReadwrCmdMerge {
        match self.bits {
            false => DisSpiflashReadwrCmdMerge::Enable,
            true => DisSpiflashReadwrCmdMerge::DisableWithPerformancePenalty,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DisSpiflashReadwrCmdMerge::Enable
    }
    #[doc = "Disable (with performance penalty)"]
    #[inline(always)]
    pub fn is_disable_with_performance_penalty(&self) -> bool {
        *self == DisSpiflashReadwrCmdMerge::DisableWithPerformancePenalty
    }
}
#[doc = "Field `DisSPIFlashReadwrCmdMerge` writer - Disable SPI flash read/write command merge"]
pub type DisSpiflashReadwrCmdMergeW<'a, REG> = crate::BitWriter<'a, REG, DisSpiflashReadwrCmdMerge>;
impl<'a, REG> DisSpiflashReadwrCmdMergeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DisSpiflashReadwrCmdMerge::Enable)
    }
    #[doc = "Disable (with performance penalty)"]
    #[inline(always)]
    pub fn disable_with_performance_penalty(self) -> &'a mut crate::W<REG> {
        self.variant(DisSpiflashReadwrCmdMerge::DisableWithPerformancePenalty)
    }
}
#[doc = "Field `DummyCyclesBeforeDataForNormalReadCmdHighBits` reader - Dummy cycles before data for Normal-Read command (high bits)"]
pub type DummyCyclesBeforeDataForNormalReadCmdHighBitsR = crate::BitReader;
#[doc = "Field `DummyCyclesBeforeDataForNormalReadCmdHighBits` writer - Dummy cycles before data for Normal-Read command (high bits)"]
pub type DummyCyclesBeforeDataForNormalReadCmdHighBitsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Dummy cycle command output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DummyCycleCmdOutput {
    #[doc = "0: dummy cycle no command output"]
    DummyCycleNoCommandOutput = 0,
    #[doc = "1: first dummy cycle has command output"]
    FirstDummyCycleHasCommandOutput = 1,
}
impl From<DummyCycleCmdOutput> for bool {
    #[inline(always)]
    fn from(variant: DummyCycleCmdOutput) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DummyCycleCmdOutput` reader - Dummy cycle command output"]
pub type DummyCycleCmdOutputR = crate::BitReader<DummyCycleCmdOutput>;
impl DummyCycleCmdOutputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DummyCycleCmdOutput {
        match self.bits {
            false => DummyCycleCmdOutput::DummyCycleNoCommandOutput,
            true => DummyCycleCmdOutput::FirstDummyCycleHasCommandOutput,
        }
    }
    #[doc = "dummy cycle no command output"]
    #[inline(always)]
    pub fn is_dummy_cycle_no_command_output(&self) -> bool {
        *self == DummyCycleCmdOutput::DummyCycleNoCommandOutput
    }
    #[doc = "first dummy cycle has command output"]
    #[inline(always)]
    pub fn is_first_dummy_cycle_has_command_output(&self) -> bool {
        *self == DummyCycleCmdOutput::FirstDummyCycleHasCommandOutput
    }
}
#[doc = "Field `DummyCycleCmdOutput` writer - Dummy cycle command output"]
pub type DummyCycleCmdOutputW<'a, REG> = crate::BitWriter<'a, REG, DummyCycleCmdOutput>;
impl<'a, REG> DummyCycleCmdOutputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "dummy cycle no command output"]
    #[inline(always)]
    pub fn dummy_cycle_no_command_output(self) -> &'a mut crate::W<REG> {
        self.variant(DummyCycleCmdOutput::DummyCycleNoCommandOutput)
    }
    #[doc = "first dummy cycle has command output"]
    #[inline(always)]
    pub fn first_dummy_cycle_has_command_output(self) -> &'a mut crate::W<REG> {
        self.variant(DummyCycleCmdOutput::FirstDummyCycleHasCommandOutput)
    }
}
#[doc = "Field `SPICmd` reader - SPI Command"]
pub type SpicmdR = crate::FieldReader;
#[doc = "Field `SPICmd` writer - SPI Command"]
pub type SpicmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "SPI base clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpibaseClkSel {
    #[doc = "0: baseclk = 0 * t-HCLK"]
    Baseclk_0_Thclk = 0,
    #[doc = "1: baseclk = 16 * t-HCLK"]
    Baseclk_16_Thclk = 1,
    #[doc = "2: baseclk = 32 * t-HCLK"]
    Baseclk_32_Thclk = 2,
    #[doc = "3: baseclk = 48 * t-HCLK"]
    Baseclk_48_Thclk = 3,
    #[doc = "15: baseclk = 240 * t-HCLK"]
    Baseclk_240_Thclk = 15,
}
impl From<SpibaseClkSel> for u8 {
    #[inline(always)]
    fn from(variant: SpibaseClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpibaseClkSel {
    type Ux = u8;
}
impl crate::IsEnum for SpibaseClkSel {}
#[doc = "Field `SPIBaseClkSel` reader - SPI base clock selection"]
pub type SpibaseClkSelR = crate::FieldReader<SpibaseClkSel>;
impl SpibaseClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SpibaseClkSel> {
        match self.bits {
            0 => Some(SpibaseClkSel::Baseclk_0_Thclk),
            1 => Some(SpibaseClkSel::Baseclk_16_Thclk),
            2 => Some(SpibaseClkSel::Baseclk_32_Thclk),
            3 => Some(SpibaseClkSel::Baseclk_48_Thclk),
            15 => Some(SpibaseClkSel::Baseclk_240_Thclk),
            _ => None,
        }
    }
    #[doc = "baseclk = 0 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__0___thclk(&self) -> bool {
        *self == SpibaseClkSel::Baseclk_0_Thclk
    }
    #[doc = "baseclk = 16 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__16__thclk(&self) -> bool {
        *self == SpibaseClkSel::Baseclk_16_Thclk
    }
    #[doc = "baseclk = 32 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__32__thclk(&self) -> bool {
        *self == SpibaseClkSel::Baseclk_32_Thclk
    }
    #[doc = "baseclk = 48 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__48__thclk(&self) -> bool {
        *self == SpibaseClkSel::Baseclk_48_Thclk
    }
    #[doc = "baseclk = 240 * t-HCLK"]
    #[inline(always)]
    pub fn is_baseclk__240__thclk(&self) -> bool {
        *self == SpibaseClkSel::Baseclk_240_Thclk
    }
}
#[doc = "Field `SPIBaseClkSel` writer - SPI base clock selection"]
pub type SpibaseClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 4, SpibaseClkSel>;
impl<'a, REG> SpibaseClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "baseclk = 0 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__0___thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel::Baseclk_0_Thclk)
    }
    #[doc = "baseclk = 16 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__16__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel::Baseclk_16_Thclk)
    }
    #[doc = "baseclk = 32 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__32__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel::Baseclk_32_Thclk)
    }
    #[doc = "baseclk = 48 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__48__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel::Baseclk_48_Thclk)
    }
    #[doc = "baseclk = 240 * t-HCLK"]
    #[inline(always)]
    pub fn baseclk__240__thclk(self) -> &'a mut crate::W<REG> {
        self.variant(SpibaseClkSel::Baseclk_240_Thclk)
    }
}
#[doc = "IO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Iomode {
    #[doc = "0: single bit."]
    SingleBit = 0,
    #[doc = "2: dual bit read/write, data cycle only."]
    DualBitReadwriteDataCycleOnly = 2,
    #[doc = "3: dual bit read/write, including address and dummy byte cycle."]
    DualBitReadwriteIncludingAddressAndDummyByteCycle = 3,
    #[doc = "4: quad bit read/write, data cycle only."]
    QuadBitReadwriteDataCycleOnly = 4,
    #[doc = "5: quad bit read/write, including address and dummy byte cycle."]
    QuadBitReadwriteIncludingAddressAndDummyByteCycle = 5,
    #[doc = "8: QPI mode, quad bit on command/address/data cycles."]
    QpiModeQuadBitOnCommandaddressdataCycles = 8,
}
impl From<Iomode> for u8 {
    #[inline(always)]
    fn from(variant: Iomode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Iomode {
    type Ux = u8;
}
impl crate::IsEnum for Iomode {}
#[doc = "Field `IOMode` reader - IO Mode"]
pub type IomodeR = crate::FieldReader<Iomode>;
impl IomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Iomode> {
        match self.bits {
            0 => Some(Iomode::SingleBit),
            2 => Some(Iomode::DualBitReadwriteDataCycleOnly),
            3 => Some(Iomode::DualBitReadwriteIncludingAddressAndDummyByteCycle),
            4 => Some(Iomode::QuadBitReadwriteDataCycleOnly),
            5 => Some(Iomode::QuadBitReadwriteIncludingAddressAndDummyByteCycle),
            8 => Some(Iomode::QpiModeQuadBitOnCommandaddressdataCycles),
            _ => None,
        }
    }
    #[doc = "single bit."]
    #[inline(always)]
    pub fn is_single_bit(&self) -> bool {
        *self == Iomode::SingleBit
    }
    #[doc = "dual bit read/write, data cycle only."]
    #[inline(always)]
    pub fn is_dual_bit_readwrite_data_cycle_only(&self) -> bool {
        *self == Iomode::DualBitReadwriteDataCycleOnly
    }
    #[doc = "dual bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn is_dual_bit_readwrite_including_address_and_dummy_byte_cycle(&self) -> bool {
        *self == Iomode::DualBitReadwriteIncludingAddressAndDummyByteCycle
    }
    #[doc = "quad bit read/write, data cycle only."]
    #[inline(always)]
    pub fn is_quad_bit_readwrite_data_cycle_only(&self) -> bool {
        *self == Iomode::QuadBitReadwriteDataCycleOnly
    }
    #[doc = "quad bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn is_quad_bit_readwrite_including_address_and_dummy_byte_cycle(&self) -> bool {
        *self == Iomode::QuadBitReadwriteIncludingAddressAndDummyByteCycle
    }
    #[doc = "QPI mode, quad bit on command/address/data cycles."]
    #[inline(always)]
    pub fn is_qpi_mode_quad_bit_on_commandaddressdata_cycles(&self) -> bool {
        *self == Iomode::QpiModeQuadBitOnCommandaddressdataCycles
    }
}
#[doc = "Field `IOMode` writer - IO Mode"]
pub type IomodeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Iomode>;
impl<'a, REG> IomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "single bit."]
    #[inline(always)]
    pub fn single_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::SingleBit)
    }
    #[doc = "dual bit read/write, data cycle only."]
    #[inline(always)]
    pub fn dual_bit_readwrite_data_cycle_only(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::DualBitReadwriteDataCycleOnly)
    }
    #[doc = "dual bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn dual_bit_readwrite_including_address_and_dummy_byte_cycle(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(Iomode::DualBitReadwriteIncludingAddressAndDummyByteCycle)
    }
    #[doc = "quad bit read/write, data cycle only."]
    #[inline(always)]
    pub fn quad_bit_readwrite_data_cycle_only(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::QuadBitReadwriteDataCycleOnly)
    }
    #[doc = "quad bit read/write, including address and dummy byte cycle."]
    #[inline(always)]
    pub fn quad_bit_readwrite_including_address_and_dummy_byte_cycle(
        self,
    ) -> &'a mut crate::W<REG> {
        self.variant(Iomode::QuadBitReadwriteIncludingAddressAndDummyByteCycle)
    }
    #[doc = "QPI mode, quad bit on command/address/data cycles."]
    #[inline(always)]
    pub fn qpi_mode_quad_bit_on_commandaddressdata_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Iomode::QpiModeQuadBitOnCommandaddressdataCycles)
    }
}
impl R {
    #[doc = "Bits 0:1 - SPICMDMODECommand Mode"]
    #[inline(always)]
    pub fn spicmdmodecmd_mode(&self) -> SpicmdmodecmdModeR {
        SpicmdmodecmdModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CE# Stop Active Control"]
    #[inline(always)]
    pub fn cestop_active_ctrl(&self) -> CestopActiveCtrlR {
        CestopActiveCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved Enable dual data input mode"]
    #[inline(always)]
    pub fn reserved_enbl_dual_data_input_mode(&self) -> ReservedEnblDualDataInputModeR {
        ReservedEnblDualDataInputModeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MSB/LSB first control"]
    #[inline(always)]
    pub fn msblsbfirst_ctrl(&self) -> MsblsbfirstCtrlR {
        MsblsbfirstCtrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Dummy cycles before data for Normal-Read command (low bits)"]
    #[inline(always)]
    pub fn dummy_cycles_before_data_for_normal_read_cmd_low_bits(
        &self,
    ) -> DummyCyclesBeforeDataForNormalReadCmdLowBitsR {
        DummyCyclesBeforeDataForNormalReadCmdLowBitsR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - SPI clock frequency selection (t-CK)"]
    #[inline(always)]
    pub fn spiclk_frequency_sel_tck(&self) -> SpiclkFrequencySelTckR {
        SpiclkFrequencySelTckR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Disable SPI flash read/write command merge"]
    #[inline(always)]
    pub fn dis_spiflash_readwr_cmd_merge(&self) -> DisSpiflashReadwrCmdMergeR {
        DisSpiflashReadwrCmdMergeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Dummy cycles before data for Normal-Read command (high bits)"]
    #[inline(always)]
    pub fn dummy_cycles_before_data_for_normal_read_cmd_high_bits(
        &self,
    ) -> DummyCyclesBeforeDataForNormalReadCmdHighBitsR {
        DummyCyclesBeforeDataForNormalReadCmdHighBitsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Dummy cycle command output"]
    #[inline(always)]
    pub fn dummy_cycle_cmd_output(&self) -> DummyCycleCmdOutputR {
        DummyCycleCmdOutputR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - SPI Command"]
    #[inline(always)]
    pub fn spicmd(&self) -> SpicmdR {
        SpicmdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - SPI base clock selection"]
    #[inline(always)]
    pub fn spibase_clk_sel(&self) -> SpibaseClkSelR {
        SpibaseClkSelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - IO Mode"]
    #[inline(always)]
    pub fn iomode(&self) -> IomodeR {
        IomodeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPICMDMODECommand Mode"]
    #[inline(always)]
    pub fn spicmdmodecmd_mode(&mut self) -> SpicmdmodecmdModeW<Spi010Spec> {
        SpicmdmodecmdModeW::new(self, 0)
    }
    #[doc = "Bit 2 - CE# Stop Active Control"]
    #[inline(always)]
    pub fn cestop_active_ctrl(&mut self) -> CestopActiveCtrlW<Spi010Spec> {
        CestopActiveCtrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved Enable dual data input mode"]
    #[inline(always)]
    pub fn reserved_enbl_dual_data_input_mode(
        &mut self,
    ) -> ReservedEnblDualDataInputModeW<Spi010Spec> {
        ReservedEnblDualDataInputModeW::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Spi010Spec> {
        Reserved2W::new(self, 4)
    }
    #[doc = "Bit 5 - MSB/LSB first control"]
    #[inline(always)]
    pub fn msblsbfirst_ctrl(&mut self) -> MsblsbfirstCtrlW<Spi010Spec> {
        MsblsbfirstCtrlW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Dummy cycles before data for Normal-Read command (low bits)"]
    #[inline(always)]
    pub fn dummy_cycles_before_data_for_normal_read_cmd_low_bits(
        &mut self,
    ) -> DummyCyclesBeforeDataForNormalReadCmdLowBitsW<Spi010Spec> {
        DummyCyclesBeforeDataForNormalReadCmdLowBitsW::new(self, 6)
    }
    #[doc = "Bits 8:11 - SPI clock frequency selection (t-CK)"]
    #[inline(always)]
    pub fn spiclk_frequency_sel_tck(&mut self) -> SpiclkFrequencySelTckW<Spi010Spec> {
        SpiclkFrequencySelTckW::new(self, 8)
    }
    #[doc = "Bit 12 - Disable SPI flash read/write command merge"]
    #[inline(always)]
    pub fn dis_spiflash_readwr_cmd_merge(&mut self) -> DisSpiflashReadwrCmdMergeW<Spi010Spec> {
        DisSpiflashReadwrCmdMergeW::new(self, 12)
    }
    #[doc = "Bit 14 - Dummy cycles before data for Normal-Read command (high bits)"]
    #[inline(always)]
    pub fn dummy_cycles_before_data_for_normal_read_cmd_high_bits(
        &mut self,
    ) -> DummyCyclesBeforeDataForNormalReadCmdHighBitsW<Spi010Spec> {
        DummyCyclesBeforeDataForNormalReadCmdHighBitsW::new(self, 14)
    }
    #[doc = "Bit 15 - Dummy cycle command output"]
    #[inline(always)]
    pub fn dummy_cycle_cmd_output(&mut self) -> DummyCycleCmdOutputW<Spi010Spec> {
        DummyCycleCmdOutputW::new(self, 15)
    }
    #[doc = "Bits 16:23 - SPI Command"]
    #[inline(always)]
    pub fn spicmd(&mut self) -> SpicmdW<Spi010Spec> {
        SpicmdW::new(self, 16)
    }
    #[doc = "Bits 24:27 - SPI base clock selection"]
    #[inline(always)]
    pub fn spibase_clk_sel(&mut self) -> SpibaseClkSelW<Spi010Spec> {
        SpibaseClkSelW::new(self, 24)
    }
    #[doc = "Bits 28:31 - IO Mode"]
    #[inline(always)]
    pub fn iomode(&mut self) -> IomodeW<Spi010Spec> {
        IomodeW::new(self, 28)
    }
}
#[doc = "CE0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi010Spec;
impl crate::RegisterSpec for Spi010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi010::R`](R) reader structure"]
impl crate::Readable for Spi010Spec {}
#[doc = "`write(|w| ..)` method takes [`spi010::W`](W) writer structure"]
impl crate::Writable for Spi010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI010 to value 0x0400"]
impl crate::Resettable for Spi010Spec {
    const RESET_VALUE: u32 = 0x0400;
}
