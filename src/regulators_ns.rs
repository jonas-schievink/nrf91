#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1280usize],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved1: [u8; 12usize],
    #[doc = "0x510 - Power-fail comparator configuration"]
    pub pofcon: POFCON,
    _reserved2: [u8; 100usize],
    #[doc = "0x578 - Enable DC/DC mode of the main voltage regulator"]
    pub dcdcen: DCDCEN,
}
#[doc = "System OFF register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systemoff](systemoff) module"]
pub type SYSTEMOFF = crate::Reg<u32, _SYSTEMOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEMOFF;
#[doc = "`write(|w| ..)` method takes [systemoff::W](systemoff::W) writer structure"]
impl crate::Writable for SYSTEMOFF {}
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "Power-fail comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pofcon](pofcon) module"]
pub type POFCON = crate::Reg<u32, _POFCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POFCON;
#[doc = "`read()` method returns [pofcon::R](pofcon::R) reader structure"]
impl crate::Readable for POFCON {}
#[doc = "`write(|w| ..)` method takes [pofcon::W](pofcon::W) writer structure"]
impl crate::Writable for POFCON {}
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
#[doc = "Enable DC/DC mode of the main voltage regulator\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcen](dcdcen) module"]
pub type DCDCEN = crate::Reg<u32, _DCDCEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCEN;
#[doc = "`read()` method returns [dcdcen::R](dcdcen::R) reader structure"]
impl crate::Readable for DCDCEN {}
#[doc = "`write(|w| ..)` method takes [dcdcen::W](dcdcen::W) writer structure"]
impl crate::Writable for DCDCEN {}
#[doc = "Enable DC/DC mode of the main voltage regulator"]
pub mod dcdcen;
