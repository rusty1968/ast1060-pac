#[doc = "Register `I2CS20` reader"]
pub type R = crate::R<I2cs20Spec>;
#[doc = "Register `I2CS20` writer"]
pub type W = crate::W<I2cs20Spec>;
#[doc = "Field `EnblTxEndedWithACKRespINT` reader - Enable Transmit ended with ACK response interrupt"]
pub type EnblTxEndedWithAckrespIntR = crate::BitReader;
#[doc = "Field `EnblTxEndedWithACKRespINT` writer - Enable Transmit ended with ACK response interrupt"]
pub type EnblTxEndedWithAckrespIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved03` reader - Reserved (0)"]
pub type Reserved03R = crate::BitReader;
#[doc = "Field `EnblRxDoneINT` reader - Enable Receive Done interrupt"]
pub type EnblRxDoneIntR = crate::BitReader;
#[doc = "Field `EnblRxDoneINT` writer - Enable Receive Done interrupt"]
pub type EnblRxDoneIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "Field `EnblNormalStopConDetINT` reader - Enable normal Stop condition detection interrupt"]
pub type EnblNormalStopConDetIntR = crate::BitReader;
#[doc = "Field `EnblNormalStopConDetINT` writer - Enable normal Stop condition detection interrupt"]
pub type EnblNormalStopConDetIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblAbnStartStopConDetINT` reader - Enable abnormal Start/Stop condition detection interrupt"]
pub type EnblAbnStartStopConDetIntR = crate::BitReader;
#[doc = "Field `EnblAbnStartStopConDetINT` writer - Enable abnormal Start/Stop condition detection interrupt"]
pub type EnblAbnStartStopConDetIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader<u16>;
#[doc = "Field `EnblSlaveModeInactiveTimeoutINT` reader - Enable Slave mode inactive timeout interrupt"]
pub type EnblSlaveModeInactiveTimeoutIntR = crate::BitReader;
#[doc = "Field `EnblSlaveModeInactiveTimeoutINT` writer - Enable Slave mode inactive timeout interrupt"]
pub type EnblSlaveModeInactiveTimeoutIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblPktCmdDoneINT` reader - Enable Packet command done interrupt"]
pub type EnblPktCmdDoneIntR = crate::BitReader;
#[doc = "Field `EnblPktCmdDoneINT` writer - Enable Packet command done interrupt"]
pub type EnblPktCmdDoneIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSlaveAddrMatchedButNACKedINT` reader - Enable Slave address matched but NACKed interrupt"]
pub type EnblSlaveAddrMatchedButNackedIntR = crate::BitReader;
#[doc = "Field `EnblSlaveAddrMatchedButNACKedINT` writer - Enable Slave address matched but NACKed interrupt"]
pub type EnblSlaveAddrMatchedButNackedIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable Transmit ended with ACK response interrupt"]
    #[inline(always)]
    pub fn enbl_tx_ended_with_ackresp_int(&self) -> EnblTxEndedWithAckrespIntR {
        EnblTxEndedWithAckrespIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved03(&self) -> Reserved03R {
        Reserved03R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Receive Done interrupt"]
    #[inline(always)]
    pub fn enbl_rx_done_int(&self) -> EnblRxDoneIntR {
        EnblRxDoneIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bits 6:14 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Enable Slave mode inactive timeout interrupt"]
    #[inline(always)]
    pub fn enbl_slave_mode_inactive_timeout_int(&self) -> EnblSlaveModeInactiveTimeoutIntR {
        EnblSlaveModeInactiveTimeoutIntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Packet command done interrupt"]
    #[inline(always)]
    pub fn enbl_pkt_cmd_done_int(&self) -> EnblPktCmdDoneIntR {
        EnblPktCmdDoneIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable Slave address matched but NACKed interrupt"]
    #[inline(always)]
    pub fn enbl_slave_addr_matched_but_nacked_int(&self) -> EnblSlaveAddrMatchedButNackedIntR {
        EnblSlaveAddrMatchedButNackedIntR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Transmit ended with ACK response interrupt"]
    #[inline(always)]
    pub fn enbl_tx_ended_with_ackresp_int(&mut self) -> EnblTxEndedWithAckrespIntW<I2cs20Spec> {
        EnblTxEndedWithAckrespIntW::new(self, 0)
    }
    #[doc = "Bit 2 - Enable Receive Done interrupt"]
    #[inline(always)]
    pub fn enbl_rx_done_int(&mut self) -> EnblRxDoneIntW<I2cs20Spec> {
        EnblRxDoneIntW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable normal Stop condition detection interrupt"]
    #[inline(always)]
    pub fn enbl_normal_stop_con_det_int(&mut self) -> EnblNormalStopConDetIntW<I2cs20Spec> {
        EnblNormalStopConDetIntW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable abnormal Start/Stop condition detection interrupt"]
    #[inline(always)]
    pub fn enbl_abn_start_stop_con_det_int(&mut self) -> EnblAbnStartStopConDetIntW<I2cs20Spec> {
        EnblAbnStartStopConDetIntW::new(self, 5)
    }
    #[doc = "Bit 15 - Enable Slave mode inactive timeout interrupt"]
    #[inline(always)]
    pub fn enbl_slave_mode_inactive_timeout_int(
        &mut self,
    ) -> EnblSlaveModeInactiveTimeoutIntW<I2cs20Spec> {
        EnblSlaveModeInactiveTimeoutIntW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Packet command done interrupt"]
    #[inline(always)]
    pub fn enbl_pkt_cmd_done_int(&mut self) -> EnblPktCmdDoneIntW<I2cs20Spec> {
        EnblPktCmdDoneIntW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable Slave address matched but NACKed interrupt"]
    #[inline(always)]
    pub fn enbl_slave_addr_matched_but_nacked_int(
        &mut self,
    ) -> EnblSlaveAddrMatchedButNackedIntW<I2cs20Spec> {
        EnblSlaveAddrMatchedButNackedIntW::new(self, 17)
    }
}
#[doc = "Slave Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs20Spec;
impl crate::RegisterSpec for I2cs20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs20::R`](R) reader structure"]
impl crate::Readable for I2cs20Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cs20::W`](W) writer structure"]
impl crate::Writable for I2cs20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS20 to value 0"]
impl crate::Resettable for I2cs20Spec {}
