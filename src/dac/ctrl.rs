#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `D1EN` reader - DAC1 enable"]
pub type D1enR = crate::BitReader;
#[doc = "Field `D1EN` writer - DAC1 enable"]
pub type D1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1OBDIS` reader - DAC1 output buffer disable"]
pub type D1obdisR = crate::BitReader;
#[doc = "Field `D1OBDIS` writer - DAC1 output buffer disable"]
pub type D1obdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1TRGEN` reader - DAC1 trigger enable"]
pub type D1trgenR = crate::BitReader;
#[doc = "Field `D1TRGEN` writer - DAC1 trigger enable"]
pub type D1trgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1TRGSEL` reader - DAC1 trigger selection"]
pub type D1trgselR = crate::FieldReader;
#[doc = "Field `D1TRGSEL` writer - DAC1 trigger selection"]
pub type D1trgselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D1NM` reader - DAC1 noise/triangle wave generation enable"]
pub type D1nmR = crate::FieldReader;
#[doc = "Field `D1NM` writer - DAC1 noise/triangle wave generation enable"]
pub type D1nmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D1NBSEL` reader - DAC1 mask/amplitude selector"]
pub type D1nbselR = crate::FieldReader;
#[doc = "Field `D1NBSEL` writer - DAC1 mask/amplitude selector"]
pub type D1nbselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D1DMAEN` reader - DAC1 DMA enable"]
pub type D1dmaenR = crate::BitReader;
#[doc = "Field `D1DMAEN` writer - DAC1 DMA enable"]
pub type D1dmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2EN` reader - DAC2 enable"]
pub type D2enR = crate::BitReader;
#[doc = "Field `D2EN` writer - DAC2 enable"]
pub type D2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2OBDIS` reader - DAC2 output buffer disable"]
pub type D2obdisR = crate::BitReader;
#[doc = "Field `D2OBDIS` writer - DAC2 output buffer disable"]
pub type D2obdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2TRGEN` reader - DAC2 trigger enable"]
pub type D2trgenR = crate::BitReader;
#[doc = "Field `D2TRGEN` writer - DAC2 trigger enable"]
pub type D2trgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2TRGSEL` reader - DAC2 trigger selection"]
pub type D2trgselR = crate::FieldReader;
#[doc = "Field `D2TRGSEL` writer - DAC2 trigger selection"]
pub type D2trgselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D2NM` reader - DAC2 noise/triangle wave generation enable"]
pub type D2nmR = crate::FieldReader;
#[doc = "Field `D2NM` writer - DAC2 noise/triangle wave generation enable"]
pub type D2nmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `D2NBSEL` reader - DAC2 mask/amplitude selector"]
pub type D2nbselR = crate::FieldReader;
#[doc = "Field `D2NBSEL` writer - DAC2 mask/amplitude selector"]
pub type D2nbselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D2DMAEN` reader - DAC2 DMA enable"]
pub type D2dmaenR = crate::BitReader;
#[doc = "Field `D2DMAEN` writer - DAC2 DMA enable"]
pub type D2dmaenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC1 enable"]
    #[inline(always)]
    pub fn d1en(&self) -> D1enR {
        D1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC1 output buffer disable"]
    #[inline(always)]
    pub fn d1obdis(&self) -> D1obdisR {
        D1obdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn d1trgen(&self) -> D1trgenR {
        D1trgenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn d1trgsel(&self) -> D1trgselR {
        D1trgselR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - DAC1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn d1nm(&self) -> D1nmR {
        D1nmR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - DAC1 mask/amplitude selector"]
    #[inline(always)]
    pub fn d1nbsel(&self) -> D1nbselR {
        D1nbselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn d1dmaen(&self) -> D1dmaenR {
        D1dmaenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - DAC2 enable"]
    #[inline(always)]
    pub fn d2en(&self) -> D2enR {
        D2enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DAC2 output buffer disable"]
    #[inline(always)]
    pub fn d2obdis(&self) -> D2obdisR {
        D2obdisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DAC2 trigger enable"]
    #[inline(always)]
    pub fn d2trgen(&self) -> D2trgenR {
        D2trgenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - DAC2 trigger selection"]
    #[inline(always)]
    pub fn d2trgsel(&self) -> D2trgselR {
        D2trgselR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - DAC2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn d2nm(&self) -> D2nmR {
        D2nmR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - DAC2 mask/amplitude selector"]
    #[inline(always)]
    pub fn d2nbsel(&self) -> D2nbselR {
        D2nbselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - DAC2 DMA enable"]
    #[inline(always)]
    pub fn d2dmaen(&self) -> D2dmaenR {
        D2dmaenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC1 enable"]
    #[inline(always)]
    pub fn d1en(&mut self) -> D1enW<'_, CtrlSpec> {
        D1enW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC1 output buffer disable"]
    #[inline(always)]
    pub fn d1obdis(&mut self) -> D1obdisW<'_, CtrlSpec> {
        D1obdisW::new(self, 1)
    }
    #[doc = "Bit 2 - DAC1 trigger enable"]
    #[inline(always)]
    pub fn d1trgen(&mut self) -> D1trgenW<'_, CtrlSpec> {
        D1trgenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - DAC1 trigger selection"]
    #[inline(always)]
    pub fn d1trgsel(&mut self) -> D1trgselW<'_, CtrlSpec> {
        D1trgselW::new(self, 3)
    }
    #[doc = "Bits 6:7 - DAC1 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn d1nm(&mut self) -> D1nmW<'_, CtrlSpec> {
        D1nmW::new(self, 6)
    }
    #[doc = "Bits 8:11 - DAC1 mask/amplitude selector"]
    #[inline(always)]
    pub fn d1nbsel(&mut self) -> D1nbselW<'_, CtrlSpec> {
        D1nbselW::new(self, 8)
    }
    #[doc = "Bit 12 - DAC1 DMA enable"]
    #[inline(always)]
    pub fn d1dmaen(&mut self) -> D1dmaenW<'_, CtrlSpec> {
        D1dmaenW::new(self, 12)
    }
    #[doc = "Bit 16 - DAC2 enable"]
    #[inline(always)]
    pub fn d2en(&mut self) -> D2enW<'_, CtrlSpec> {
        D2enW::new(self, 16)
    }
    #[doc = "Bit 17 - DAC2 output buffer disable"]
    #[inline(always)]
    pub fn d2obdis(&mut self) -> D2obdisW<'_, CtrlSpec> {
        D2obdisW::new(self, 17)
    }
    #[doc = "Bit 18 - DAC2 trigger enable"]
    #[inline(always)]
    pub fn d2trgen(&mut self) -> D2trgenW<'_, CtrlSpec> {
        D2trgenW::new(self, 18)
    }
    #[doc = "Bits 19:21 - DAC2 trigger selection"]
    #[inline(always)]
    pub fn d2trgsel(&mut self) -> D2trgselW<'_, CtrlSpec> {
        D2trgselW::new(self, 19)
    }
    #[doc = "Bits 22:23 - DAC2 noise/triangle wave generation enable"]
    #[inline(always)]
    pub fn d2nm(&mut self) -> D2nmW<'_, CtrlSpec> {
        D2nmW::new(self, 22)
    }
    #[doc = "Bits 24:27 - DAC2 mask/amplitude selector"]
    #[inline(always)]
    pub fn d2nbsel(&mut self) -> D2nbselW<'_, CtrlSpec> {
        D2nbselW::new(self, 24)
    }
    #[doc = "Bit 28 - DAC2 DMA enable"]
    #[inline(always)]
    pub fn d2dmaen(&mut self) -> D2dmaenW<'_, CtrlSpec> {
        D2dmaenW::new(self, 28)
    }
}
#[doc = "Control register (DAC_CTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
