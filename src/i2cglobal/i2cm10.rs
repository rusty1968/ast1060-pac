#[doc = "Register `I2CM10` reader"]
pub type R = crate::R<I2cm10Spec>;
#[doc = "Register `I2CM10` writer"]
pub type W = crate::W<I2cm10Spec>;
#[doc = "Field `EnblTxEndedWithACKRespINT` reader - Enable Transmit ended with ACK response interrupt"]
pub type EnblTxEndedWithAckrespIntR = crate::BitReader;
#[doc = "Field `EnblTxEndedWithACKRespINT` writer - Enable Transmit ended with ACK response interrupt"]
pub type EnblTxEndedWithAckrespIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblTxEndedWithNACKRespINT` reader - Enable Transmit ended with NACK response interrupt"]
pub type EnblTxEndedWithNackrespIntR = crate::BitReader;
#[doc = "Field `EnblTxEndedWithNACKRespINT` writer - Enable Transmit ended with NACK response interrupt"]
pub type EnblTxEndedWithNackrespIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRxDoneINT` reader - Enable Receive Done interrupt"]
pub type EnblRxDoneIntR = crate::BitReader;
#[doc = "Field `EnblRxDoneINT` writer - Enable Receive Done interrupt"]
pub type EnblRxDoneIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblMasterArbLossINT` reader - Enable master arbitration loss interrupt"]
pub type EnblMasterArbLossIntR = crate::BitReader;
#[doc = "Field `EnblMasterArbLossINT` writer - Enable master arbitration loss interrupt"]
pub type EnblMasterArbLossIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblNormalStopConDetINT` reader - Enable normal Stop condition detection interrupt"]
pub type EnblNormalStopConDetIntR = crate::BitReader;
#[doc = "Field `EnblNormalStopConDetINT` writer - Enable normal Stop condition detection interrupt"]
pub type EnblNormalStopConDetIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblAbnStartStopConDetINT` reader - Enable abnormal Start/Stop condition detection interrupt"]
pub type EnblAbnStartStopConDetIntR = crate::BitReader;
#[doc = "Field `EnblAbnStartStopConDetINT` writer - Enable abnormal Start/Stop condition detection interrupt"]
pub type EnblAbnStartStopConDetIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSCLClklowTimeoutINT` reader - Enable SCL clock-low timeout interrupt"]
pub type EnblSclclklowTimeoutIntR = crate::BitReader;
#[doc = "Field `EnblSCLClklowTimeoutINT` writer - Enable SCL clock-low timeout interrupt"]
pub type EnblSclclklowTimeoutIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `EnblSMBusDevAlertINT` reader - Enable SMBus Device Alert interrupt"]
pub type EnblSmbusDevAlertIntR = crate::BitReader;
#[doc = "Field `EnblSMBusDevAlertINT` writer - Enable SMBus Device Alert interrupt"]
pub type EnblSmbusDevAlertIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblBusRecoverDoneINT` reader - Enable Bus Recover Done interrupt"]
pub type EnblBusRecoverDoneIntR = crate::BitReader;
#[doc = "Field `EnblBusRecoverDoneINT` writer - Enable Bus Recover Done interrupt"]
pub type EnblBusRecoverDoneIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSDADatalowTimeoutINT` reader - Enable SDA data-low timeout interrupt"]
pub type EnblSdadatalowTimeoutIntR = crate::BitReader;
#[doc = "Field `EnblSDADatalowTimeoutINT` writer - Enable SDA data-low timeout interrupt"]
pub type EnblSdadatalowTimeoutIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `EnblPktCmdDoneINT` reader - newverEnable Packet command done interrupt"]
pub type EnblPktCmdDoneIntR = crate::BitReader;
#[doc = "Field `EnblPktCmdDoneINT` writer - newverEnable Packet command done interrupt"]
pub type EnblPktCmdDoneIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable Transmit ended with ACK response interrupt"]
    #[inline(always)]
    pub fn enbl_tx_ended_with_ackresp_int(&self) -> EnblTxEndedWithAckrespIntR {
        EnblTxEndedWithAckrespIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Transmit ended with NACK response interrupt"]
    #[inline(always)]
    pub fn enbl_tx_ended_with_nackresp_int(&self) -> EnblTxEndedWithNackrespIntR {
        EnblTxEndedWithNackrespIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receive Done interrupt"]
    #[inline(always)]
    pub fn enbl_rx_done_int(&self) -> EnblRxDoneIntR {
        EnblRxDoneIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable master arbitration loss interrupt"]
    #[inline(always)]
    pub fn enbl_master_arb_loss_int(&self) -> EnblMasterArbLossIntR {
        EnblMasterArbLossIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable normal Stop condition detection interrupt"]
    #[inline(always)]
    pub fn enbl_normal_stop_con_det_int(&self) -> EnblNormalStopConDetIntR {
        EnblNormalStopConDetIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable abnormal Start/Stop condition detection interrupt"]
    #[inline(always)]
    pub fn enbl_abn_start_stop_con_det_int(&self) -> EnblAbnStartStopConDetIntR {
        EnblAbnStartStopConDetIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable SCL clock-low timeout interrupt"]
    #[inline(always)]
    pub fn enbl_sclclklow_timeout_int(&self) -> EnblSclclklowTimeoutIntR {
        EnblSclclklowTimeoutIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - Enable SMBus Device Alert interrupt"]
    #[inline(always)]
    pub fn enbl_smbus_dev_alert_int(&self) -> EnblSmbusDevAlertIntR {
        EnblSmbusDevAlertIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Bus Recover Done interrupt"]
    #[inline(always)]
    pub fn enbl_bus_recover_done_int(&self) -> EnblBusRecoverDoneIntR {
        EnblBusRecoverDoneIntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable SDA data-low timeout interrupt"]
    #[inline(always)]
    pub fn enbl_sdadatalow_timeout_int(&self) -> EnblSdadatalowTimeoutIntR {
        EnblSdadatalowTimeoutIntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - newverEnable Packet command done interrupt"]
    #[inline(always)]
    pub fn enbl_pkt_cmd_done_int(&self) -> EnblPktCmdDoneIntR {
        EnblPktCmdDoneIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Transmit ended with ACK response interrupt"]
    #[inline(always)]
    pub fn enbl_tx_ended_with_ackresp_int(&mut self) -> EnblTxEndedWithAckrespIntW<I2cm10Spec> {
        EnblTxEndedWithAckrespIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Transmit ended with NACK response interrupt"]
    #[inline(always)]
    pub fn enbl_tx_ended_with_nackresp_int(&mut self) -> EnblTxEndedWithNackrespIntW<I2cm10Spec> {
        EnblTxEndedWithNackrespIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Receive Done interrupt"]
    #[inline(always)]
    pub fn enbl_rx_done_int(&mut self) -> EnblRxDoneIntW<I2cm10Spec> {
        EnblRxDoneIntW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable master arbitration loss interrupt"]
    #[inline(always)]
    pub fn enbl_master_arb_loss_int(&mut self) -> EnblMasterArbLossIntW<I2cm10Spec> {
        EnblMasterArbLossIntW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable normal Stop condition detection interrupt"]
    #[inline(always)]
    pub fn enbl_normal_stop_con_det_int(&mut self) -> EnblNormalStopConDetIntW<I2cm10Spec> {
        EnblNormalStopConDetIntW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable abnormal Start/Stop condition detection interrupt"]
    #[inline(always)]
    pub fn enbl_abn_start_stop_con_det_int(&mut self) -> EnblAbnStartStopConDetIntW<I2cm10Spec> {
        EnblAbnStartStopConDetIntW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable SCL clock-low timeout interrupt"]
    #[inline(always)]
    pub fn enbl_sclclklow_timeout_int(&mut self) -> EnblSclclklowTimeoutIntW<I2cm10Spec> {
        EnblSclclklowTimeoutIntW::new(self, 6)
    }
    #[doc = "Bit 12 - Enable SMBus Device Alert interrupt"]
    #[inline(always)]
    pub fn enbl_smbus_dev_alert_int(&mut self) -> EnblSmbusDevAlertIntW<I2cm10Spec> {
        EnblSmbusDevAlertIntW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable Bus Recover Done interrupt"]
    #[inline(always)]
    pub fn enbl_bus_recover_done_int(&mut self) -> EnblBusRecoverDoneIntW<I2cm10Spec> {
        EnblBusRecoverDoneIntW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable SDA data-low timeout interrupt"]
    #[inline(always)]
    pub fn enbl_sdadatalow_timeout_int(&mut self) -> EnblSdadatalowTimeoutIntW<I2cm10Spec> {
        EnblSdadatalowTimeoutIntW::new(self, 14)
    }
    #[doc = "Bit 16 - newverEnable Packet command done interrupt"]
    #[inline(always)]
    pub fn enbl_pkt_cmd_done_int(&mut self) -> EnblPktCmdDoneIntW<I2cm10Spec> {
        EnblPktCmdDoneIntW::new(self, 16)
    }
}
#[doc = "Master Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cm10Spec;
impl crate::RegisterSpec for I2cm10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cm10::R`](R) reader structure"]
impl crate::Readable for I2cm10Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cm10::W`](W) writer structure"]
impl crate::Writable for I2cm10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CM10 to value 0"]
impl crate::Resettable for I2cm10Spec {}
