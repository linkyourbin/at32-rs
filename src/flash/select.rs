#[doc = "Register `SELECT` writer"]
pub type W = crate::W<SelectSpec>;
#[doc = "Field `SELECT` writer - spim type selection"]
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - spim type selection"]
    #[inline(always)]
    pub fn select(&mut self) -> SelectW<'_, SelectSpec> {
        SelectW::new(self, 0)
    }
}
#[doc = "Select register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`select::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SelectSpec;
impl crate::RegisterSpec for SelectSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`select::W`](W) writer structure"]
impl crate::Writable for SelectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SELECT to value 0"]
impl crate::Resettable for SelectSpec {}
