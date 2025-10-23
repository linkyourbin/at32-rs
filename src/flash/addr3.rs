#[doc = "Register `ADDR3` writer"]
pub type W = crate::W<Addr3Spec>;
#[doc = "Field `FA` writer - Flash Address"]
pub type FaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Flash Address"]
    #[inline(always)]
    pub fn fa(&mut self) -> FaW<'_, Addr3Spec> {
        FaW::new(self, 0)
    }
}
#[doc = "Address 3 register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr3Spec;
impl crate::RegisterSpec for Addr3Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`addr3::W`](W) writer structure"]
impl crate::Writable for Addr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR3 to value 0"]
impl crate::Resettable for Addr3Spec {}
