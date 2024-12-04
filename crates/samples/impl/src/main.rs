use washington_rs::Microsoft::States::*;
use windows_core::{implement, Result, BSTR};

#[derive(Debug)]
#[implement(IState)]
struct Atlantis;

#[allow(non_snake_case)]
impl IState_Impl for Atlantis_Impl {
    fn GetFlower(&self) -> Result<BSTR> {
        Ok("Red algae".into())
    }

    fn GetBool(&self) -> u32 {
        1
    }

    fn GetData(&self) -> *mut StateData {
        std::ptr::null_mut()
    }
}

fn main() -> windows::core::Result<()> {
    let com: IState = Atlantis {}.into();
    let s = unsafe { com.GetFlower() }?;
    println!("GetFlower: {}", s);
    println!("GetBool: {}", unsafe { com.GetBool() });
    Ok(())
}
