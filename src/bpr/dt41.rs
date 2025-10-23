#[doc = "Register `DT41` reader"]
pub type R = crate::R<Dt41Spec>;
#[doc = "Register `DT41` writer"]
pub type W = crate::W<Dt41Spec>;
#[doc = "Field `DT41` reader - BPR data41"]
pub type Dt41R = crate::FieldReader<u16>;
#[doc = "Field `DT41` writer - BPR data41"]
pub type Dt41W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data41"]
    #[inline(always)]
    pub fn dt41(&self) -> Dt41R {
        Dt41R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data41"]
    #[inline(always)]
    pub fn dt41(&mut self) -> Dt41W<'_, Dt41Spec> {
        Dt41W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt41Spec;
impl crate::RegisterSpec for Dt41Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt41::R`](R) reader structure"]
impl crate::Readable for Dt41Spec {}
#[doc = "`write(|w| ..)` method takes [`dt41::W`](W) writer structure"]
impl crate::Writable for Dt41Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT41 to value 0"]
impl crate::Resettable for Dt41Spec {}
