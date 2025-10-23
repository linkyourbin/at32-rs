#[doc = "Register `MISC2` reader"]
pub type R = crate::R<Misc2Spec>;
#[doc = "Register `MISC2` writer"]
pub type W = crate::W<Misc2Spec>;
#[doc = "Field `CLK_TO_TMR` reader - Clock output internal connect to timer10"]
pub type ClkToTmrR = crate::BitReader;
#[doc = "Field `CLK_TO_TMR` writer - Clock output internal connect to timer10"]
pub type ClkToTmrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Clock output internal connect to timer10"]
    #[inline(always)]
    pub fn clk_to_tmr(&self) -> ClkToTmrR {
        ClkToTmrR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Clock output internal connect to timer10"]
    #[inline(always)]
    pub fn clk_to_tmr(&mut self) -> ClkToTmrW<'_, Misc2Spec> {
        ClkToTmrW::new(self, 16)
    }
}
#[doc = "Miscellaneous register2\n\nYou can [`read`](crate::Reg::read) this register and get [`misc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Misc2Spec;
impl crate::RegisterSpec for Misc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc2::R`](R) reader structure"]
impl crate::Readable for Misc2Spec {}
#[doc = "`write(|w| ..)` method takes [`misc2::W`](W) writer structure"]
impl crate::Writable for Misc2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC2 to value 0"]
impl crate::Resettable for Misc2Spec {}
