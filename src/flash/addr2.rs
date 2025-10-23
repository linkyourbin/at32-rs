#[doc = "Register `ADDR2` writer"]
pub type W = crate::W<Addr2Spec>;
#[doc = "Field `FA` writer - Flash Address"]
pub type FaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash Address"]
    #[inline(always)]
    pub fn fa(&mut self) -> FaW<'_, Addr2Spec> {
        FaW::new(self, 0)
    }
}
#[doc = "Address 2 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr2Spec;
impl crate::RegisterSpec for Addr2Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`addr2::W`](W) writer structure"]
impl crate::Writable for Addr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR2 to value 0"]
impl crate::Resettable for Addr2Spec {}
