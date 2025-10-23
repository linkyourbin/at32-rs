#[doc = "Register `UNLOCK2` writer"]
pub type W = crate::W<Unlock2Spec>;
#[doc = "Field `UKVAL` writer - Unlock key value"]
pub type UkvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Unlock key value"]
    #[inline(always)]
    pub fn ukval(&mut self) -> UkvalW<'_, Unlock2Spec> {
        UkvalW::new(self, 0)
    }
}
#[doc = "Unlock 2 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unlock2Spec;
impl crate::RegisterSpec for Unlock2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unlock2::W`](W) writer structure"]
impl crate::Writable for Unlock2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNLOCK2 to value 0"]
impl crate::Resettable for Unlock2Spec {}
