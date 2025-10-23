#[doc = "Register `DT19` reader"]
pub type R = crate::R<Dt19Spec>;
#[doc = "Register `DT19` writer"]
pub type W = crate::W<Dt19Spec>;
#[doc = "Field `DT19` reader - BPR data19"]
pub type Dt19R = crate::FieldReader<u16>;
#[doc = "Field `DT19` writer - BPR data19"]
pub type Dt19W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data19"]
    #[inline(always)]
    pub fn dt19(&self) -> Dt19R {
        Dt19R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data19"]
    #[inline(always)]
    pub fn dt19(&mut self) -> Dt19W<'_, Dt19Spec> {
        Dt19W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt19Spec;
impl crate::RegisterSpec for Dt19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt19::R`](R) reader structure"]
impl crate::Readable for Dt19Spec {}
#[doc = "`write(|w| ..)` method takes [`dt19::W`](W) writer structure"]
impl crate::Writable for Dt19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT19 to value 0"]
impl crate::Resettable for Dt19Spec {}
