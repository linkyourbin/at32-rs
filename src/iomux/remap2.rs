#[doc = "Register `REMAP2` reader"]
pub type R = crate::R<Remap2Spec>;
#[doc = "Register `REMAP2` writer"]
pub type W = crate::W<Remap2Spec>;
#[doc = "Field `TMR9_MUX` reader - TMR9 muxing"]
pub type Tmr9MuxR = crate::BitReader;
#[doc = "Field `TMR9_MUX` writer - TMR9 muxing"]
pub type Tmr9MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMC_NADV_MUX` reader - NADV connect/disconnect"]
pub type XmcNadvMuxR = crate::BitReader;
#[doc = "Field `XMC_NADV_MUX` writer - NADV connect/disconnect"]
pub type XmcNadvMuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4_MUX` reader - SPI4 muxing"]
pub type Spi4MuxR = crate::BitReader;
#[doc = "Field `SPI4_MUX` writer - SPI4 muxing"]
pub type Spi4MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3_MUX` reader - I2C3 muxing"]
pub type I2c3MuxR = crate::BitReader;
#[doc = "Field `I2C3_MUX` writer - I2C3 muxing"]
pub type I2c3MuxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO2_MUX` reader - SDIO2 muxing"]
pub type Sdio2MuxR = crate::FieldReader;
#[doc = "Field `SDIO2_MUX` writer - SDIO2 muxing"]
pub type Sdio2MuxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXT_SPIM_EN_MUX` reader - SPIM enable muxing"]
pub type ExtSpimEnMuxR = crate::BitReader;
#[doc = "Field `EXT_SPIM_EN_MUX` writer - SPIM enable muxing"]
pub type ExtSpimEnMuxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_mux(&self) -> Tmr9MuxR {
        Tmr9MuxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn xmc_nadv_mux(&self) -> XmcNadvMuxR {
        XmcNadvMuxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 17 - SPI4 muxing"]
    #[inline(always)]
    pub fn spi4_mux(&self) -> Spi4MuxR {
        Spi4MuxR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I2C3 muxing"]
    #[inline(always)]
    pub fn i2c3_mux(&self) -> I2c3MuxR {
        I2c3MuxR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - SDIO2 muxing"]
    #[inline(always)]
    pub fn sdio2_mux(&self) -> Sdio2MuxR {
        Sdio2MuxR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - SPIM enable muxing"]
    #[inline(always)]
    pub fn ext_spim_en_mux(&self) -> ExtSpimEnMuxR {
        ExtSpimEnMuxR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_mux(&mut self) -> Tmr9MuxW<'_, Remap2Spec> {
        Tmr9MuxW::new(self, 5)
    }
    #[doc = "Bit 10 - NADV connect/disconnect"]
    #[inline(always)]
    pub fn xmc_nadv_mux(&mut self) -> XmcNadvMuxW<'_, Remap2Spec> {
        XmcNadvMuxW::new(self, 10)
    }
    #[doc = "Bit 17 - SPI4 muxing"]
    #[inline(always)]
    pub fn spi4_mux(&mut self) -> Spi4MuxW<'_, Remap2Spec> {
        Spi4MuxW::new(self, 17)
    }
    #[doc = "Bit 18 - I2C3 muxing"]
    #[inline(always)]
    pub fn i2c3_mux(&mut self) -> I2c3MuxW<'_, Remap2Spec> {
        I2c3MuxW::new(self, 18)
    }
    #[doc = "Bits 19:20 - SDIO2 muxing"]
    #[inline(always)]
    pub fn sdio2_mux(&mut self) -> Sdio2MuxW<'_, Remap2Spec> {
        Sdio2MuxW::new(self, 19)
    }
    #[doc = "Bit 21 - SPIM enable muxing"]
    #[inline(always)]
    pub fn ext_spim_en_mux(&mut self) -> ExtSpimEnMuxW<'_, Remap2Spec> {
        ExtSpimEnMuxW::new(self, 21)
    }
}
#[doc = "IO MUX remap register 2 (IOMUX_REMAP2)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap2Spec;
impl crate::RegisterSpec for Remap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap2::R`](R) reader structure"]
impl crate::Readable for Remap2Spec {}
#[doc = "`write(|w| ..)` method takes [`remap2::W`](W) writer structure"]
impl crate::Writable for Remap2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP2 to value 0"]
impl crate::Resettable for Remap2Spec {}
