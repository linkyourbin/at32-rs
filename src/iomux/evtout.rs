#[doc = "Register `EVTOUT` reader"]
pub type R = crate::R<EvtoutSpec>;
#[doc = "Register `EVTOUT` writer"]
pub type W = crate::W<EvtoutSpec>;
#[doc = "Field `SELPIN` reader - Select pin"]
pub type SelpinR = crate::FieldReader;
#[doc = "Field `SELPIN` writer - Select pin"]
pub type SelpinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SELPORT` reader - Select port"]
pub type SelportR = crate::FieldReader;
#[doc = "Field `SELPORT` writer - Select port"]
pub type SelportW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EVOEN` reader - Event output enable"]
pub type EvoenR = crate::BitReader;
#[doc = "Field `EVOEN` writer - Event output enable"]
pub type EvoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    pub fn selpin(&self) -> SelpinR {
        SelpinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    pub fn selport(&self) -> SelportR {
        SelportR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn evoen(&self) -> EvoenR {
        EvoenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    pub fn selpin(&mut self) -> SelpinW<'_, EvtoutSpec> {
        SelpinW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    pub fn selport(&mut self) -> SelportW<'_, EvtoutSpec> {
        SelportW::new(self, 4)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn evoen(&mut self) -> EvoenW<'_, EvtoutSpec> {
        EvoenW::new(self, 7)
    }
}
#[doc = "Event output register (IOMUX_EVTOUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`evtout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evtout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtoutSpec;
impl crate::RegisterSpec for EvtoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtout::R`](R) reader structure"]
impl crate::Readable for EvtoutSpec {}
#[doc = "`write(|w| ..)` method takes [`evtout::W`](W) writer structure"]
impl crate::Writable for EvtoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVTOUT to value 0"]
impl crate::Resettable for EvtoutSpec {}
