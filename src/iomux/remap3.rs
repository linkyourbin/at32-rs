#[doc = "Register `REMAP3` reader"]
pub type R = crate::R<Remap3Spec>;
#[doc = "Register `REMAP3` writer"]
pub type W = crate::W<Remap3Spec>;
#[doc = "Field `TMR9_GMUX` reader - TMR9 muxing"]
pub type Tmr9GmuxR = crate::FieldReader;
#[doc = "Field `TMR9_GMUX` writer - TMR9 muxing"]
pub type Tmr9GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_gmux(&self) -> Tmr9GmuxR {
        Tmr9GmuxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TMR9 muxing"]
    #[inline(always)]
    pub fn tmr9_gmux(&mut self) -> Tmr9GmuxW<'_, Remap3Spec> {
        Tmr9GmuxW::new(self, 0)
    }
}
#[doc = "IO MUX remap register 3 (IOMUX_REMAP3)\n\nYou can [`read`](crate::Reg::read) this register and get [`remap3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`remap3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap3Spec;
impl crate::RegisterSpec for Remap3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap3::R`](R) reader structure"]
impl crate::Readable for Remap3Spec {}
#[doc = "`write(|w| ..)` method takes [`remap3::W`](W) writer structure"]
impl crate::Writable for Remap3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REMAP3 to value 0"]
impl crate::Resettable for Remap3Spec {}
