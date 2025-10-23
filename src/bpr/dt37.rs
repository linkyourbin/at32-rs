#[doc = "Register `DT37` reader"]
pub type R = crate::R<Dt37Spec>;
#[doc = "Register `DT37` writer"]
pub type W = crate::W<Dt37Spec>;
#[doc = "Field `DT37` reader - BPR data37"]
pub type Dt37R = crate::FieldReader<u16>;
#[doc = "Field `DT37` writer - BPR data37"]
pub type Dt37W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data37"]
    #[inline(always)]
    pub fn dt37(&self) -> Dt37R {
        Dt37R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data37"]
    #[inline(always)]
    pub fn dt37(&mut self) -> Dt37W<'_, Dt37Spec> {
        Dt37W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt37Spec;
impl crate::RegisterSpec for Dt37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt37::R`](R) reader structure"]
impl crate::Readable for Dt37Spec {}
#[doc = "`write(|w| ..)` method takes [`dt37::W`](W) writer structure"]
impl crate::Writable for Dt37Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT37 to value 0"]
impl crate::Resettable for Dt37Spec {}
