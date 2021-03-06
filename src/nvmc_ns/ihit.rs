#[doc = "Reader of register IHIT"]
pub type R = crate::R<u32, super::IHIT>;
#[doc = "Writer for register IHIT"]
pub type W = crate::W<u32, super::IHIT>;
#[doc = "Register IHIT `reset()`'s with value 0"]
impl crate::ResetValue for super::IHIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HITS`"]
pub type HITS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HITS`"]
pub struct HITS_W<'a> {
    w: &'a mut W,
}
impl<'a> HITS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of cache hits Write zero to clear"]
    #[inline(always)]
    pub fn hits(&self) -> HITS_R {
        HITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of cache hits Write zero to clear"]
    #[inline(always)]
    pub fn hits(&mut self) -> HITS_W {
        HITS_W { w: self }
    }
}
