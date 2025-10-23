#[doc = "Register `DT6` reader"]
pub type R = crate::R<Dt6Spec>;
#[doc = "Register `DT6` writer"]
pub type W = crate::W<Dt6Spec>;
#[doc = "Field `DT6` reader - BPR data6"]
pub type Dt6R = crate::FieldReader<u16>;
#[doc = "Field `DT6` writer - BPR data6"]
pub type Dt6W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data6"]
    #[inline(always)]
    pub fn dt6(&self) -> Dt6R {
        Dt6R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data6"]
    #[inline(always)]
    pub fn dt6(&mut self) -> Dt6W<'_, Dt6Spec> {
        Dt6W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt6Spec;
impl crate::RegisterSpec for Dt6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt6::R`](R) reader structure"]
impl crate::Readable for Dt6Spec {}
#[doc = "`write(|w| ..)` method takes [`dt6::W`](W) writer structure"]
impl crate::Writable for Dt6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT6 to value 0"]
impl crate::Resettable for Dt6Spec {}
