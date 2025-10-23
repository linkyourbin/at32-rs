#[doc = "Register `DT36` reader"]
pub type R = crate::R<Dt36Spec>;
#[doc = "Register `DT36` writer"]
pub type W = crate::W<Dt36Spec>;
#[doc = "Field `DT36` reader - BPR data36"]
pub type Dt36R = crate::FieldReader<u16>;
#[doc = "Field `DT36` writer - BPR data36"]
pub type Dt36W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data36"]
    #[inline(always)]
    pub fn dt36(&self) -> Dt36R {
        Dt36R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data36"]
    #[inline(always)]
    pub fn dt36(&mut self) -> Dt36W<'_, Dt36Spec> {
        Dt36W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt36Spec;
impl crate::RegisterSpec for Dt36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt36::R`](R) reader structure"]
impl crate::Readable for Dt36Spec {}
#[doc = "`write(|w| ..)` method takes [`dt36::W`](W) writer structure"]
impl crate::Writable for Dt36Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT36 to value 0"]
impl crate::Resettable for Dt36Spec {}
