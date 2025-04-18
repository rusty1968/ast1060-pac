#[doc = "Register `I2CS24` reader"]
pub type R = crate::R<I2cs24Spec>;
#[doc = "Register `I2CS24` writer"]
pub type W = crate::W<I2cs24Spec>;
#[doc = "Field `WCTxEndedWithACKRespINTSts` reader - (WC) Transmit ended with ACK response interrupt status"]
pub type WctxEndedWithAckrespIntstsR = crate::BitReader;
#[doc = "Field `WCTxEndedWithACKRespINTSts` writer - (WC) Transmit ended with ACK response interrupt status"]
pub type WctxEndedWithAckrespIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCTxEndedWithNACKRespINTSts` reader - (WC) Transmit ended with NACK response interrupt status"]
pub type WctxEndedWithNackrespIntstsR = crate::BitReader;
#[doc = "Field `WCTxEndedWithNACKRespINTSts` writer - (WC) Transmit ended with NACK response interrupt status"]
pub type WctxEndedWithNackrespIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCRxDoneINTSts` reader - (WC) Receive Done interrupt status"]
pub type WcrxDoneIntstsR = crate::BitReader;
#[doc = "Field `WCRxDoneINTSts` writer - (WC) Receive Done interrupt status"]
pub type WcrxDoneIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxDoneEndedWithReturningNACK` reader - Receive Done ended with returning NACK"]
pub type RxDoneEndedWithReturningNackR = crate::BitReader;
#[doc = "Field `WCNormalStopCondDetINTSts` reader - (WC) Normal Stop Condition Detection interrupt status"]
pub type WcnormalStopCondDetIntstsR = crate::BitReader;
#[doc = "Field `WCNormalStopCondDetINTSts` writer - (WC) Normal Stop Condition Detection interrupt status"]
pub type WcnormalStopCondDetIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCAbnStartStopCondDetINTSts` reader - (WC) Abnormal Start/Stop Condition Detection interrupt status"]
pub type WcabnStartStopCondDetIntstsR = crate::BitReader;
#[doc = "Field `WCAbnStartStopCondDetINTSts` writer - (WC) Abnormal Start/Stop Condition Detection interrupt status"]
pub type WcabnStartStopCondDetIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `WCSlaveAddrRxdMatchINTSts` reader - (WC) Slave Address Received Match interrupt status"]
pub type WcslaveAddrRxdMatchIntstsR = crate::BitReader;
#[doc = "Field `WCSlaveAddrRxdMatchINTSts` writer - (WC) Slave Address Received Match interrupt status"]
pub type WcslaveAddrRxdMatchIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `WCSlaveModeInactiveTimeoutINTSts` reader - (WC) Slave mode inactive timeout interrupt status"]
pub type WcslaveModeInactiveTimeoutIntstsR = crate::BitReader;
#[doc = "Field `WCSlaveModeInactiveTimeoutINTSts` writer - (WC) Slave mode inactive timeout interrupt status"]
pub type WcslaveModeInactiveTimeoutIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCPktCmdDoneINTSts` reader - (WC) newverPacket command done interrupt status"]
pub type WcpktCmdDoneIntstsR = crate::BitReader;
#[doc = "Field `WCPktCmdDoneINTSts` writer - (WC) newverPacket command done interrupt status"]
pub type WcpktCmdDoneIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCPktCmdFailINTSts` reader - (WC) newverPacket command fail interrupt status"]
pub type WcpktCmdFailIntstsR = crate::BitReader;
#[doc = "Field `WCPktCmdFailINTSts` writer - (WC) newverPacket command fail interrupt status"]
pub type WcpktCmdFailIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TheCurActiveSlaveAddrIndicator` reader - The current active Slave address indicator"]
pub type TheCurActiveSlaveAddrIndicatorR = crate::FieldReader;
#[doc = "Field `WCSlaveAddr1NACKedIndicator` reader - (WC) newverSlave address1 NACKed indicator"]
pub type WcslaveAddr1nackedIndicatorR = crate::BitReader;
#[doc = "Field `WCSlaveAddr1NACKedIndicator` writer - (WC) newverSlave address1 NACKed indicator"]
pub type WcslaveAddr1nackedIndicatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCSlaveAddr2NACKedIndicator` reader - (WC) newverSlave address2 NACKed indicator"]
pub type WcslaveAddr2nackedIndicatorR = crate::BitReader;
#[doc = "Field `WCSlaveAddr2NACKedIndicator` writer - (WC) newverSlave address2 NACKed indicator"]
pub type WcslaveAddr2nackedIndicatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCSlaveAddr3NACKedIndicator` reader - (WC) newverSlave address3 NACKed indicator"]
pub type WcslaveAddr3nackedIndicatorR = crate::BitReader;
#[doc = "Field `WCSlaveAddr3NACKedIndicator` writer - (WC) newverSlave address3 NACKed indicator"]
pub type WcslaveAddr3nackedIndicatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `CurSlaveParkingSts` reader - Current Slave parking status"]
pub type CurSlaveParkingStsR = crate::FieldReader;
#[doc = "Field `SlaveAddrRxdPending` reader - Slave Address Received Pending"]
pub type SlaveAddrRxdPendingR = crate::BitReader;
#[doc = "Field `TheLastSlaveAddrMatchIndicator` reader - The Last Slave address match indicator"]
pub type TheLastSlaveAddrMatchIndicatorR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - (WC) Transmit ended with ACK response interrupt status"]
    #[inline(always)]
    pub fn wctx_ended_with_ackresp_intsts(&self) -> WctxEndedWithAckrespIntstsR {
        WctxEndedWithAckrespIntstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (WC) Transmit ended with NACK response interrupt status"]
    #[inline(always)]
    pub fn wctx_ended_with_nackresp_intsts(&self) -> WctxEndedWithNackrespIntstsR {
        WctxEndedWithNackrespIntstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (WC) Receive Done interrupt status"]
    #[inline(always)]
    pub fn wcrx_done_intsts(&self) -> WcrxDoneIntstsR {
        WcrxDoneIntstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Done ended with returning NACK"]
    #[inline(always)]
    pub fn rx_done_ended_with_returning_nack(&self) -> RxDoneEndedWithReturningNackR {
        RxDoneEndedWithReturningNackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (WC) Normal Stop Condition Detection interrupt status"]
    #[inline(always)]
    pub fn wcnormal_stop_cond_det_intsts(&self) -> WcnormalStopCondDetIntstsR {
        WcnormalStopCondDetIntstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - (WC) Abnormal Start/Stop Condition Detection interrupt status"]
    #[inline(always)]
    pub fn wcabn_start_stop_cond_det_intsts(&self) -> WcabnStartStopCondDetIntstsR {
        WcabnStartStopCondDetIntstsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - (WC) Slave Address Received Match interrupt status"]
    #[inline(always)]
    pub fn wcslave_addr_rxd_match_intsts(&self) -> WcslaveAddrRxdMatchIntstsR {
        WcslaveAddrRxdMatchIntstsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - (WC) Slave mode inactive timeout interrupt status"]
    #[inline(always)]
    pub fn wcslave_mode_inactive_timeout_intsts(&self) -> WcslaveModeInactiveTimeoutIntstsR {
        WcslaveModeInactiveTimeoutIntstsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - (WC) newverPacket command done interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_done_intsts(&self) -> WcpktCmdDoneIntstsR {
        WcpktCmdDoneIntstsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - (WC) newverPacket command fail interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_fail_intsts(&self) -> WcpktCmdFailIntstsR {
        WcpktCmdFailIntstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - The current active Slave address indicator"]
    #[inline(always)]
    pub fn the_cur_active_slave_addr_indicator(&self) -> TheCurActiveSlaveAddrIndicatorR {
        TheCurActiveSlaveAddrIndicatorR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - (WC) newverSlave address1 NACKed indicator"]
    #[inline(always)]
    pub fn wcslave_addr1nacked_indicator(&self) -> WcslaveAddr1nackedIndicatorR {
        WcslaveAddr1nackedIndicatorR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - (WC) newverSlave address2 NACKed indicator"]
    #[inline(always)]
    pub fn wcslave_addr2nacked_indicator(&self) -> WcslaveAddr2nackedIndicatorR {
        WcslaveAddr2nackedIndicatorR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - (WC) newverSlave address3 NACKed indicator"]
    #[inline(always)]
    pub fn wcslave_addr3nacked_indicator(&self) -> WcslaveAddr3nackedIndicatorR {
        WcslaveAddr3nackedIndicatorR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Current Slave parking status"]
    #[inline(always)]
    pub fn cur_slave_parking_sts(&self) -> CurSlaveParkingStsR {
        CurSlaveParkingStsR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 29 - Slave Address Received Pending"]
    #[inline(always)]
    pub fn slave_addr_rxd_pending(&self) -> SlaveAddrRxdPendingR {
        SlaveAddrRxdPendingR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - The Last Slave address match indicator"]
    #[inline(always)]
    pub fn the_last_slave_addr_match_indicator(&self) -> TheLastSlaveAddrMatchIndicatorR {
        TheLastSlaveAddrMatchIndicatorR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - (WC) Transmit ended with ACK response interrupt status"]
    #[inline(always)]
    pub fn wctx_ended_with_ackresp_intsts(&mut self) -> WctxEndedWithAckrespIntstsW<I2cs24Spec> {
        WctxEndedWithAckrespIntstsW::new(self, 0)
    }
    #[doc = "Bit 1 - (WC) Transmit ended with NACK response interrupt status"]
    #[inline(always)]
    pub fn wctx_ended_with_nackresp_intsts(&mut self) -> WctxEndedWithNackrespIntstsW<I2cs24Spec> {
        WctxEndedWithNackrespIntstsW::new(self, 1)
    }
    #[doc = "Bit 2 - (WC) Receive Done interrupt status"]
    #[inline(always)]
    pub fn wcrx_done_intsts(&mut self) -> WcrxDoneIntstsW<I2cs24Spec> {
        WcrxDoneIntstsW::new(self, 2)
    }
    #[doc = "Bit 4 - (WC) Normal Stop Condition Detection interrupt status"]
    #[inline(always)]
    pub fn wcnormal_stop_cond_det_intsts(&mut self) -> WcnormalStopCondDetIntstsW<I2cs24Spec> {
        WcnormalStopCondDetIntstsW::new(self, 4)
    }
    #[doc = "Bit 5 - (WC) Abnormal Start/Stop Condition Detection interrupt status"]
    #[inline(always)]
    pub fn wcabn_start_stop_cond_det_intsts(&mut self) -> WcabnStartStopCondDetIntstsW<I2cs24Spec> {
        WcabnStartStopCondDetIntstsW::new(self, 5)
    }
    #[doc = "Bit 7 - (WC) Slave Address Received Match interrupt status"]
    #[inline(always)]
    pub fn wcslave_addr_rxd_match_intsts(&mut self) -> WcslaveAddrRxdMatchIntstsW<I2cs24Spec> {
        WcslaveAddrRxdMatchIntstsW::new(self, 7)
    }
    #[doc = "Bit 15 - (WC) Slave mode inactive timeout interrupt status"]
    #[inline(always)]
    pub fn wcslave_mode_inactive_timeout_intsts(
        &mut self,
    ) -> WcslaveModeInactiveTimeoutIntstsW<I2cs24Spec> {
        WcslaveModeInactiveTimeoutIntstsW::new(self, 15)
    }
    #[doc = "Bit 16 - (WC) newverPacket command done interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_done_intsts(&mut self) -> WcpktCmdDoneIntstsW<I2cs24Spec> {
        WcpktCmdDoneIntstsW::new(self, 16)
    }
    #[doc = "Bit 17 - (WC) newverPacket command fail interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_fail_intsts(&mut self) -> WcpktCmdFailIntstsW<I2cs24Spec> {
        WcpktCmdFailIntstsW::new(self, 17)
    }
    #[doc = "Bit 20 - (WC) newverSlave address1 NACKed indicator"]
    #[inline(always)]
    pub fn wcslave_addr1nacked_indicator(&mut self) -> WcslaveAddr1nackedIndicatorW<I2cs24Spec> {
        WcslaveAddr1nackedIndicatorW::new(self, 20)
    }
    #[doc = "Bit 21 - (WC) newverSlave address2 NACKed indicator"]
    #[inline(always)]
    pub fn wcslave_addr2nacked_indicator(&mut self) -> WcslaveAddr2nackedIndicatorW<I2cs24Spec> {
        WcslaveAddr2nackedIndicatorW::new(self, 21)
    }
    #[doc = "Bit 22 - (WC) newverSlave address3 NACKed indicator"]
    #[inline(always)]
    pub fn wcslave_addr3nacked_indicator(&mut self) -> WcslaveAddr3nackedIndicatorW<I2cs24Spec> {
        WcslaveAddr3nackedIndicatorW::new(self, 22)
    }
}
#[doc = "Slave Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs24Spec;
impl crate::RegisterSpec for I2cs24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs24::R`](R) reader structure"]
impl crate::Readable for I2cs24Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cs24::W`](W) writer structure"]
impl crate::Writable for I2cs24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS24 to value 0"]
impl crate::Resettable for I2cs24Spec {}
