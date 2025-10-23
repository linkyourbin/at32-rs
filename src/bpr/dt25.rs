#[doc = "Register `DT25` reader"]
pub type R = crate::R<Dt25Spec>;
#[doc = "Register `DT25` writer"]
pub type W = crate::W<Dt25Spec>;
#[doc = "Field `DT25` reader - BPR data25"]
pub type Dt25R = crate::FieldReader<u16>;
#[doc = "Field `DT25` writer - BPR data25"]
pub type Dt25W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data25"]
    #[inline(always)]
    pub fn dt25(&self) -> Dt25R {
        Dt25R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data25"]
    #[inline(always)]
    pub fn dt25(&mut self) -> Dt25W<'_, Dt25Spec> {
        Dt25W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt25Spec;
impl crate::RegisterSpec for Dt25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt25::R`](R) reader structure"]
impl crate::Readable for Dt25Spec {}
#[doc = "`write(|w| ..)` method takes [`dt25::W`](W) writer structure"]
impl crate::Writable for Dt25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT25 to value 0"]
impl crate::Resettable for Dt25Spec {}
