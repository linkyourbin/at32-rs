#[doc = "Register `DT20` reader"]
pub type R = crate::R<Dt20Spec>;
#[doc = "Register `DT20` writer"]
pub type W = crate::W<Dt20Spec>;
#[doc = "Field `DT20` reader - BPR data20"]
pub type Dt20R = crate::FieldReader<u16>;
#[doc = "Field `DT20` writer - BPR data20"]
pub type Dt20W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data20"]
    #[inline(always)]
    pub fn dt20(&self) -> Dt20R {
        Dt20R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data20"]
    #[inline(always)]
    pub fn dt20(&mut self) -> Dt20W<'_, Dt20Spec> {
        Dt20W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt20Spec;
impl crate::RegisterSpec for Dt20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt20::R`](R) reader structure"]
impl crate::Readable for Dt20Spec {}
#[doc = "`write(|w| ..)` method takes [`dt20::W`](W) writer structure"]
impl crate::Writable for Dt20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT20 to value 0"]
impl crate::Resettable for Dt20Spec {}
