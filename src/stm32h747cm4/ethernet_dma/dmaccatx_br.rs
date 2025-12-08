#[doc = "Register `DMACCATxBR` reader"]
pub struct R(crate::R<DMACCATXBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACCATXBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACCATXBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACCATXBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACCATxBR` writer"]
pub struct W(crate::W<DMACCATXBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACCATXBR_SPEC>;
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
impl From<crate::W<DMACCATXBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACCATXBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURTBUFAPTR` reader - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CURTBUFAPTR` writer - Application Transmit Buffer Address Pointer"]
pub type CURTBUFAPTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACCATXBR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&self) -> CURTBUFAPTR_R {
        CURTBUFAPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Application Transmit Buffer Address Pointer"]
    #[inline(always)]
    pub fn curtbufaptr(&mut self) -> CURTBUFAPTR_W<0> {
        CURTBUFAPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel current application transmit buffer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaccatx_br](index.html) module"]
pub struct DMACCATXBR_SPEC;
impl crate::RegisterSpec for DMACCATXBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmaccatx_br::R](R) reader structure"]
impl crate::Readable for DMACCATXBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmaccatx_br::W](W) writer structure"]
impl crate::Writable for DMACCATXBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMACCATxBR to value 0"]
impl crate::Resettable for DMACCATXBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
