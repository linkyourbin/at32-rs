#[doc = "Register `REMAP8` reader"]
pub type R = crate::R<Remap8Spec>;
#[doc = "Register `REMAP8` writer"]
pub type W = crate::W<Remap8Spec>;
#[doc = "Field `EMAC_GMUX` reader - Ethernet MAC muxing"]
pub type EmacGmuxR = crate::FieldReader;
#[doc = "Field `EMAC_GMUX` writer - Ethernet MAC muxing"]
pub type EmacGmuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MII_RMII_SEL_GMUX` reader - MII_RMII select muxing"]
pub type MiiRmiiSelGmuxR = crate::BitReader;
#[doc = "Field `MII_RMII_SEL_GMUX` writer - MII_RMII select muxing"]
pub type MiiRmiiSelGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTP_PPS_GMUX` reader - PTP_PPS muxing"]
pub type PtpPpsGmuxR = crate::BitReader;
#[doc = "Field `PTP_PPS_GMUX` writer - PTP_PPS muxing"]
pub type PtpPpsGmuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6_GMUX` reader - USART6 muxing"]
pub type Usart6GmuxR = crate::FieldReader;
#[doc = "Field `USART6_GMUX` writer - USART6 muxing"]
pub type Usart6GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UART7_GMUX` reader - UART7 muxing"]
pub type Uart7GmuxR = crate::FieldReader;
#[doc = "Field `UART7_GMUX` writer - UART7 muxing"]
pub type Uart7GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UART8_GMUX` reader - UART8 muxing"]
pub type Uart8GmuxR = crate::FieldReader;
#[doc = "Field `UART8_GMUX` writer - UART8 muxing"]
pub type Uart8GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 16:17 - Ethernet MAC muxing"]
    #[inline(always)]
    pub fn emac_gmux(&self) -> EmacGmuxR {
        EmacGmuxR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - MII_RMII select muxing"]
    #[inline(always)]
    pub fn mii_rmii_sel_gmux(&self) -> MiiRmiiSelGmuxR {
        MiiRmiiSelGmuxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PTP_PPS muxing"]
    #[inline(always)]
    pub fn ptp_pps_gmux(&self) -> PtpPpsGmuxR {
        PtpPpsGmuxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - USART6 muxing"]
    #[inline(always)]
    pub fn usart6_gmux(&self) -> Usart6GmuxR {
        Usart6GmuxR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - UART7 muxing"]
    #[inline(always)]
    pub fn uart7_gmux(&self) -> Uart7GmuxR {
        Uart7GmuxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - UART8 muxing"]
    #[inline(always)]
    pub fn uart8_gmux(&self) -> Uart8GmuxR {
        Uart8GmuxR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Ethernet MAC muxing"]
    #[inline(always)]
    pub fn emac_gmux(&mut self) -> EmacGmuxW<'_, Remap8Spec> {
        EmacGmuxW::new(self, 16)
    }
    #[doc = "Bit 18 - MII_RMII select muxing"]
    #[inline(always)]
    pub fn mii_rmii_sel_gmux(&mut self) -> MiiRmiiSelGmuxW<'_, Remap8Spec> {
        MiiRmiiSelGmuxW::new(self, 18)
    }
    #[doc = "Bit 19 - PTP_PPS muxing"]
    #[inline(always)]
    pub fn ptp_pps_gmux(&mut self) -> PtpPpsGmuxW<'_, Remap8Spec> {
        PtpPpsGmuxW::new(self, 19)
    }
    #[doc = "Bits 20:23 - USART6 muxing"]
    #[inline(always)]
    pub fn usart6_gmux(&mut self) -> Usart6GmuxW<'_, Remap8Spec> {
        Usart6GmuxW::new(self, 20)
    }
    #[doc = "Bits 24:27 - UART7 muxing"]
    #[inline(always)]
    pub fn uart7_gmux(&mut self) -> Uart7GmuxW<'_, Remap8Spec> {
        Uart7GmuxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - UART8 muxing"]
    #[inline(always)]
    pub fn uart8_gmux(&mut self) -> Uart8GmuxW<'_, Remap8Spec> {
        Uart8GmuxW::new(self, 28)
    }
}
#[doc = "IO MUX remap register 8 (IOMUX_REMAP8)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap8Spec;
impl crate::RegisterSpec for Remap8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap8::R`](R) reader structure"]
impl crate::Readable for Remap8Spec {}
#[doc = "`write(|w| ..)` method takes [`remap8::W`](W) writer structure"]
impl crate::Writable for Remap8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP8 to value 0"]
impl crate::Resettable for Remap8Spec {}
