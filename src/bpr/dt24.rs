#[doc = "Register `DT24` reader"]
pub type R = crate::R<Dt24Spec>;
#[doc = "Register `DT24` writer"]
pub type W = crate::W<Dt24Spec>;
#[doc = "Field `DT24` reader - BPR data24"]
pub type Dt24R = crate::FieldReader<u16>;
#[doc = "Field `DT24` writer - BPR data24"]
pub type Dt24W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data24"]
    #[inline(always)]
    pub fn dt24(&self) -> Dt24R {
        Dt24R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data24"]
    #[inline(always)]
    pub fn dt24(&mut self) -> Dt24W<'_, Dt24Spec> {
        Dt24W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt24Spec;
impl crate::RegisterSpec for Dt24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt24::R`](R) reader structure"]
impl crate::Readable for Dt24Spec {}
#[doc = "`write(|w| ..)` method takes [`dt24::W`](W) writer structure"]
impl crate::Writable for Dt24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT24 to value 0"]
impl crate::Resettable for Dt24Spec {}
