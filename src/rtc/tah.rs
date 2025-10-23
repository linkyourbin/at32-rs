#[doc = "Register `TAH` writer"]
pub type W = crate::W<TahSpec>;
#[doc = "Field `TA` writer - Time alarm register high"]
pub type TaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Time alarm register high"]
    #[inline(always)]
    pub fn ta(&mut self) -> TaW<'_, TahSpec> {
        TaW::new(self, 0)
    }
}
#[doc = "RTC Alarm Register High\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tah::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TahSpec;
impl crate::RegisterSpec for TahSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tah::W`](W) writer structure"]
impl crate::Writable for TahSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAH to value 0xffff"]
impl crate::Resettable for TahSpec {
    const RESET_VALUE: u32 = 0xffff;
}
