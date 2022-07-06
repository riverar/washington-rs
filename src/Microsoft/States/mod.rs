pub mod Washington;
#[repr(transparent)]
pub struct IState(::windows::core::IUnknown);
impl IState {
    pub unsafe fn GetFlower(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<
            ::core::mem::ManuallyDrop<::windows::Win32::Foundation::BSTR>,
        >::zeroed();
        (::windows::core::Interface::vtable(self).GetFlower)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<::windows::Win32::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IState> for ::windows::core::IUnknown {
    fn from(value: IState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IState> for ::windows::core::IUnknown {
    fn from(value: &IState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IState {}
impl ::core::fmt::Debug for IState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IState {
    type Vtable = IState_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2f0cf45d_04a9_43cc_bc50_ebfe429fcecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IState_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetFlower: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        flower: *mut ::windows::Win32::Foundation::BSTR,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IState(::windows::core::IUnknown);
impl IState {
    pub unsafe fn GetFlower(&self) -> ::windows::core::Result<::windows::Win32::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::<
            ::core::mem::ManuallyDrop<::windows::Win32::Foundation::BSTR>,
        >::zeroed();
        (::windows::core::Interface::vtable(self).GetFlower)(
            ::windows::core::Interface::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<::windows::Win32::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IState> for ::windows::core::IUnknown {
    fn from(value: IState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IState> for ::windows::core::IUnknown {
    fn from(value: &IState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IState {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IState {}
impl ::core::fmt::Debug for IState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IState {
    type Vtable = IState_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2f0cf45d_04a9_43cc_bc50_ebfe429fcecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IState_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetFlower: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        flower: *mut ::windows::Win32::Foundation::BSTR,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
