#[doc = "Register `BK2CTRL` reader"]
pub type R = crate::R<Bk2ctrlSpec>;
#[doc = "Register `BK2CTRL` writer"]
pub type W = crate::W<Bk2ctrlSpec>;
#[doc = "Field `NWEN` reader - Wait feature enable"]
pub type NwenR = crate::BitReader;
#[doc = "Field `NWEN` writer - Wait feature enable"]
pub type NwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - Memory bank enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Memory bank enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV` reader - Memory device type"]
pub type DevR = crate::BitReader;
#[doc = "Field `DEV` writer - Memory device type"]
pub type DevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTMDBW` reader - External memory data bus width"]
pub type ExtmdbwR = crate::FieldReader;
#[doc = "Field `EXTMDBW` writer - External memory data bus width"]
pub type ExtmdbwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCEN` reader - ECC enable"]
pub type EccenR = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECC enable"]
pub type EccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCR` reader - CLE to RE delay"]
pub type TcrR = crate::FieldReader;
#[doc = "Field `TCR` writer - CLE to RE delay"]
pub type TcrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TAR` reader - ALE to RE delay"]
pub type TarR = crate::FieldReader;
#[doc = "Field `TAR` writer - ALE to RE delay"]
pub type TarW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECCPGS` reader - ECC page size"]
pub type EccpgsR = crate::FieldReader;
#[doc = "Field `ECCPGS` writer - ECC page size"]
pub type EccpgsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn nwen(&self) -> NwenR {
        NwenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Memory device type"]
    #[inline(always)]
    pub fn dev(&self) -> DevR {
        DevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    pub fn extmdbw(&self) -> ExtmdbwR {
        ExtmdbwR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&self) -> EccenR {
        EccenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn tar(&self) -> TarR {
        TarR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECC page size"]
    #[inline(always)]
    pub fn eccpgs(&self) -> EccpgsR {
        EccpgsR::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Wait feature enable"]
    #[inline(always)]
    pub fn nwen(&mut self) -> NwenW<'_, Bk2ctrlSpec> {
        NwenW::new(self, 1)
    }
    #[doc = "Bit 2 - Memory bank enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, Bk2ctrlSpec> {
        EnW::new(self, 2)
    }
    #[doc = "Bit 3 - Memory device type"]
    #[inline(always)]
    pub fn dev(&mut self) -> DevW<'_, Bk2ctrlSpec> {
        DevW::new(self, 3)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    pub fn extmdbw(&mut self) -> ExtmdbwW<'_, Bk2ctrlSpec> {
        ExtmdbwW::new(self, 4)
    }
    #[doc = "Bit 6 - ECC enable"]
    #[inline(always)]
    pub fn eccen(&mut self) -> EccenW<'_, Bk2ctrlSpec> {
        EccenW::new(self, 6)
    }
    #[doc = "Bits 9:12 - CLE to RE delay"]
    #[inline(always)]
    pub fn tcr(&mut self) -> TcrW<'_, Bk2ctrlSpec> {
        TcrW::new(self, 9)
    }
    #[doc = "Bits 13:16 - ALE to RE delay"]
    #[inline(always)]
    pub fn tar(&mut self) -> TarW<'_, Bk2ctrlSpec> {
        TarW::new(self, 13)
    }
    #[doc = "Bits 17:19 - ECC page size"]
    #[inline(always)]
    pub fn eccpgs(&mut self) -> EccpgsW<'_, Bk2ctrlSpec> {
        EccpgsW::new(self, 17)
    }
}
#[doc = "PC Card/NAND Flash control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`bk2ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bk2ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk2ctrlSpec;
impl crate::RegisterSpec for Bk2ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk2ctrl::R`](R) reader structure"]
impl crate::Readable for Bk2ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bk2ctrl::W`](W) writer structure"]
impl crate::Writable for Bk2ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BK2CTRL to value 0x18"]
impl crate::Resettable for Bk2ctrlSpec {
    const RESET_VALUE: u32 = 0x18;
}
