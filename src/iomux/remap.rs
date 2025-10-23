#[doc = "Register `REMAP` reader"]
pub type R = crate::R<RemapSpec>;
#[doc = "Register `REMAP` writer"]
pub type W = crate::W<RemapSpec>;
#[doc = "Field `SPI1_MUX0` reader - SPI1 muxing bit0"]
pub type Spi1Mux0R = crate::BitReader;
#[doc = "Field `SPI1_MUX0` writer - SPI1 muxing bit0"]
pub type Spi1Mux0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1_MUX` reader - I2C1 muxing"]
pub type I2c1MuxR = crate::BitReader;
#[doc = "Field `I2C1_MUX` writer - I2C1 muxing"]
pub type I2c1MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1_MUX` reader - USART1 muxing"]
pub type Usart1MuxR = crate::BitReader;
#[doc = "Field `USART1_MUX` writer - USART1 muxing"]
pub type Usart1MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2_MUX` reader - USART2 muxing"]
pub type Usart2MuxR = crate::BitReader;
#[doc = "Field `USART2_MUX` writer - USART2 muxing"]
pub type Usart2MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3_MUX` reader - USART3 muxing"]
pub type Usart3MuxR = crate::FieldReader;
#[doc = "Field `USART3_MUX` writer - USART3 muxing"]
pub type Usart3MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR1_MUX` reader - TMR1 muxing"]
pub type Tmr1MuxR = crate::FieldReader;
#[doc = "Field `TMR1_MUX` writer - TMR1 muxing"]
pub type Tmr1MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR2_MUX` reader - TMR2 muxing"]
pub type Tmr2MuxR = crate::FieldReader;
#[doc = "Field `TMR2_MUX` writer - TMR2 muxing"]
pub type Tmr2MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR3_MUX` reader - TMR3 muxing"]
pub type Tmr3MuxR = crate::FieldReader;
#[doc = "Field `TMR3_MUX` writer - TMR3 muxing"]
pub type Tmr3MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR4_MUX` reader - TMR4 muxing"]
pub type Tmr4MuxR = crate::BitReader;
#[doc = "Field `TMR4_MUX` writer - TMR4 muxing"]
pub type Tmr4MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN_MUX` reader - CAN1 muxing"]
pub type CanMuxR = crate::FieldReader;
#[doc = "Field `CAN_MUX` writer - CAN1 muxing"]
pub type CanMuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PD01_MUX` reader - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type Pd01MuxR = crate::BitReader;
#[doc = "Field `PD01_MUX` writer - PD0/PD1 muxing on OSCIN/OSCOUT"]
pub type Pd01MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR5CH4_MUX` reader - TMR5 channel4 internal muxing"]
pub type Tmr5ch4MuxR = crate::BitReader;
#[doc = "Field `TMR5CH4_MUX` writer - TMR5 channel4 internal muxing"]
pub type Tmr5ch4MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETP_MUX` reader - ADC1 external trigger preempted conversion muxing"]
pub type Adc1EtpMuxR = crate::BitReader;
#[doc = "Field `ADC1_ETP_MUX` writer - ADC1 external trigger preempted conversion muxing"]
pub type Adc1EtpMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_ETO_MUX` reader - ADC1 external trigger ordinary conversion muxing"]
pub type Adc1EtoMuxR = crate::BitReader;
#[doc = "Field `ADC1_ETO_MUX` writer - ADC1 external trigger ordinary conversion muxing"]
pub type Adc1EtoMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETP_MUX` reader - ADC2 external trigger preempted conversion muxing"]
pub type Adc2EtpMuxR = crate::BitReader;
#[doc = "Field `ADC2_ETP_MUX` writer - ADC2 external trigger preempted conversion muxing"]
pub type Adc2EtpMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC2_ETO_MUX` reader - ADC2 external trigger ordinary conversion muxing"]
pub type Adc2EtoMuxR = crate::BitReader;
#[doc = "Field `ADC2_ETO_MUX` writer - ADC2 external trigger ordinary conversion muxing"]
pub type Adc2EtoMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMAC_MUX` reader - Ethernet MAC muxing"]
pub type EmacMuxR = crate::BitReader;
#[doc = "Field `EMAC_MUX` writer - Ethernet MAC muxing"]
pub type EmacMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2_MUX` reader - CAN2 muxing"]
pub type Can2MuxR = crate::BitReader;
#[doc = "Field `CAN2_MUX` writer - CAN2 muxing"]
pub type Can2MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MII_RMII_SEL_MUX` reader - MII_RMII select muxing"]
pub type MiiRmiiSelMuxR = crate::BitReader;
#[doc = "Field `MII_RMII_SEL_MUX` writer - MII_RMII select muxing"]
pub type MiiRmiiSelMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWJTAG_MUX` reader - SWD JTAG muxing"]
pub type SwjtagMuxR = crate::FieldReader;
#[doc = "Field `SWJTAG_MUX` writer - SWD JTAG muxing"]
pub type SwjtagMuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI3_MUX` reader - SPI3 muxing"]
pub type Spi3MuxR = crate::BitReader;
#[doc = "Field `SPI3_MUX` writer - SPI3 muxing"]
pub type Spi3MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR2ITR1_MUX` reader - TMR2 internal trigger 1 muxing"]
pub type Tmr2itr1MuxR = crate::BitReader;
#[doc = "Field `TMR2ITR1_MUX` writer - TMR2 internal trigger 1 muxing"]
pub type Tmr2itr1MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTP_PPS_MUX` reader - PTP_PPS muxing"]
pub type PtpPpsMuxR = crate::BitReader;
#[doc = "Field `PTP_PPS_MUX` writer - PTP_PPS muxing"]
pub type PtpPpsMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1_MUX1` reader - SPI1 muxing bit1"]
pub type Spi1Mux1R = crate::BitReader;
#[doc = "Field `SPI1_MUX1` writer - SPI1 muxing bit1"]
pub type Spi1Mux1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    pub fn spi1_mux0(&self) -> Spi1Mux0R {
        Spi1Mux0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_mux(&self) -> I2c1MuxR {
        I2c1MuxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_mux(&self) -> Usart1MuxR {
        Usart1MuxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USART2 muxing"]
    #[inline(always)]
    pub fn usart2_mux(&self) -> Usart2MuxR {
        Usart2MuxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_mux(&self) -> Usart3MuxR {
        Usart3MuxR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_mux(&self) -> Tmr1MuxR {
        Tmr1MuxR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_mux(&self) -> Tmr2MuxR {
        Tmr2MuxR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_mux(&self) -> Tmr3MuxR {
        Tmr3MuxR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - TMR4 muxing"]
    #[inline(always)]
    pub fn tmr4_mux(&self) -> Tmr4MuxR {
        Tmr4MuxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    pub fn can_mux(&self) -> CanMuxR {
        CanMuxR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_mux(&self) -> Pd01MuxR {
        Pd01MuxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_mux(&self) -> Tmr5ch4MuxR {
        Tmr5ch4MuxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_mux(&self) -> Adc1EtpMuxR {
        Adc1EtpMuxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_mux(&self) -> Adc1EtoMuxR {
        Adc1EtoMuxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc2_etp_mux(&self) -> Adc2EtpMuxR {
        Adc2EtpMuxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc2_eto_mux(&self) -> Adc2EtoMuxR {
        Adc2EtoMuxR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Ethernet MAC muxing"]
    #[inline(always)]
    pub fn emac_mux(&self) -> EmacMuxR {
        EmacMuxR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CAN2 muxing"]
    #[inline(always)]
    pub fn can2_mux(&self) -> Can2MuxR {
        Can2MuxR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MII_RMII select muxing"]
    #[inline(always)]
    pub fn mii_rmii_sel_mux(&self) -> MiiRmiiSelMuxR {
        MiiRmiiSelMuxR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_mux(&self) -> SwjtagMuxR {
        SwjtagMuxR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - SPI3 muxing"]
    #[inline(always)]
    pub fn spi3_mux(&self) -> Spi3MuxR {
        Spi3MuxR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TMR2 internal trigger 1 muxing"]
    #[inline(always)]
    pub fn tmr2itr1_mux(&self) -> Tmr2itr1MuxR {
        Tmr2itr1MuxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - PTP_PPS muxing"]
    #[inline(always)]
    pub fn ptp_pps_mux(&self) -> PtpPpsMuxR {
        PtpPpsMuxR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    pub fn spi1_mux1(&self) -> Spi1Mux1R {
        Spi1Mux1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI1 muxing bit0"]
    #[inline(always)]
    pub fn spi1_mux0(&mut self) -> Spi1Mux0W<'_, RemapSpec> {
        Spi1Mux0W::new(self, 0)
    }
    #[doc = "Bit 1 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_mux(&mut self) -> I2c1MuxW<'_, RemapSpec> {
        I2c1MuxW::new(self, 1)
    }
    #[doc = "Bit 2 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_mux(&mut self) -> Usart1MuxW<'_, RemapSpec> {
        Usart1MuxW::new(self, 2)
    }
    #[doc = "Bit 3 - USART2 muxing"]
    #[inline(always)]
    pub fn usart2_mux(&mut self) -> Usart2MuxW<'_, RemapSpec> {
        Usart2MuxW::new(self, 3)
    }
    #[doc = "Bits 4:5 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_mux(&mut self) -> Usart3MuxW<'_, RemapSpec> {
        Usart3MuxW::new(self, 4)
    }
    #[doc = "Bits 6:7 - TMR1 muxing"]
    #[inline(always)]
    pub fn tmr1_mux(&mut self) -> Tmr1MuxW<'_, RemapSpec> {
        Tmr1MuxW::new(self, 6)
    }
    #[doc = "Bits 8:9 - TMR2 muxing"]
    #[inline(always)]
    pub fn tmr2_mux(&mut self) -> Tmr2MuxW<'_, RemapSpec> {
        Tmr2MuxW::new(self, 8)
    }
    #[doc = "Bits 10:11 - TMR3 muxing"]
    #[inline(always)]
    pub fn tmr3_mux(&mut self) -> Tmr3MuxW<'_, RemapSpec> {
        Tmr3MuxW::new(self, 10)
    }
    #[doc = "Bit 12 - TMR4 muxing"]
    #[inline(always)]
    pub fn tmr4_mux(&mut self) -> Tmr4MuxW<'_, RemapSpec> {
        Tmr4MuxW::new(self, 12)
    }
    #[doc = "Bits 13:14 - CAN1 muxing"]
    #[inline(always)]
    pub fn can_mux(&mut self) -> CanMuxW<'_, RemapSpec> {
        CanMuxW::new(self, 13)
    }
    #[doc = "Bit 15 - PD0/PD1 muxing on OSCIN/OSCOUT"]
    #[inline(always)]
    pub fn pd01_mux(&mut self) -> Pd01MuxW<'_, RemapSpec> {
        Pd01MuxW::new(self, 15)
    }
    #[doc = "Bit 16 - TMR5 channel4 internal muxing"]
    #[inline(always)]
    pub fn tmr5ch4_mux(&mut self) -> Tmr5ch4MuxW<'_, RemapSpec> {
        Tmr5ch4MuxW::new(self, 16)
    }
    #[doc = "Bit 17 - ADC1 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc1_etp_mux(&mut self) -> Adc1EtpMuxW<'_, RemapSpec> {
        Adc1EtpMuxW::new(self, 17)
    }
    #[doc = "Bit 18 - ADC1 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc1_eto_mux(&mut self) -> Adc1EtoMuxW<'_, RemapSpec> {
        Adc1EtoMuxW::new(self, 18)
    }
    #[doc = "Bit 19 - ADC2 external trigger preempted conversion muxing"]
    #[inline(always)]
    pub fn adc2_etp_mux(&mut self) -> Adc2EtpMuxW<'_, RemapSpec> {
        Adc2EtpMuxW::new(self, 19)
    }
    #[doc = "Bit 20 - ADC2 external trigger ordinary conversion muxing"]
    #[inline(always)]
    pub fn adc2_eto_mux(&mut self) -> Adc2EtoMuxW<'_, RemapSpec> {
        Adc2EtoMuxW::new(self, 20)
    }
    #[doc = "Bit 21 - Ethernet MAC muxing"]
    #[inline(always)]
    pub fn emac_mux(&mut self) -> EmacMuxW<'_, RemapSpec> {
        EmacMuxW::new(self, 21)
    }
    #[doc = "Bit 22 - CAN2 muxing"]
    #[inline(always)]
    pub fn can2_mux(&mut self) -> Can2MuxW<'_, RemapSpec> {
        Can2MuxW::new(self, 22)
    }
    #[doc = "Bit 23 - MII_RMII select muxing"]
    #[inline(always)]
    pub fn mii_rmii_sel_mux(&mut self) -> MiiRmiiSelMuxW<'_, RemapSpec> {
        MiiRmiiSelMuxW::new(self, 23)
    }
    #[doc = "Bits 24:26 - SWD JTAG muxing"]
    #[inline(always)]
    pub fn swjtag_mux(&mut self) -> SwjtagMuxW<'_, RemapSpec> {
        SwjtagMuxW::new(self, 24)
    }
    #[doc = "Bit 28 - SPI3 muxing"]
    #[inline(always)]
    pub fn spi3_mux(&mut self) -> Spi3MuxW<'_, RemapSpec> {
        Spi3MuxW::new(self, 28)
    }
    #[doc = "Bit 29 - TMR2 internal trigger 1 muxing"]
    #[inline(always)]
    pub fn tmr2itr1_mux(&mut self) -> Tmr2itr1MuxW<'_, RemapSpec> {
        Tmr2itr1MuxW::new(self, 29)
    }
    #[doc = "Bit 30 - PTP_PPS muxing"]
    #[inline(always)]
    pub fn ptp_pps_mux(&mut self) -> PtpPpsMuxW<'_, RemapSpec> {
        PtpPpsMuxW::new(self, 30)
    }
    #[doc = "Bit 31 - SPI1 muxing bit1"]
    #[inline(always)]
    pub fn spi1_mux1(&mut self) -> Spi1Mux1W<'_, RemapSpec> {
        Spi1Mux1W::new(self, 31)
    }
}
#[doc = "IO MUX remap register (IOMUX_REMAP)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemapSpec;
impl crate::RegisterSpec for RemapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap::R`](R) reader structure"]
impl crate::Readable for RemapSpec {}
#[doc = "`write(|w| ..)` method takes [`remap::W`](W) writer structure"]
impl crate::Writable for RemapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP to value 0"]
impl crate::Resettable for RemapSpec {}
