#[doc = "Register `DT3` reader"]
pub type R = crate::R<Dt3Spec>;
#[doc = "Register `DT3` writer"]
pub type W = crate::W<Dt3Spec>;
#[doc = "Field `DT3` reader - BPR data3"]
pub type Dt3R = crate::FieldReader<u16>;
#[doc = "Field `DT3` writer - BPR data3"]
pub type Dt3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data3"]
    #[inline(always)]
    pub fn dt3(&self) -> Dt3R {
        Dt3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data3"]
    #[inline(always)]
    pub fn dt3(&mut self) -> Dt3W<'_, Dt3Spec> {
        Dt3W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt3Spec;
impl crate::RegisterSpec for Dt3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt3::R`](R) reader structure"]
impl crate::Readable for Dt3Spec {}
#[doc = "`write(|w| ..)` method takes [`dt3::W`](W) writer structure"]
impl crate::Writable for Dt3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT3 to value 0"]
impl crate::Resettable for Dt3Spec {}
