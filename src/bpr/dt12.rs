#[doc = "Register `DT12` reader"]
pub type R = crate::R<Dt12Spec>;
#[doc = "Register `DT12` writer"]
pub type W = crate::W<Dt12Spec>;
#[doc = "Field `DT12` reader - BPR data12"]
pub type Dt12R = crate::FieldReader<u16>;
#[doc = "Field `DT12` writer - BPR data12"]
pub type Dt12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data12"]
    #[inline(always)]
    pub fn dt12(&self) -> Dt12R {
        Dt12R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data12"]
    #[inline(always)]
    pub fn dt12(&mut self) -> Dt12W<'_, Dt12Spec> {
        Dt12W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt12Spec;
impl crate::RegisterSpec for Dt12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt12::R`](R) reader structure"]
impl crate::Readable for Dt12Spec {}
#[doc = "`write(|w| ..)` method takes [`dt12::W`](W) writer structure"]
impl crate::Writable for Dt12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT12 to value 0"]
impl crate::Resettable for Dt12Spec {}
