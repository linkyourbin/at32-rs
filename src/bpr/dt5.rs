#[doc = "Register `DT5` reader"]
pub type R = crate::R<Dt5Spec>;
#[doc = "Register `DT5` writer"]
pub type W = crate::W<Dt5Spec>;
#[doc = "Field `DT5` reader - BPR data5"]
pub type Dt5R = crate::FieldReader<u16>;
#[doc = "Field `DT5` writer - BPR data5"]
pub type Dt5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data5"]
    #[inline(always)]
    pub fn dt5(&self) -> Dt5R {
        Dt5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data5"]
    #[inline(always)]
    pub fn dt5(&mut self) -> Dt5W<'_, Dt5Spec> {
        Dt5W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt5Spec;
impl crate::RegisterSpec for Dt5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt5::R`](R) reader structure"]
impl crate::Readable for Dt5Spec {}
#[doc = "`write(|w| ..)` method takes [`dt5::W`](W) writer structure"]
impl crate::Writable for Dt5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT5 to value 0"]
impl crate::Resettable for Dt5Spec {}
