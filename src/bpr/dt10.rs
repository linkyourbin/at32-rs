#[doc = "Register `DT10` reader"]
pub type R = crate::R<Dt10Spec>;
#[doc = "Register `DT10` writer"]
pub type W = crate::W<Dt10Spec>;
#[doc = "Field `DT10` reader - BPR data10"]
pub type Dt10R = crate::FieldReader<u16>;
#[doc = "Field `DT10` writer - BPR data10"]
pub type Dt10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data10"]
    #[inline(always)]
    pub fn dt10(&self) -> Dt10R {
        Dt10R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data10"]
    #[inline(always)]
    pub fn dt10(&mut self) -> Dt10W<'_, Dt10Spec> {
        Dt10W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt10Spec;
impl crate::RegisterSpec for Dt10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt10::R`](R) reader structure"]
impl crate::Readable for Dt10Spec {}
#[doc = "`write(|w| ..)` method takes [`dt10::W`](W) writer structure"]
impl crate::Writable for Dt10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT10 to value 0"]
impl crate::Resettable for Dt10Spec {}
