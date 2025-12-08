#[doc = "Register `BDTxUPR` reader"]
pub struct R(crate::R<BDTXUPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTXUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTXUPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTXUPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDTxUPR` writer"]
pub struct W(crate::W<BDTXUPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTXUPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BDTXUPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTXUPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMxFLTR` reader - HRTIM_FLTxR register update enable"]
pub type TIMXFLTR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxFLTR` writer - HRTIM_FLTxR register update enable"]
pub type TIMXFLTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxOUTR` reader - HRTIM_OUTxR register update enable"]
pub type TIMXOUTR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxOUTR` writer - HRTIM_OUTxR register update enable"]
pub type TIMXOUTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCHPR` reader - HRTIM_CHPxR register update enable"]
pub type TIMXCHPR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCHPR` writer - HRTIM_CHPxR register update enable"]
pub type TIMXCHPR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxRSTR` reader - HRTIM_RSTxR register update enable"]
pub type TIMXRSTR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxRSTR` writer - HRTIM_RSTxR register update enable"]
pub type TIMXRSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxEEFR2` reader - HRTIM_EEFxR2 register update enable"]
pub type TIMXEEFR2_R = crate::BitReader<bool>;
#[doc = "Field `TIMxEEFR2` writer - HRTIM_EEFxR2 register update enable"]
pub type TIMXEEFR2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxEEFR1` reader - HRTIM_EEFxR1 register update enable"]
pub type TIMXEEFR1_R = crate::BitReader<bool>;
#[doc = "Field `TIMxEEFR1` writer - HRTIM_EEFxR1 register update enable"]
pub type TIMXEEFR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxRST2R` reader - HRTIM_RST2xR register update enable"]
pub type TIMXRST2R_R = crate::BitReader<bool>;
#[doc = "Field `TIMxRST2R` writer - HRTIM_RST2xR register update enable"]
pub type TIMXRST2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxSET2R` reader - HRTIM_SET2xR register update enable"]
pub type TIMXSET2R_R = crate::BitReader<bool>;
#[doc = "Field `TIMxSET2R` writer - HRTIM_SET2xR register update enable"]
pub type TIMXSET2R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxRST1R` reader - HRTIM_RST1xR register update enable"]
pub type TIMXRST1R_R = crate::BitReader<bool>;
#[doc = "Field `TIMxRST1R` writer - HRTIM_RST1xR register update enable"]
pub type TIMXRST1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxSET1R` reader - HRTIM_SET1xR register update enable"]
pub type TIMXSET1R_R = crate::BitReader<bool>;
#[doc = "Field `TIMxSET1R` writer - HRTIM_SET1xR register update enable"]
pub type TIMXSET1R_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMx_DTxR` reader - HRTIM_DTxR register update enable"]
pub type TIMX_DTXR_R = crate::BitReader<bool>;
#[doc = "Field `TIMx_DTxR` writer - HRTIM_DTxR register update enable"]
pub type TIMX_DTXR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCMP4` reader - HRTIM_CMP4xR register update enable"]
pub type TIMXCMP4_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCMP4` writer - HRTIM_CMP4xR register update enable"]
pub type TIMXCMP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCMP3` reader - HRTIM_CMP3xR register update enable"]
pub type TIMXCMP3_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCMP3` writer - HRTIM_CMP3xR register update enable"]
pub type TIMXCMP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCMP2` reader - HRTIM_CMP2xR register update enable"]
pub type TIMXCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCMP2` writer - HRTIM_CMP2xR register update enable"]
pub type TIMXCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCMP1` reader - HRTIM_CMP1xR register update enable"]
pub type TIMXCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCMP1` writer - HRTIM_CMP1xR register update enable"]
pub type TIMXCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxREP` reader - HRTIM_REPxR register update enable"]
pub type TIMXREP_R = crate::BitReader<bool>;
#[doc = "Field `TIMxREP` writer - HRTIM_REPxR register update enable"]
pub type TIMXREP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxPER` reader - HRTIM_PERxR register update enable"]
pub type TIMXPER_R = crate::BitReader<bool>;
#[doc = "Field `TIMxPER` writer - HRTIM_PERxR register update enable"]
pub type TIMXPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCNT` reader - HRTIM_CNTxR register update enable"]
pub type TIMXCNT_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCNT` writer - HRTIM_CNTxR register update enable"]
pub type TIMXCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxDIER` reader - HRTIM_TIMxDIER register update enable"]
pub type TIMXDIER_R = crate::BitReader<bool>;
#[doc = "Field `TIMxDIER` writer - HRTIM_TIMxDIER register update enable"]
pub type TIMXDIER_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxICR` reader - HRTIM_TIMxICR register update enable"]
pub type TIMXICR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxICR` writer - HRTIM_TIMxICR register update enable"]
pub type TIMXICR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
#[doc = "Field `TIMxCR` reader - HRTIM_TIMxCR register update enable"]
pub type TIMXCR_R = crate::BitReader<bool>;
#[doc = "Field `TIMxCR` writer - HRTIM_TIMxCR register update enable"]
pub type TIMXCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTXUPR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 20 - HRTIM_FLTxR register update enable"]
    #[inline(always)]
    pub fn timx_fltr(&self) -> TIMXFLTR_R {
        TIMXFLTR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - HRTIM_OUTxR register update enable"]
    #[inline(always)]
    pub fn timx_outr(&self) -> TIMXOUTR_R {
        TIMXOUTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - HRTIM_CHPxR register update enable"]
    #[inline(always)]
    pub fn timx_chpr(&self) -> TIMXCHPR_R {
        TIMXCHPR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - HRTIM_RSTxR register update enable"]
    #[inline(always)]
    pub fn timx_rstr(&self) -> TIMXRSTR_R {
        TIMXRSTR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - HRTIM_EEFxR2 register update enable"]
    #[inline(always)]
    pub fn timx_eefr2(&self) -> TIMXEEFR2_R {
        TIMXEEFR2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - HRTIM_EEFxR1 register update enable"]
    #[inline(always)]
    pub fn timx_eefr1(&self) -> TIMXEEFR1_R {
        TIMXEEFR1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - HRTIM_RST2xR register update enable"]
    #[inline(always)]
    pub fn timx_rst2r(&self) -> TIMXRST2R_R {
        TIMXRST2R_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - HRTIM_SET2xR register update enable"]
    #[inline(always)]
    pub fn timx_set2r(&self) -> TIMXSET2R_R {
        TIMXSET2R_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - HRTIM_RST1xR register update enable"]
    #[inline(always)]
    pub fn timx_rst1r(&self) -> TIMXRST1R_R {
        TIMXRST1R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - HRTIM_SET1xR register update enable"]
    #[inline(always)]
    pub fn timx_set1r(&self) -> TIMXSET1R_R {
        TIMXSET1R_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - HRTIM_DTxR register update enable"]
    #[inline(always)]
    pub fn timx_dtx_r(&self) -> TIMX_DTXR_R {
        TIMX_DTXR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - HRTIM_CMP4xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp4(&self) -> TIMXCMP4_R {
        TIMXCMP4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - HRTIM_CMP3xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp3(&self) -> TIMXCMP3_R {
        TIMXCMP3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - HRTIM_CMP2xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp2(&self) -> TIMXCMP2_R {
        TIMXCMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - HRTIM_CMP1xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp1(&self) -> TIMXCMP1_R {
        TIMXCMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - HRTIM_REPxR register update enable"]
    #[inline(always)]
    pub fn timx_rep(&self) -> TIMXREP_R {
        TIMXREP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - HRTIM_PERxR register update enable"]
    #[inline(always)]
    pub fn timx_per(&self) -> TIMXPER_R {
        TIMXPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - HRTIM_CNTxR register update enable"]
    #[inline(always)]
    pub fn timx_cnt(&self) -> TIMXCNT_R {
        TIMXCNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - HRTIM_TIMxDIER register update enable"]
    #[inline(always)]
    pub fn timx_dier(&self) -> TIMXDIER_R {
        TIMXDIER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - HRTIM_TIMxICR register update enable"]
    #[inline(always)]
    pub fn timx_icr(&self) -> TIMXICR_R {
        TIMXICR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - HRTIM_TIMxCR register update enable"]
    #[inline(always)]
    pub fn timx_cr(&self) -> TIMXCR_R {
        TIMXCR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - HRTIM_FLTxR register update enable"]
    #[inline(always)]
    pub fn timx_fltr(&mut self) -> TIMXFLTR_W<20> {
        TIMXFLTR_W::new(self)
    }
    #[doc = "Bit 19 - HRTIM_OUTxR register update enable"]
    #[inline(always)]
    pub fn timx_outr(&mut self) -> TIMXOUTR_W<19> {
        TIMXOUTR_W::new(self)
    }
    #[doc = "Bit 18 - HRTIM_CHPxR register update enable"]
    #[inline(always)]
    pub fn timx_chpr(&mut self) -> TIMXCHPR_W<18> {
        TIMXCHPR_W::new(self)
    }
    #[doc = "Bit 17 - HRTIM_RSTxR register update enable"]
    #[inline(always)]
    pub fn timx_rstr(&mut self) -> TIMXRSTR_W<17> {
        TIMXRSTR_W::new(self)
    }
    #[doc = "Bit 16 - HRTIM_EEFxR2 register update enable"]
    #[inline(always)]
    pub fn timx_eefr2(&mut self) -> TIMXEEFR2_W<16> {
        TIMXEEFR2_W::new(self)
    }
    #[doc = "Bit 15 - HRTIM_EEFxR1 register update enable"]
    #[inline(always)]
    pub fn timx_eefr1(&mut self) -> TIMXEEFR1_W<15> {
        TIMXEEFR1_W::new(self)
    }
    #[doc = "Bit 14 - HRTIM_RST2xR register update enable"]
    #[inline(always)]
    pub fn timx_rst2r(&mut self) -> TIMXRST2R_W<14> {
        TIMXRST2R_W::new(self)
    }
    #[doc = "Bit 13 - HRTIM_SET2xR register update enable"]
    #[inline(always)]
    pub fn timx_set2r(&mut self) -> TIMXSET2R_W<13> {
        TIMXSET2R_W::new(self)
    }
    #[doc = "Bit 12 - HRTIM_RST1xR register update enable"]
    #[inline(always)]
    pub fn timx_rst1r(&mut self) -> TIMXRST1R_W<12> {
        TIMXRST1R_W::new(self)
    }
    #[doc = "Bit 11 - HRTIM_SET1xR register update enable"]
    #[inline(always)]
    pub fn timx_set1r(&mut self) -> TIMXSET1R_W<11> {
        TIMXSET1R_W::new(self)
    }
    #[doc = "Bit 10 - HRTIM_DTxR register update enable"]
    #[inline(always)]
    pub fn timx_dtx_r(&mut self) -> TIMX_DTXR_W<10> {
        TIMX_DTXR_W::new(self)
    }
    #[doc = "Bit 9 - HRTIM_CMP4xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp4(&mut self) -> TIMXCMP4_W<9> {
        TIMXCMP4_W::new(self)
    }
    #[doc = "Bit 8 - HRTIM_CMP3xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp3(&mut self) -> TIMXCMP3_W<8> {
        TIMXCMP3_W::new(self)
    }
    #[doc = "Bit 7 - HRTIM_CMP2xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp2(&mut self) -> TIMXCMP2_W<7> {
        TIMXCMP2_W::new(self)
    }
    #[doc = "Bit 6 - HRTIM_CMP1xR register update enable"]
    #[inline(always)]
    pub fn timx_cmp1(&mut self) -> TIMXCMP1_W<6> {
        TIMXCMP1_W::new(self)
    }
    #[doc = "Bit 5 - HRTIM_REPxR register update enable"]
    #[inline(always)]
    pub fn timx_rep(&mut self) -> TIMXREP_W<5> {
        TIMXREP_W::new(self)
    }
    #[doc = "Bit 4 - HRTIM_PERxR register update enable"]
    #[inline(always)]
    pub fn timx_per(&mut self) -> TIMXPER_W<4> {
        TIMXPER_W::new(self)
    }
    #[doc = "Bit 3 - HRTIM_CNTxR register update enable"]
    #[inline(always)]
    pub fn timx_cnt(&mut self) -> TIMXCNT_W<3> {
        TIMXCNT_W::new(self)
    }
    #[doc = "Bit 2 - HRTIM_TIMxDIER register update enable"]
    #[inline(always)]
    pub fn timx_dier(&mut self) -> TIMXDIER_W<2> {
        TIMXDIER_W::new(self)
    }
    #[doc = "Bit 1 - HRTIM_TIMxICR register update enable"]
    #[inline(always)]
    pub fn timx_icr(&mut self) -> TIMXICR_W<1> {
        TIMXICR_W::new(self)
    }
    #[doc = "Bit 0 - HRTIM_TIMxCR register update enable"]
    #[inline(always)]
    pub fn timx_cr(&mut self) -> TIMXCR_W<0> {
        TIMXCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtx_upr](index.html) module"]
pub struct BDTXUPR_SPEC;
impl crate::RegisterSpec for BDTXUPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdtx_upr::R](R) reader structure"]
impl crate::Readable for BDTXUPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdtx_upr::W](W) writer structure"]
impl crate::Writable for BDTXUPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDTxUPR to value 0"]
impl crate::Resettable for BDTXUPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
