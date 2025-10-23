#[doc = "Register `DIVCNTH` reader"]
pub type R = crate::R<DivcnthSpec>;
#[doc = "Register `DIVCNTH` writer"]
pub type W = crate::W<DivcnthSpec>;
#[doc = "Field `DIVCNT` reader - RTC divider register high"]
pub type DivcntR = crate::FieldReader;
#[doc = "Field `DIVCNT` writer - RTC divider register high"]
pub type DivcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RTC divider register high"]
    #[inline(always)]
    pub fn divcnt(&self) -> DivcntR {
        DivcntR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC divider register high"]
    #[inline(always)]
    pub fn divcnt(&mut self) -> DivcntW<'_, DivcnthSpec> {
        DivcntW::new(self, 0)
    }
}
#[doc = "RTC Divider Register High\n\nYou can [`read`](crate::Reg::read) this register and get [`divcnth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcnth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivcnthSpec;
impl crate::RegisterSpec for DivcnthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divcnth::R`](R) reader structure"]
impl crate::Readable for DivcnthSpec {}
#[doc = "`write(|w| ..)` method takes [`divcnth::W`](W) writer structure"]
impl crate::Writable for DivcnthSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVCNTH to value 0"]
impl crate::Resettable for DivcnthSpec {}
