#[repr(transparent)]
pub struct IWashington(::windows::core::IUnknown);
impl IWashington {
    pub unsafe fn Load(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Load)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn LoadFrom<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).LoadFrom)(
            ::windows::core::Vtable::as_raw(self),
            path.into(),
        )
        .ok()
    }
}
::windows::core::interface_hierarchy!(IWashington, ::windows::core::IUnknown);
impl ::core::clone::Clone for IWashington {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWashington {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWashington {}
impl ::core::fmt::Debug for IWashington {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWashington").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWashington {
    type Vtable = IWashington_Vtbl;
}
unsafe impl ::windows::core::Interface for IWashington {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa7c97d53_cf24_4453_bd17_55a48e1d0510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWashington_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadFrom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IWashington2(::windows::core::IUnknown);
impl IWashington2 {
    pub unsafe fn Load(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Load)(::windows::core::Vtable::as_raw(self))
            .ok()
    }
    pub unsafe fn LoadFrom<'a, P0>(&self, path: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Vtable::vtable(self).base__.LoadFrom)(
            ::windows::core::Vtable::as_raw(self),
            path.into(),
        )
        .ok()
    }
    pub unsafe fn Load2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Load2)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IWashington2, ::windows::core::IUnknown, IWashington);
impl ::core::clone::Clone for IWashington2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWashington2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWashington2 {}
impl ::core::fmt::Debug for IWashington2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWashington2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IWashington2 {
    type Vtable = IWashington2_Vtbl;
}
unsafe impl ::windows::core::Interface for IWashington2 {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xe27af699_bc37_47b8_ad97_1c6720389efd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWashington2_Vtbl {
    pub base__: IWashington_Vtbl,
    pub Load2:
        unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
