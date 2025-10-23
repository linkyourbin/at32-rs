#[doc = "Register `DT17` reader"]
pub type R = crate::R<Dt17Spec>;
#[doc = "Register `DT17` writer"]
pub type W = crate::W<Dt17Spec>;
#[doc = "Field `DT17` reader - BPR data17"]
pub type Dt17R = crate::FieldReader<u16>;
#[doc = "Field `DT17` writer - BPR data17"]
pub type Dt17W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data17"]
    #[inline(always)]
    pub fn dt17(&self) -> Dt17R {
        Dt17R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data17"]
    #[inline(always)]
    pub fn dt17(&mut self) -> Dt17W<'_, Dt17Spec> {
        Dt17W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt17Spec;
impl crate::RegisterSpec for Dt17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt17::R`](R) reader structure"]
impl crate::Readable for Dt17Spec {}
#[doc = "`write(|w| ..)` method takes [`dt17::W`](W) writer structure"]
impl crate::Writable for Dt17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT17 to value 0"]
impl crate::Resettable for Dt17Spec {}
