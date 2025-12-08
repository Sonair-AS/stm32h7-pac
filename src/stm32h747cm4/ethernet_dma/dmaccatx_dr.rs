#[doc = "Register `DMACCATxDR` reader"]
pub struct R(crate::R<DMACCATXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCATXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCATXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCATXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACCATxDR` writer"]
pub struct W(crate::W<DMACCATXDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCATXDR_SPEC>;
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
impl From<crate::W<DMACCATXDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCATXDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURTDESAPTR` reader - Application Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURTDESAPTR` writer - Application Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACCATXDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&mut self) -> CURTDESAPTR_W<0> {
        CURTDESAPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current application transmit descriptor register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccatx_dr](index.html) module"]
pub struct DMACCATXDR_SPEC;
impl crate::RegisterSpec for DMACCATXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccatx_dr::R](R) reader structure"]
impl crate::Readable for DMACCATXDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaccatx_dr::W](W) writer structure"]
impl crate::Writable for DMACCATXDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACCATxDR to value 0"]
impl crate::Resettable for DMACCATXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
