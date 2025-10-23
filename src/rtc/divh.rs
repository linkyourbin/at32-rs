#[doc = "Register `DIVH` writer"]
pub type W = crate::W<DivhSpec>;
#[doc = "Field `DIV` writer - RTC divider high"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl W {
    #[doc = "Bits 0:3 - RTC divider high"]
    #[inline(always)]
    pub fn div(&mut self) -> DivW<'_, DivhSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "RTC Divider Register High\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divh::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivhSpec;
impl crate::RegisterSpec for DivhSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`divh::W`](W) writer structure"]
impl crate::Writable for DivhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVH to value 0"]
impl crate::Resettable for DivhSpec {}
