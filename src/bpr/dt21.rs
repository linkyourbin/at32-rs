#[doc = "Register `DT21` reader"]
pub type R = crate::R<Dt21Spec>;
#[doc = "Register `DT21` writer"]
pub type W = crate::W<Dt21Spec>;
#[doc = "Field `DT21` reader - BPR data21"]
pub type Dt21R = crate::FieldReader<u16>;
#[doc = "Field `DT21` writer - BPR data21"]
pub type Dt21W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data21"]
    #[inline(always)]
    pub fn dt21(&self) -> Dt21R {
        Dt21R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data21"]
    #[inline(always)]
    pub fn dt21(&mut self) -> Dt21W<'_, Dt21Spec> {
        Dt21W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt21Spec;
impl crate::RegisterSpec for Dt21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt21::R`](R) reader structure"]
impl crate::Readable for Dt21Spec {}
#[doc = "`write(|w| ..)` method takes [`dt21::W`](W) writer structure"]
impl crate::Writable for Dt21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT21 to value 0"]
impl crate::Resettable for Dt21Spec {}
