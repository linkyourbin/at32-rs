#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `TPEN` reader - Tamper pin enable"]
pub type TpenR = crate::BitReader;
#[doc = "Field `TPEN` writer - Tamper pin enable"]
pub type TpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPP` reader - TAMPER pin polarity"]
pub type TppR = crate::BitReader;
#[doc = "Field `TPP` writer - TAMPER pin polarity"]
pub type TppW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpen(&self) -> TpenR {
        TpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMPER pin polarity"]
    #[inline(always)]
    pub fn tpp(&self) -> TppR {
        TppR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper pin enable"]
    #[inline(always)]
    pub fn tpen(&mut self) -> TpenW<'_, CtrlSpec> {
        TpenW::new(self, 0)
    }
    #[doc = "Bit 1 - TAMPER pin polarity"]
    #[inline(always)]
    pub fn tpp(&mut self) -> TppW<'_, CtrlSpec> {
        TppW::new(self, 1)
    }
}
#[doc = "BPR control register (BPR_CTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
