pub trait IState_Impl: Sized {
    fn GetFlower(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
impl ::windows::core::RuntimeName for IState {}
impl IState_Vtbl {
    pub const fn new<
        Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
        Impl: IState_Impl,
        const OFFSET: isize,
    >() -> IState_Vtbl {
        unsafe extern "system" fn GetFlower<
            Identity: ::windows::core::IUnknownImpl<Impl = Impl>,
            Impl: IState_Impl,
            const OFFSET: isize,
        >(
            this: *mut ::core::ffi::c_void,
            flower: *mut ::core::mem::ManuallyDrop<::windows::core::BSTR>,
        ) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlower() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flower, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFlower: GetFlower::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IState as ::windows::core::Interface>::IID
    }
}
