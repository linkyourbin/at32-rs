#[doc = "Register `DMA_SRC_SEL1` reader"]
pub type R = crate::R<DmaSrcSel1Spec>;
#[doc = "Register `DMA_SRC_SEL1` writer"]
pub type W = crate::W<DmaSrcSel1Spec>;
#[doc = "Field `CH5_SRC` reader - CH5 SRC select"]
pub type Ch5SrcR = crate::FieldReader;
#[doc = "Field `CH5_SRC` writer - CH5 SRC select"]
pub type Ch5SrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH6_SRC` reader - CH6 SRC select"]
pub type Ch6SrcR = crate::FieldReader;
#[doc = "Field `CH6_SRC` writer - CH6 SRC select"]
pub type Ch6SrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CH7_SRC` reader - CH7 SRC select"]
pub type Ch7SrcR = crate::FieldReader;
#[doc = "Field `CH7_SRC` writer - CH7 SRC select"]
pub type Ch7SrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMA_FLEX_EN` reader - DMA FLEX Enable"]
pub type DmaFlexEnR = crate::BitReader;
#[doc = "Field `DMA_FLEX_EN` writer - DMA FLEX Enable"]
pub type DmaFlexEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - CH5 SRC select"]
    #[inline(always)]
    pub fn ch5_src(&self) -> Ch5SrcR {
        Ch5SrcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CH6 SRC select"]
    #[inline(always)]
    pub fn ch6_src(&self) -> Ch6SrcR {
        Ch6SrcR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CH7 SRC select"]
    #[inline(always)]
    pub fn ch7_src(&self) -> Ch7SrcR {
        Ch7SrcR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - DMA FLEX Enable"]
    #[inline(always)]
    pub fn dma_flex_en(&self) -> DmaFlexEnR {
        DmaFlexEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - CH5 SRC select"]
    #[inline(always)]
    pub fn ch5_src(&mut self) -> Ch5SrcW<'_, DmaSrcSel1Spec> {
        Ch5SrcW::new(self, 0)
    }
    #[doc = "Bits 8:15 - CH6 SRC select"]
    #[inline(always)]
    pub fn ch6_src(&mut self) -> Ch6SrcW<'_, DmaSrcSel1Spec> {
        Ch6SrcW::new(self, 8)
    }
    #[doc = "Bits 16:23 - CH7 SRC select"]
    #[inline(always)]
    pub fn ch7_src(&mut self) -> Ch7SrcW<'_, DmaSrcSel1Spec> {
        Ch7SrcW::new(self, 16)
    }
    #[doc = "Bit 24 - DMA FLEX Enable"]
    #[inline(always)]
    pub fn dma_flex_en(&mut self) -> DmaFlexEnW<'_, DmaSrcSel1Spec> {
        DmaFlexEnW::new(self, 24)
    }
}
#[doc = "DMA channel source assignment register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_src_sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_src_sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSrcSel1Spec;
impl crate::RegisterSpec for DmaSrcSel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_src_sel1::R`](R) reader structure"]
impl crate::Readable for DmaSrcSel1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma_src_sel1::W`](W) writer structure"]
impl crate::Writable for DmaSrcSel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_SRC_SEL1 to value 0"]
impl crate::Resettable for DmaSrcSel1Spec {}
