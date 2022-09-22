pub trait IWashington_Impl: Sized {
    fn Load(&self) -> ::windows::core::Result<()>;
    fn LoadFrom(&self, path: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWashington {}
impl IWashington_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IWashington_Impl,
        const OFFSET: isize,
    >() -> IWashington_Vtbl {
        unsafe extern "system" fn Load<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWashington_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load().into()
        }
        unsafe extern "system" fn LoadFrom<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWashington_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            path: ::windows::core::PCWSTR,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadFrom(::core::mem::transmute(&path)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Load: Load::<Identity, Impl, OFFSET>,
            LoadFrom: LoadFrom::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWashington as ::windows::core::Interface>::IID
    }
}
pub trait IWashington2_Impl: Sized + IWashington_Impl {
    fn Load2(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWashington2 {}
impl IWashington2_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IWashington2_Impl,
        const OFFSET: isize,
    >() -> IWashington2_Vtbl {
        unsafe extern "system" fn Load2<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IWashington2_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load2().into()
        }
        Self {
            base__: IWashington_Vtbl::new::<Identity, Impl, OFFSET>(),
            Load2: Load2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWashington2 as ::windows::core::Interface>::IID
            || iid == &<IWashington as ::windows::core::Interface>::IID
    }
}
