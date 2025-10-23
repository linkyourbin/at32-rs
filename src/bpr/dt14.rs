#[doc = "Register `DT14` reader"]
pub type R = crate::R<Dt14Spec>;
#[doc = "Register `DT14` writer"]
pub type W = crate::W<Dt14Spec>;
#[doc = "Field `DT14` reader - BPR data14"]
pub type Dt14R = crate::FieldReader<u16>;
#[doc = "Field `DT14` writer - BPR data14"]
pub type Dt14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data14"]
    #[inline(always)]
    pub fn dt14(&self) -> Dt14R {
        Dt14R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data14"]
    #[inline(always)]
    pub fn dt14(&mut self) -> Dt14W<'_, Dt14Spec> {
        Dt14W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt14Spec;
impl crate::RegisterSpec for Dt14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt14::R`](R) reader structure"]
impl crate::Readable for Dt14Spec {}
#[doc = "`write(|w| ..)` method takes [`dt14::W`](W) writer structure"]
impl crate::Writable for Dt14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT14 to value 0"]
impl crate::Resettable for Dt14Spec {}
