#[doc = "Register `BK2TMGMEM` reader"]
pub type R = crate::R<Bk2tmgmemSpec>;
#[doc = "Register `BK2TMGMEM` writer"]
pub type W = crate::W<Bk2tmgmemSpec>;
#[doc = "Field `RGST` reader - Regular memory setup time"]
pub type RgstR = crate::FieldReader;
#[doc = "Field `RGST` writer - Regular memory setup time"]
pub type RgstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGWT` reader - Regular memory wait time"]
pub type RgwtR = crate::FieldReader;
#[doc = "Field `RGWT` writer - Regular memory wait time"]
pub type RgwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGHT` reader - Regular memory hold time"]
pub type RghtR = crate::FieldReader;
#[doc = "Field `RGHT` writer - Regular memory hold time"]
pub type RghtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGDHIZT` reader - Regular memory databus High resistance time"]
pub type RgdhiztR = crate::FieldReader;
#[doc = "Field `RGDHIZT` writer - Regular memory databus High resistance time"]
pub type RgdhiztW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    pub fn rgst(&self) -> RgstR {
        RgstR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    pub fn rgwt(&self) -> RgwtR {
        RgwtR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    pub fn rght(&self) -> RghtR {
        RghtR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    pub fn rgdhizt(&self) -> RgdhiztR {
        RgdhiztR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Regular memory setup time"]
    #[inline(always)]
    pub fn rgst(&mut self) -> RgstW<'_, Bk2tmgmemSpec> {
        RgstW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Regular memory wait time"]
    #[inline(always)]
    pub fn rgwt(&mut self) -> RgwtW<'_, Bk2tmgmemSpec> {
        RgwtW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Regular memory hold time"]
    #[inline(always)]
    pub fn rght(&mut self) -> RghtW<'_, Bk2tmgmemSpec> {
        RghtW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Regular memory databus High resistance time"]
    #[inline(always)]
    pub fn rgdhizt(&mut self) -> RgdhiztW<'_, Bk2tmgmemSpec> {
        RgdhiztW::new(self, 24)
    }
}
#[doc = "Regular memory space timing register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2tmgmem::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2tmgmem::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk2tmgmemSpec;
impl crate::RegisterSpec for Bk2tmgmemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk2tmgmem::R`](R) reader structure"]
impl crate::Readable for Bk2tmgmemSpec {}
#[doc = "`write(|w| ..)` method takes [`bk2tmgmem::W`](W) writer structure"]
impl crate::Writable for Bk2tmgmemSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BK2TMGMEM to value 0xfcfc_fcfc"]
impl crate::Resettable for Bk2tmgmemSpec {
    const RESET_VALUE: u32 = 0xfcfc_fcfc;
}
