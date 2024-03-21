use washington_rs::States::*;
use windows::core::*;

#[derive(Debug)]
#[implement(IState)]
struct Atlantis {}

#[allow(non_snake_case)]
impl IState_Impl for Atlantis {
    fn GetFlower(&self) -> windows::core::Result<BSTR> {
        Ok("Red algae".into())
    }
}

fn main() -> windows::core::Result<()> {
    println!("{}", Atlantis {}.GetFlower()?);
    Ok(())
}
