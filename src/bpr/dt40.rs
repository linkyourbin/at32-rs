#[doc = "Register `DT40` reader"]
pub type R = crate::R<Dt40Spec>;
#[doc = "Register `DT40` writer"]
pub type W = crate::W<Dt40Spec>;
#[doc = "Field `DT40` reader - BPR data40"]
pub type Dt40R = crate::FieldReader<u16>;
#[doc = "Field `DT40` writer - BPR data40"]
pub type Dt40W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BPR data40"]
    #[inline(always)]
    pub fn dt40(&self) -> Dt40R {
        Dt40R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BPR data40"]
    #[inline(always)]
    pub fn dt40(&mut self) -> Dt40W<'_, Dt40Spec> {
        Dt40W::new(self, 0)
    }
}
#[doc = "Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::Reg::read) this register and get [`dt40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dt40Spec;
impl crate::RegisterSpec for Dt40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt40::R`](R) reader structure"]
impl crate::Readable for Dt40Spec {}
#[doc = "`write(|w| ..)` method takes [`dt40::W`](W) writer structure"]
impl crate::Writable for Dt40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT40 to value 0"]
impl crate::Resettable for Dt40Spec {}
