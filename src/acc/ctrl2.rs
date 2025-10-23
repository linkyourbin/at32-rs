#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `HICKCAL` reader - Internal high-speed auto clock calibration"]
pub type HickcalR = crate::FieldReader;
#[doc = "Field `HICKTWK` reader - Internal high-speed auto clock trimming"]
pub type HicktwkR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Internal high-speed auto clock calibration"]
    #[inline(always)]
    pub fn hickcal(&self) -> HickcalR {
        HickcalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - Internal high-speed auto clock trimming"]
    #[inline(always)]
    pub fn hicktwk(&self) -> HicktwkR {
        HicktwkR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0x2080"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0x2080;
}
