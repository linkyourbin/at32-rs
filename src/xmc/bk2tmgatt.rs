#[doc = "Register `BK2TMGATT` reader"]
pub type R = crate::R<Bk2tmgattSpec>;
#[doc = "Register `BK2TMGATT` writer"]
pub type W = crate::W<Bk2tmgattSpec>;
#[doc = "Field `SPST` reader - special memory setup time"]
pub type SpstR = crate::FieldReader;
#[doc = "Field `SPST` writer - special memory setup time"]
pub type SpstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPWT` reader - special memory wait time"]
pub type SpwtR = crate::FieldReader;
#[doc = "Field `SPWT` writer - special memory wait time"]
pub type SpwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPHT` reader - special memory hold time"]
pub type SphtR = crate::FieldReader;
#[doc = "Field `SPHT` writer - special memory hold time"]
pub type SphtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPDHIZT` reader - special memory databus High resistance time"]
pub type SpdhiztR = crate::FieldReader;
#[doc = "Field `SPDHIZT` writer - special memory databus High resistance time"]
pub type SpdhiztW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    pub fn spst(&self) -> SpstR {
        SpstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    pub fn spwt(&self) -> SpwtR {
        SpwtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    pub fn spht(&self) -> SphtR {
        SphtR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    pub fn spdhizt(&self) -> SpdhiztR {
        SpdhiztR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - special memory setup time"]
    #[inline(always)]
    pub fn spst(&mut self) -> SpstW<'_, Bk2tmgattSpec> {
        SpstW::new(self, 0)
    }
    #[doc = "Bits 8:15 - special memory wait time"]
    #[inline(always)]
    pub fn spwt(&mut self) -> SpwtW<'_, Bk2tmgattSpec> {
        SpwtW::new(self, 8)
    }
    #[doc = "Bits 16:23 - special memory hold time"]
    #[inline(always)]
    pub fn spht(&mut self) -> SphtW<'_, Bk2tmgattSpec> {
        SphtW::new(self, 16)
    }
    #[doc = "Bits 24:31 - special memory databus High resistance time"]
    #[inline(always)]
    pub fn spdhizt(&mut self) -> SpdhiztW<'_, Bk2tmgattSpec> {
        SpdhiztW::new(self, 24)
    }
}
#[doc = "special memory space timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2tmgatt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2tmgatt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk2tmgattSpec;
impl crate::RegisterSpec for Bk2tmgattSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk2tmgatt::R`](R) reader structure"]
impl crate::Readable for Bk2tmgattSpec {}
#[doc = "`write(|w| ..)` method takes [`bk2tmgatt::W`](W) writer structure"]
impl crate::Writable for Bk2tmgattSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BK2TMGATT to value 0xfcfc_fcfc"]
impl crate::Resettable for Bk2tmgattSpec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
