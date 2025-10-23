#[doc = "Register `REMAP5` reader"]
pub type R = crate::R<Remap5Spec>;
#[doc = "Register `REMAP5` writer"]
pub type W = crate::W<Remap5Spec>;
#[doc = "Field `USART5_GMUX` reader - USART5 muxing"]
pub type Usart5GmuxR = crate::FieldReader;
#[doc = "Field `USART5_GMUX` writer - USART5 muxing"]
pub type Usart5GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2C1_GMUX` reader - I2C1 muxing"]
pub type I2c1GmuxR = crate::FieldReader;
#[doc = "Field `I2C1_GMUX` writer - I2C1 muxing"]
pub type I2c1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2C3_GMUX` reader - I2C3 muxing"]
pub type I2c3GmuxR = crate::FieldReader;
#[doc = "Field `I2C3_GMUX` writer - I2C3 muxing"]
pub type I2c3GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI1_GMUX` reader - SPI1 muxing"]
pub type Spi1GmuxR = crate::FieldReader;
#[doc = "Field `SPI1_GMUX` writer - SPI1 muxing"]
pub type Spi1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI2_GMUX` reader - SPI2 muxing"]
pub type Spi2GmuxR = crate::FieldReader;
#[doc = "Field `SPI2_GMUX` writer - SPI2 muxing"]
pub type Spi2GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI3_GMUX` reader - SPI3 muxing"]
pub type Spi3GmuxR = crate::FieldReader;
#[doc = "Field `SPI3_GMUX` writer - SPI3 muxing"]
pub type Spi3GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPI4_GMUX` reader - SPI4 muxing"]
pub type Spi4GmuxR = crate::FieldReader;
#[doc = "Field `SPI4_GMUX` writer - SPI4 muxing"]
pub type Spi4GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - USART5 muxing"]
    #[inline(always)]
    pub fn usart5_gmux(&self) -> Usart5GmuxR {
        Usart5GmuxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_gmux(&self) -> I2c1GmuxR {
        I2c1GmuxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - I2C3 muxing"]
    #[inline(always)]
    pub fn i2c3_gmux(&self) -> I2c3GmuxR {
        I2c3GmuxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SPI1 muxing"]
    #[inline(always)]
    pub fn spi1_gmux(&self) -> Spi1GmuxR {
        Spi1GmuxR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    pub fn spi2_gmux(&self) -> Spi2GmuxR {
        Spi2GmuxR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SPI3 muxing"]
    #[inline(always)]
    pub fn spi3_gmux(&self) -> Spi3GmuxR {
        Spi3GmuxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SPI4 muxing"]
    #[inline(always)]
    pub fn spi4_gmux(&self) -> Spi4GmuxR {
        Spi4GmuxR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART5 muxing"]
    #[inline(always)]
    pub fn usart5_gmux(&mut self) -> Usart5GmuxW<'_, Remap5Spec> {
        Usart5GmuxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - I2C1 muxing"]
    #[inline(always)]
    pub fn i2c1_gmux(&mut self) -> I2c1GmuxW<'_, Remap5Spec> {
        I2c1GmuxW::new(self, 4)
    }
    #[doc = "Bits 12:15 - I2C3 muxing"]
    #[inline(always)]
    pub fn i2c3_gmux(&mut self) -> I2c3GmuxW<'_, Remap5Spec> {
        I2c3GmuxW::new(self, 12)
    }
    #[doc = "Bits 16:19 - SPI1 muxing"]
    #[inline(always)]
    pub fn spi1_gmux(&mut self) -> Spi1GmuxW<'_, Remap5Spec> {
        Spi1GmuxW::new(self, 16)
    }
    #[doc = "Bits 20:23 - SPI2 muxing"]
    #[inline(always)]
    pub fn spi2_gmux(&mut self) -> Spi2GmuxW<'_, Remap5Spec> {
        Spi2GmuxW::new(self, 20)
    }
    #[doc = "Bits 24:27 - SPI3 muxing"]
    #[inline(always)]
    pub fn spi3_gmux(&mut self) -> Spi3GmuxW<'_, Remap5Spec> {
        Spi3GmuxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - SPI4 muxing"]
    #[inline(always)]
    pub fn spi4_gmux(&mut self) -> Spi4GmuxW<'_, Remap5Spec> {
        Spi4GmuxW::new(self, 28)
    }
}
#[doc = "IO MUX remap register 5 (IOMUX_REMAP5)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap5Spec;
impl crate::RegisterSpec for Remap5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap5::R`](R) reader structure"]
impl crate::Readable for Remap5Spec {}
#[doc = "`write(|w| ..)` method takes [`remap5::W`](W) writer structure"]
impl crate::Writable for Remap5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP5 to value 0"]
impl crate::Resettable for Remap5Spec {}
