#[doc = "Register `I2CM18` reader"]
pub type R = crate::R<I2cm18Spec>;
#[doc = "Register `I2CM18` writer"]
pub type W = crate::W<I2cm18Spec>;
#[doc = "Field `MasterStartCmdSSr` reader - Master Start Command (S/Sr)"]
pub type MasterStartCmdSsrR = crate::BitReader;
#[doc = "Field `MasterStartCmdSSr` writer - Master Start Command (S/Sr)"]
pub type MasterStartCmdSsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MasterTxCmdTxD` reader - Master Transmit Command (TxD)"]
pub type MasterTxCmdTxDR = crate::BitReader;
#[doc = "Field `MasterTxCmdTxD` writer - Master Transmit Command (TxD)"]
pub type MasterTxCmdTxDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "Field `MasterRxCmdRxD` reader - Master Receive Command (RxD)"]
pub type MasterRxCmdRxDR = crate::BitReader;
#[doc = "Field `MasterRxCmdRxD` writer - Master Receive Command (RxD)"]
pub type MasterRxCmdRxDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MasterRxCmdLast` reader - Master Receive Command Last"]
pub type MasterRxCmdLastR = crate::BitReader;
#[doc = "Field `MasterRxCmdLast` writer - Master Receive Command Last"]
pub type MasterRxCmdLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MasterStopCmdP` reader - Master Stop Command (P)"]
pub type MasterStopCmdPR = crate::BitReader;
#[doc = "Field `MasterStopCmdP` writer - Master Stop Command (P)"]
pub type MasterStopCmdPW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblMasterTxPoolDataBuf` reader - Enable Master Tx Pool Data buffer"]
pub type EnblMasterTxPoolDataBufR = crate::BitReader;
#[doc = "Field `EnblMasterTxPoolDataBuf` writer - Enable Master Tx Pool Data buffer"]
pub type EnblMasterTxPoolDataBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblMasterRxPoolDataBuf` reader - Enable Master Rx Pool Data buffer"]
pub type EnblMasterRxPoolDataBufR = crate::BitReader;
#[doc = "Field `EnblMasterRxPoolDataBuf` writer - Enable Master Rx Pool Data buffer"]
pub type EnblMasterRxPoolDataBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblMasterTxDMADataBuf` reader - Enable Master Tx DMA Data buffer"]
pub type EnblMasterTxDmadataBufR = crate::BitReader;
#[doc = "Field `EnblMasterTxDMADataBuf` writer - Enable Master Tx DMA Data buffer"]
pub type EnblMasterTxDmadataBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblMasterRxDMADataBuf` reader - Enable Master Rx DMA Data buffer"]
pub type EnblMasterRxDmadataBufR = crate::BitReader;
#[doc = "Field `EnblMasterRxDMADataBuf` writer - Enable Master Rx DMA Data buffer"]
pub type EnblMasterRxDmadataBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `EnblBusRecoverCmd` reader - Enable Bus Recover Command"]
pub type EnblBusRecoverCmdR = crate::BitReader;
#[doc = "Field `EnblBusRecoverCmd` writer - Enable Bus Recover Command"]
pub type EnblBusRecoverCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLOOutputDirCtrl` reader - SCL_O output direct control"]
pub type SclooutputDirCtrlR = crate::BitReader;
#[doc = "Field `SCLOOutputDirCtrl` writer - SCL_O output direct control"]
pub type SclooutputDirCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLOEOutputDirCtrl` reader - SCL_OE output direct control"]
pub type ScloeoutputDirCtrlR = crate::BitReader;
#[doc = "Field `SCLOEOutputDirCtrl` writer - SCL_OE output direct control"]
pub type ScloeoutputDirCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAOOutputDirCtrl` reader - SDA_O output direct control"]
pub type SdaooutputDirCtrlR = crate::BitReader;
#[doc = "Field `SDAOOutputDirCtrl` writer - SDA_O output direct control"]
pub type SdaooutputDirCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAOEOutputDirCtrl` reader - SDA_OE output direct control"]
pub type SdaoeoutputDirCtrlR = crate::BitReader;
#[doc = "Field `SDAOEOutputDirCtrl` writer - SDA_OE output direct control"]
pub type SdaoeoutputDirCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblMasterPktOpMode` reader - Enable master Packet operation mode"]
pub type EnblMasterPktOpModeR = crate::BitReader;
#[doc = "Field `EnblMasterPktOpMode` writer - Enable master Packet operation mode"]
pub type EnblMasterPktOpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HighSpeedModeMasterCodeLSB` reader - High Speed mode master code LSB"]
pub type HighSpeedModeMasterCodeLsbR = crate::FieldReader;
#[doc = "Field `HighSpeedModeMasterCodeLSB` writer - High Speed mode master code LSB"]
pub type HighSpeedModeMasterCodeLsbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `TargetDevAddr` reader - Target device address"]
pub type TargetDevAddrR = crate::FieldReader;
#[doc = "Field `TargetDevAddr` writer - Target device address"]
pub type TargetDevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Wr1CtrlForI2CM18ForCurWrCmd` reader - Write '1' control for I2CM18 for current write command"]
pub type Wr1ctrlForI2cm18forCurWrCmdR = crate::BitReader;
#[doc = "Field `Wr1CtrlForI2CM18ForCurWrCmd` writer - Write '1' control for I2CM18 for current write command"]
pub type Wr1ctrlForI2cm18forCurWrCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Master Start Command (S/Sr)"]
    #[inline(always)]
    pub fn master_start_cmd_ssr(&self) -> MasterStartCmdSsrR {
        MasterStartCmdSsrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit Command (TxD)"]
    #[inline(always)]
    pub fn master_tx_cmd_tx_d(&self) -> MasterTxCmdTxDR {
        MasterTxCmdTxDR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Receive Command (RxD)"]
    #[inline(always)]
    pub fn master_rx_cmd_rx_d(&self) -> MasterRxCmdRxDR {
        MasterRxCmdRxDR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Receive Command Last"]
    #[inline(always)]
    pub fn master_rx_cmd_last(&self) -> MasterRxCmdLastR {
        MasterRxCmdLastR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master Stop Command (P)"]
    #[inline(always)]
    pub fn master_stop_cmd_p(&self) -> MasterStopCmdPR {
        MasterStopCmdPR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Master Tx Pool Data buffer"]
    #[inline(always)]
    pub fn enbl_master_tx_pool_data_buf(&self) -> EnblMasterTxPoolDataBufR {
        EnblMasterTxPoolDataBufR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Master Rx Pool Data buffer"]
    #[inline(always)]
    pub fn enbl_master_rx_pool_data_buf(&self) -> EnblMasterRxPoolDataBufR {
        EnblMasterRxPoolDataBufR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Master Tx DMA Data buffer"]
    #[inline(always)]
    pub fn enbl_master_tx_dmadata_buf(&self) -> EnblMasterTxDmadataBufR {
        EnblMasterTxDmadataBufR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Master Rx DMA Data buffer"]
    #[inline(always)]
    pub fn enbl_master_rx_dmadata_buf(&self) -> EnblMasterRxDmadataBufR {
        EnblMasterRxDmadataBufR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Bus Recover Command"]
    #[inline(always)]
    pub fn enbl_bus_recover_cmd(&self) -> EnblBusRecoverCmdR {
        EnblBusRecoverCmdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SCL_O output direct control"]
    #[inline(always)]
    pub fn sclooutput_dir_ctrl(&self) -> SclooutputDirCtrlR {
        SclooutputDirCtrlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCL_OE output direct control"]
    #[inline(always)]
    pub fn scloeoutput_dir_ctrl(&self) -> ScloeoutputDirCtrlR {
        ScloeoutputDirCtrlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SDA_O output direct control"]
    #[inline(always)]
    pub fn sdaooutput_dir_ctrl(&self) -> SdaooutputDirCtrlR {
        SdaooutputDirCtrlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDA_OE output direct control"]
    #[inline(always)]
    pub fn sdaoeoutput_dir_ctrl(&self) -> SdaoeoutputDirCtrlR {
        SdaoeoutputDirCtrlR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable master Packet operation mode"]
    #[inline(always)]
    pub fn enbl_master_pkt_op_mode(&self) -> EnblMasterPktOpModeR {
        EnblMasterPktOpModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - High Speed mode master code LSB"]
    #[inline(always)]
    pub fn high_speed_mode_master_code_lsb(&self) -> HighSpeedModeMasterCodeLsbR {
        HighSpeedModeMasterCodeLsbR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - Target device address"]
    #[inline(always)]
    pub fn target_dev_addr(&self) -> TargetDevAddrR {
        TargetDevAddrR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Write '1' control for I2CM18 for current write command"]
    #[inline(always)]
    pub fn wr1ctrl_for_i2cm18for_cur_wr_cmd(&self) -> Wr1ctrlForI2cm18forCurWrCmdR {
        Wr1ctrlForI2cm18forCurWrCmdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Start Command (S/Sr)"]
    #[inline(always)]
    pub fn master_start_cmd_ssr(&mut self) -> MasterStartCmdSsrW<I2cm18Spec> {
        MasterStartCmdSsrW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit Command (TxD)"]
    #[inline(always)]
    pub fn master_tx_cmd_tx_d(&mut self) -> MasterTxCmdTxDW<I2cm18Spec> {
        MasterTxCmdTxDW::new(self, 1)
    }
    #[doc = "Bit 3 - Master Receive Command (RxD)"]
    #[inline(always)]
    pub fn master_rx_cmd_rx_d(&mut self) -> MasterRxCmdRxDW<I2cm18Spec> {
        MasterRxCmdRxDW::new(self, 3)
    }
    #[doc = "Bit 4 - Master Receive Command Last"]
    #[inline(always)]
    pub fn master_rx_cmd_last(&mut self) -> MasterRxCmdLastW<I2cm18Spec> {
        MasterRxCmdLastW::new(self, 4)
    }
    #[doc = "Bit 5 - Master Stop Command (P)"]
    #[inline(always)]
    pub fn master_stop_cmd_p(&mut self) -> MasterStopCmdPW<I2cm18Spec> {
        MasterStopCmdPW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Master Tx Pool Data buffer"]
    #[inline(always)]
    pub fn enbl_master_tx_pool_data_buf(&mut self) -> EnblMasterTxPoolDataBufW<I2cm18Spec> {
        EnblMasterTxPoolDataBufW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Master Rx Pool Data buffer"]
    #[inline(always)]
    pub fn enbl_master_rx_pool_data_buf(&mut self) -> EnblMasterRxPoolDataBufW<I2cm18Spec> {
        EnblMasterRxPoolDataBufW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Master Tx DMA Data buffer"]
    #[inline(always)]
    pub fn enbl_master_tx_dmadata_buf(&mut self) -> EnblMasterTxDmadataBufW<I2cm18Spec> {
        EnblMasterTxDmadataBufW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Master Rx DMA Data buffer"]
    #[inline(always)]
    pub fn enbl_master_rx_dmadata_buf(&mut self) -> EnblMasterRxDmadataBufW<I2cm18Spec> {
        EnblMasterRxDmadataBufW::new(self, 9)
    }
    #[doc = "Bit 11 - Enable Bus Recover Command"]
    #[inline(always)]
    pub fn enbl_bus_recover_cmd(&mut self) -> EnblBusRecoverCmdW<I2cm18Spec> {
        EnblBusRecoverCmdW::new(self, 11)
    }
    #[doc = "Bit 12 - SCL_O output direct control"]
    #[inline(always)]
    pub fn sclooutput_dir_ctrl(&mut self) -> SclooutputDirCtrlW<I2cm18Spec> {
        SclooutputDirCtrlW::new(self, 12)
    }
    #[doc = "Bit 13 - SCL_OE output direct control"]
    #[inline(always)]
    pub fn scloeoutput_dir_ctrl(&mut self) -> ScloeoutputDirCtrlW<I2cm18Spec> {
        ScloeoutputDirCtrlW::new(self, 13)
    }
    #[doc = "Bit 14 - SDA_O output direct control"]
    #[inline(always)]
    pub fn sdaooutput_dir_ctrl(&mut self) -> SdaooutputDirCtrlW<I2cm18Spec> {
        SdaooutputDirCtrlW::new(self, 14)
    }
    #[doc = "Bit 15 - SDA_OE output direct control"]
    #[inline(always)]
    pub fn sdaoeoutput_dir_ctrl(&mut self) -> SdaoeoutputDirCtrlW<I2cm18Spec> {
        SdaoeoutputDirCtrlW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable master Packet operation mode"]
    #[inline(always)]
    pub fn enbl_master_pkt_op_mode(&mut self) -> EnblMasterPktOpModeW<I2cm18Spec> {
        EnblMasterPktOpModeW::new(self, 16)
    }
    #[doc = "Bits 17:19 - High Speed mode master code LSB"]
    #[inline(always)]
    pub fn high_speed_mode_master_code_lsb(&mut self) -> HighSpeedModeMasterCodeLsbW<I2cm18Spec> {
        HighSpeedModeMasterCodeLsbW::new(self, 17)
    }
    #[doc = "Bits 24:30 - Target device address"]
    #[inline(always)]
    pub fn target_dev_addr(&mut self) -> TargetDevAddrW<I2cm18Spec> {
        TargetDevAddrW::new(self, 24)
    }
    #[doc = "Bit 31 - Write '1' control for I2CM18 for current write command"]
    #[inline(always)]
    pub fn wr1ctrl_for_i2cm18for_cur_wr_cmd(&mut self) -> Wr1ctrlForI2cm18forCurWrCmdW<I2cm18Spec> {
        Wr1ctrlForI2cm18forCurWrCmdW::new(self, 31)
    }
}
#[doc = "Master Command/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cm18Spec;
impl crate::RegisterSpec for I2cm18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cm18::R`](R) reader structure"]
impl crate::Readable for I2cm18Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cm18::W`](W) writer structure"]
impl crate::Writable for I2cm18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CM18 to value 0"]
impl crate::Resettable for I2cm18Spec {}
