#[doc = "Register `FBWCFG` reader"]
pub type R = crate::R<FbwcfgSpec>;
#[doc = "Register `FBWCFG` writer"]
pub type W = crate::W<FbwcfgSpec>;
#[doc = "Field `FBWSEL0` reader - Filter bit width select"]
pub type Fbwsel0R = crate::BitReader;
#[doc = "Field `FBWSEL0` writer - Filter bit width select"]
pub type Fbwsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL1` reader - Filter bit width select"]
pub type Fbwsel1R = crate::BitReader;
#[doc = "Field `FBWSEL1` writer - Filter bit width select"]
pub type Fbwsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL2` reader - Filter bit width select"]
pub type Fbwsel2R = crate::BitReader;
#[doc = "Field `FBWSEL2` writer - Filter bit width select"]
pub type Fbwsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL3` reader - Filter bit width select"]
pub type Fbwsel3R = crate::BitReader;
#[doc = "Field `FBWSEL3` writer - Filter bit width select"]
pub type Fbwsel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL4` reader - Filter bit width select"]
pub type Fbwsel4R = crate::BitReader;
#[doc = "Field `FBWSEL4` writer - Filter bit width select"]
pub type Fbwsel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL5` reader - Filter bit width select"]
pub type Fbwsel5R = crate::BitReader;
#[doc = "Field `FBWSEL5` writer - Filter bit width select"]
pub type Fbwsel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL6` reader - Filter bit width select"]
pub type Fbwsel6R = crate::BitReader;
#[doc = "Field `FBWSEL6` writer - Filter bit width select"]
pub type Fbwsel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL7` reader - Filter bit width select"]
pub type Fbwsel7R = crate::BitReader;
#[doc = "Field `FBWSEL7` writer - Filter bit width select"]
pub type Fbwsel7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL8` reader - Filter bit width select"]
pub type Fbwsel8R = crate::BitReader;
#[doc = "Field `FBWSEL8` writer - Filter bit width select"]
pub type Fbwsel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL9` reader - Filter bit width select"]
pub type Fbwsel9R = crate::BitReader;
#[doc = "Field `FBWSEL9` writer - Filter bit width select"]
pub type Fbwsel9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL10` reader - Filter bit width select"]
pub type Fbwsel10R = crate::BitReader;
#[doc = "Field `FBWSEL10` writer - Filter bit width select"]
pub type Fbwsel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL11` reader - Filter bit width select"]
pub type Fbwsel11R = crate::BitReader;
#[doc = "Field `FBWSEL11` writer - Filter bit width select"]
pub type Fbwsel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL12` reader - Filter bit width select"]
pub type Fbwsel12R = crate::BitReader;
#[doc = "Field `FBWSEL12` writer - Filter bit width select"]
pub type Fbwsel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBWSEL13` reader - Filter bit width select"]
pub type Fbwsel13R = crate::BitReader;
#[doc = "Field `FBWSEL13` writer - Filter bit width select"]
pub type Fbwsel13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel0(&self) -> Fbwsel0R {
        Fbwsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel1(&self) -> Fbwsel1R {
        Fbwsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel2(&self) -> Fbwsel2R {
        Fbwsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel3(&self) -> Fbwsel3R {
        Fbwsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel4(&self) -> Fbwsel4R {
        Fbwsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel5(&self) -> Fbwsel5R {
        Fbwsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel6(&self) -> Fbwsel6R {
        Fbwsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel7(&self) -> Fbwsel7R {
        Fbwsel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel8(&self) -> Fbwsel8R {
        Fbwsel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel9(&self) -> Fbwsel9R {
        Fbwsel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel10(&self) -> Fbwsel10R {
        Fbwsel10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel11(&self) -> Fbwsel11R {
        Fbwsel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel12(&self) -> Fbwsel12R {
        Fbwsel12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel13(&self) -> Fbwsel13R {
        Fbwsel13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel0(&mut self) -> Fbwsel0W<'_, FbwcfgSpec> {
        Fbwsel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel1(&mut self) -> Fbwsel1W<'_, FbwcfgSpec> {
        Fbwsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel2(&mut self) -> Fbwsel2W<'_, FbwcfgSpec> {
        Fbwsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel3(&mut self) -> Fbwsel3W<'_, FbwcfgSpec> {
        Fbwsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel4(&mut self) -> Fbwsel4W<'_, FbwcfgSpec> {
        Fbwsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel5(&mut self) -> Fbwsel5W<'_, FbwcfgSpec> {
        Fbwsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel6(&mut self) -> Fbwsel6W<'_, FbwcfgSpec> {
        Fbwsel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel7(&mut self) -> Fbwsel7W<'_, FbwcfgSpec> {
        Fbwsel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel8(&mut self) -> Fbwsel8W<'_, FbwcfgSpec> {
        Fbwsel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel9(&mut self) -> Fbwsel9W<'_, FbwcfgSpec> {
        Fbwsel9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel10(&mut self) -> Fbwsel10W<'_, FbwcfgSpec> {
        Fbwsel10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel11(&mut self) -> Fbwsel11W<'_, FbwcfgSpec> {
        Fbwsel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel12(&mut self) -> Fbwsel12W<'_, FbwcfgSpec> {
        Fbwsel12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter bit width select"]
    #[inline(always)]
    pub fn fbwsel13(&mut self) -> Fbwsel13W<'_, FbwcfgSpec> {
        Fbwsel13W::new(self, 13)
    }
}
#[doc = "Filter bit width config register\n\nYou can [`read`](crate::Reg::read) this register and get [`fbwcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbwcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbwcfgSpec;
impl crate::RegisterSpec for FbwcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbwcfg::R`](R) reader structure"]
impl crate::Readable for FbwcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fbwcfg::W`](W) writer structure"]
impl crate::Writable for FbwcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FBWCFG to value 0"]
impl crate::Resettable for FbwcfgSpec {}
