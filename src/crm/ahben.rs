#[doc = "Register `AHBEN` reader"]
pub type R = crate::R<AhbenSpec>;
#[doc = "Register `AHBEN` writer"]
pub type W = crate::W<AhbenSpec>;
#[doc = "Field `DMA1EN` reader - DMA1 clock enable"]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 clock enable"]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2 clock enable"]
pub type Dma2enR = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 clock enable"]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAMEN` reader - SRAM interface clock enable"]
pub type SramenR = crate::BitReader;
#[doc = "Field `SRAMEN` writer - SRAM interface clock enable"]
pub type SramenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHEN` reader - FLASH clock enable"]
pub type FlashenR = crate::BitReader;
#[doc = "Field `FLASHEN` writer - FLASH clock enable"]
pub type FlashenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XMCEN` reader - XMC clock enable"]
pub type XmcenR = crate::BitReader;
#[doc = "Field `XMCEN` writer - XMC clock enable"]
pub type XmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO1EN` reader - SDIO1 clock enable"]
pub type Sdio1enR = crate::BitReader;
#[doc = "Field `SDIO1EN` writer - SDIO1 clock enable"]
pub type Sdio1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO2EN` reader - SDIO2 clock enable"]
pub type Sdio2enR = crate::BitReader;
#[doc = "Field `SDIO2EN` writer - SDIO2 clock enable"]
pub type Sdio2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACEN` reader - EMACEN clock enable"]
pub type EmacenR = crate::BitReader;
#[doc = "Field `EMACEN` writer - EMACEN clock enable"]
pub type EmacenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACTXEN` reader - EMACEN Tx clock enable"]
pub type EmactxenR = crate::BitReader;
#[doc = "Field `EMACTXEN` writer - EMACEN Tx clock enable"]
pub type EmactxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACRXEN` reader - EMACEN Rx clock enable"]
pub type EmacrxenR = crate::BitReader;
#[doc = "Field `EMACRXEN` writer - EMACEN Rx clock enable"]
pub type EmacrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMACPTPEN` reader - EMACPTP clock enable"]
pub type EmacptpenR = crate::BitReader;
#[doc = "Field `EMACPTPEN` writer - EMACPTP clock enable"]
pub type EmacptpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&self) -> SramenR {
        SramenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flashen(&self) -> FlashenR {
        FlashenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - XMC clock enable"]
    #[inline(always)]
    pub fn xmcen(&self) -> XmcenR {
        XmcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1en(&self) -> Sdio1enR {
        Sdio1enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO2 clock enable"]
    #[inline(always)]
    pub fn sdio2en(&self) -> Sdio2enR {
        Sdio2enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - EMACEN clock enable"]
    #[inline(always)]
    pub fn emacen(&self) -> EmacenR {
        EmacenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - EMACEN Tx clock enable"]
    #[inline(always)]
    pub fn emactxen(&self) -> EmactxenR {
        EmactxenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EMACEN Rx clock enable"]
    #[inline(always)]
    pub fn emacrxen(&self) -> EmacrxenR {
        EmacrxenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - EMACPTP clock enable"]
    #[inline(always)]
    pub fn emacptpen(&self) -> EmacptpenR {
        EmacptpenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 clock enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> Dma1enW<'_, AhbenSpec> {
        Dma1enW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> Dma2enW<'_, AhbenSpec> {
        Dma2enW::new(self, 1)
    }
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sramen(&mut self) -> SramenW<'_, AhbenSpec> {
        SramenW::new(self, 2)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flashen(&mut self) -> FlashenW<'_, AhbenSpec> {
        FlashenW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, AhbenSpec> {
        CrcenW::new(self, 6)
    }
    #[doc = "Bit 8 - XMC clock enable"]
    #[inline(always)]
    pub fn xmcen(&mut self) -> XmcenW<'_, AhbenSpec> {
        XmcenW::new(self, 8)
    }
    #[doc = "Bit 10 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1en(&mut self) -> Sdio1enW<'_, AhbenSpec> {
        Sdio1enW::new(self, 10)
    }
    #[doc = "Bit 11 - SDIO2 clock enable"]
    #[inline(always)]
    pub fn sdio2en(&mut self) -> Sdio2enW<'_, AhbenSpec> {
        Sdio2enW::new(self, 11)
    }
    #[doc = "Bit 14 - EMACEN clock enable"]
    #[inline(always)]
    pub fn emacen(&mut self) -> EmacenW<'_, AhbenSpec> {
        EmacenW::new(self, 14)
    }
    #[doc = "Bit 15 - EMACEN Tx clock enable"]
    #[inline(always)]
    pub fn emactxen(&mut self) -> EmactxenW<'_, AhbenSpec> {
        EmactxenW::new(self, 15)
    }
    #[doc = "Bit 16 - EMACEN Rx clock enable"]
    #[inline(always)]
    pub fn emacrxen(&mut self) -> EmacrxenW<'_, AhbenSpec> {
        EmacrxenW::new(self, 16)
    }
    #[doc = "Bit 28 - EMACPTP clock enable"]
    #[inline(always)]
    pub fn emacptpen(&mut self) -> EmacptpenW<'_, AhbenSpec> {
        EmacptpenW::new(self, 28)
    }
}
#[doc = "AHB Peripheral Clock enable register (CRM_AHBEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenSpec;
impl crate::RegisterSpec for AhbenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben::R`](R) reader structure"]
impl crate::Readable for AhbenSpec {}
#[doc = "`write(|w| ..)` method takes [`ahben::W`](W) writer structure"]
impl crate::Writable for AhbenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBEN to value 0x14"]
impl crate::Resettable for AhbenSpec {
    const RESET_VALUE: u32 = 0x14;
}
