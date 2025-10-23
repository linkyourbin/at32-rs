#[doc = "Register `DT28` reader"]
pub type R = crate::R<Dt28Spec>;
#[doc = "Register `DT28` writer"]
pub type W = crate::W<Dt28Spec>;
#[doc = "Field `DT28` reader - BPR data28"]
pub type Dt28R = crate::FieldReader<u16>;
#[doc = "Field `DT28` writer - BPR data28"]
pub type Dt28W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data28"]
    #[inline(always)]
    pub fn dt28(&self) -> Dt28R {
        Dt28R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data28"]
    #[inline(always)]
    pub fn dt28(&mut self) -> Dt28W<'_, Dt28Spec> {
        Dt28W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt28Spec;
impl crate::RegisterSpec for Dt28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt28::R`](R) reader structure"]
impl crate::Readable for Dt28Spec {}
#[doc = "`write(|w| ..)` method takes [`dt28::W`](W) writer structure"]
impl crate::Writable for Dt28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT28 to value 0"]
impl crate::Resettable for Dt28Spec {}
