# [doc = "Register `SPIPF6E0` reader"] pub type R = crate :: R < Spipf6e0Spec > ; # [doc = "Register `SPIPF6E0` writer"] pub type W = crate :: W < Spipf6e0Spec > ; impl W { } # [doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6e0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6e0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Spipf6e0Spec ; impl crate :: RegisterSpec for Spipf6e0Spec { type Ux = u32 ; } # [doc = "`read()` method returns [`spipf6e0::R`](R) reader structure"] impl crate :: Readable for Spipf6e0Spec { } # [doc = "`write(|w| ..)` method takes [`spipf6e0::W`](W) writer structure"] impl crate :: Writable for Spipf6e0Spec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets SPIPF6E0 to value 0"] impl crate :: Resettable for Spipf6e0Spec { }