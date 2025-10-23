#[doc = "Register `BTMG` reader"]
pub type R = crate::R<BtmgSpec>;
#[doc = "Register `BTMG` writer"]
pub type W = crate::W<BtmgSpec>;
#[doc = "Field `BRDIV` reader - Baud rate division"]
pub type BrdivR = crate::FieldReader<u16>;
#[doc = "Field `BRDIV` writer - Baud rate division"]
pub type BrdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BTS1` reader - Bit time segment 1"]
pub type Bts1R = crate::FieldReader;
#[doc = "Field `BTS1` writer - Bit time segment 1"]
pub type Bts1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BTS2` reader - Bit time segment 2"]
pub type Bts2R = crate::FieldReader;
#[doc = "Field `BTS2` writer - Bit time segment 2"]
pub type Bts2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSAW` reader - Resynchronization adjust width"]
pub type RsawR = crate::FieldReader;
#[doc = "Field `RSAW` writer - Resynchronization adjust width"]
pub type RsawW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBEN` reader - Loop back mode"]
pub type LbenR = crate::BitReader;
#[doc = "Field `LBEN` writer - Loop back mode"]
pub type LbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOEN` reader - Listen-Only mode"]
pub type LoenR = crate::BitReader;
#[doc = "Field `LOEN` writer - Listen-Only mode"]
pub type LoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Baud rate division"]
    #[inline(always)]
    pub fn brdiv(&self) -> BrdivR {
        BrdivR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Bit time segment 1"]
    #[inline(always)]
    pub fn bts1(&self) -> Bts1R {
        Bts1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Bit time segment 2"]
    #[inline(always)]
    pub fn bts2(&self) -> Bts2R {
        Bts2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Resynchronization adjust width"]
    #[inline(always)]
    pub fn rsaw(&self) -> RsawR {
        RsawR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Loop back mode"]
    #[inline(always)]
    pub fn lben(&self) -> LbenR {
        LbenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Listen-Only mode"]
    #[inline(always)]
    pub fn loen(&self) -> LoenR {
        LoenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Baud rate division"]
    #[inline(always)]
    pub fn brdiv(&mut self) -> BrdivW<'_, BtmgSpec> {
        BrdivW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Bit time segment 1"]
    #[inline(always)]
    pub fn bts1(&mut self) -> Bts1W<'_, BtmgSpec> {
        Bts1W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Bit time segment 2"]
    #[inline(always)]
    pub fn bts2(&mut self) -> Bts2W<'_, BtmgSpec> {
        Bts2W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Resynchronization adjust width"]
    #[inline(always)]
    pub fn rsaw(&mut self) -> RsawW<'_, BtmgSpec> {
        RsawW::new(self, 24)
    }
    #[doc = "Bit 30 - Loop back mode"]
    #[inline(always)]
    pub fn lben(&mut self) -> LbenW<'_, BtmgSpec> {
        LbenW::new(self, 30)
    }
    #[doc = "Bit 31 - Listen-Only mode"]
    #[inline(always)]
    pub fn loen(&mut self) -> LoenW<'_, BtmgSpec> {
        LoenW::new(self, 31)
    }
}
#[doc = "Bit timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`btmg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btmg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtmgSpec;
impl crate::RegisterSpec for BtmgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btmg::R`](R) reader structure"]
impl crate::Readable for BtmgSpec {}
#[doc = "`write(|w| ..)` method takes [`btmg::W`](W) writer structure"]
impl crate::Writable for BtmgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTMG to value 0"]
impl crate::Resettable for BtmgSpec {}
