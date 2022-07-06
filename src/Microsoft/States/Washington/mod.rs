#[repr(transparent)]
pub struct IWashington(::windows::core::IUnknown);
impl IWashington {
    pub unsafe fn Load(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::windows::core::Interface::as_raw(self))
            .ok()
    }
    pub unsafe fn LoadFrom<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(
        &self,
        path: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadFrom)(
            ::windows::core::Interface::as_raw(self),
            path.into_param().abi(),
        )
        .ok()
    }
}
impl ::core::convert::From<IWashington> for ::windows::core::IUnknown {
    fn from(value: IWashington) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWashington> for ::windows::core::IUnknown {
    fn from(value: &IWashington) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWashington {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWashington {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWashington {
    type Vtable = IWashington_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa7c97d53_cf24_4453_bd17_55a48e1d0510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWashington_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadFrom: unsafe extern "system" fn(
        this: *mut ::core::ffi::c_void,
        path: ::windows::core::PCWSTR,
    ) -> ::windows::core::HRESULT,
}
#[repr(transparent)]
pub struct IWashington(::windows::core::IUnknown);
impl IWashington {
    pub unsafe fn Load(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load)(::windows::core::Interface::as_raw(self))
            .ok()
    }
    pub unsafe fn LoadFrom<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(
        &self,
        path: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadFrom)(
            ::windows::core::Interface::as_raw(self),
            path.into_param().abi(),
        )
        .ok()
    }
}
impl ::core::convert::From<IWashington> for ::windows::core::IUnknown {
    fn from(value: IWashington) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWashington> for ::windows::core::IUnknown {
    fn from(value: &IWashington) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWashington {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWashington {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWashington {
    type Vtable = IWashington_Vtbl;
    const IID: ::windows::core::GUID =
        ::windows::core::GUID::from_u128(0xa7c97d53_cf24_4453_bd17_55a48e1d0510);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWashington_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
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
        (::windows::core::Interface::vtable(self).base__.Load)(::windows::core::Interface::as_raw(
            self,
        ))
        .ok()
    }
    pub unsafe fn LoadFrom<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(
        &self,
        path: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LoadFrom)(
            ::windows::core::Interface::as_raw(self),
            path.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Load2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load2)(::windows::core::Interface::as_raw(self))
            .ok()
    }
}
impl ::core::convert::From<IWashington2> for ::windows::core::IUnknown {
    fn from(value: IWashington2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWashington2> for ::windows::core::IUnknown {
    fn from(value: &IWashington2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWashington2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWashington2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWashington2> for IWashington {
    fn from(value: IWashington2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWashington2> for IWashington {
    fn from(value: &IWashington2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWashington> for IWashington2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWashington> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWashington> for &'a IWashington2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWashington> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWashington2 {
    type Vtable = IWashington2_Vtbl;
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
#[repr(transparent)]
pub struct IWashington2(::windows::core::IUnknown);
impl IWashington2 {
    pub unsafe fn Load(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Load)(::windows::core::Interface::as_raw(
            self,
        ))
        .ok()
    }
    pub unsafe fn LoadFrom<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(
        &self,
        path: Param0,
    ) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LoadFrom)(
            ::windows::core::Interface::as_raw(self),
            path.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn Load2(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Load2)(::windows::core::Interface::as_raw(self))
            .ok()
    }
}
impl ::core::convert::From<IWashington2> for ::windows::core::IUnknown {
    fn from(value: IWashington2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWashington2> for ::windows::core::IUnknown {
    fn from(value: &IWashington2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWashington2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWashington2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWashington2> for IWashington {
    fn from(value: IWashington2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWashington2> for IWashington {
    fn from(value: &IWashington2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWashington> for IWashington2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWashington> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWashington> for &'a IWashington2 {
    fn into_param(self) -> ::windows::core::Param<'a, IWashington> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWashington2 {
    type Vtable = IWashington2_Vtbl;
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
