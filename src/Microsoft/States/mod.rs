pub mod Washington;
#[repr(transparent)]
pub struct IState(::windows::core::IUnknown);
impl IState {
    pub unsafe fn GetFlower(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFlower)(
            ::windows::core::Vtable::as_raw(self),
            ::core::mem::transmute(result__.as_mut_ptr()),
        )
        .from_abi::<::windows::core::BSTR>(result__)
    }
}
impl ::core::convert::From<IState> for ::windows::core::IUnknown {
    fn from(value: IState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IState> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IState> for ::windows::core::IUnknown {
    fn from(value: &IState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
unsafe impl ::windows::core::Vtable for IState {
    type Vtable = IState_Vtbl;
}
unsafe impl ::windows::core::Interface for IState {
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0x2f0cf45d_04a9_43cc_bc50_ebfe429fcecf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IState_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFlower: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        flower: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>,
    ) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
