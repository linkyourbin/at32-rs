#[doc = "Register `UNLOCK3` writer"]
pub type W = crate::W<Unlock3Spec>;
#[doc = "Field `UKVAL` writer - Unlock key value"]
pub type UkvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Unlock key value"]
    #[inline(always)]
    pub fn ukval(&mut self) -> UkvalW<'_, Unlock3Spec> {
        UkvalW::new(self, 0)
    }
}
#[doc = "Unlock 3 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`unlock3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Unlock3Spec;
impl crate::RegisterSpec for Unlock3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`unlock3::W`](W) writer structure"]
impl crate::Writable for Unlock3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UNLOCK3 to value 0"]
impl crate::Resettable for Unlock3Spec {}
