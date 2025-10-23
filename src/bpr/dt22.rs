#[doc = "Register `DT22` reader"]
pub type R = crate::R<Dt22Spec>;
#[doc = "Register `DT22` writer"]
pub type W = crate::W<Dt22Spec>;
#[doc = "Field `DT22` reader - BPR data22"]
pub type Dt22R = crate::FieldReader<u16>;
#[doc = "Field `DT22` writer - BPR data22"]
pub type Dt22W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data22"]
    #[inline(always)]
    pub fn dt22(&self) -> Dt22R {
        Dt22R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data22"]
    #[inline(always)]
    pub fn dt22(&mut self) -> Dt22W<'_, Dt22Spec> {
        Dt22W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt22Spec;
impl crate::RegisterSpec for Dt22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt22::R`](R) reader structure"]
impl crate::Readable for Dt22Spec {}
#[doc = "`write(|w| ..)` method takes [`dt22::W`](W) writer structure"]
impl crate::Writable for Dt22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT22 to value 0"]
impl crate::Resettable for Dt22Spec {}
