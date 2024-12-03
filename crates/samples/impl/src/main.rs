use washington_rs::States::*;
use windows::core::*;

#[derive(Debug)]
#[implement(IState)]
struct Atlantis;

#[allow(non_snake_case)]
impl IState_Impl for Atlantis_Impl {
    fn GetFlower(&self) -> Result<BSTR> {
        Ok("Red algae".into())
    }
}

fn main() -> windows::core::Result<()> {
    unsafe {
        let state: IState = Atlantis.into();
        println!("{}", state.GetFlower()?);
    }
    Ok(())
}
