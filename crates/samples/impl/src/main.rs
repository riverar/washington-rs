use washington_rs::IState_Impl;
use windows::core::*;

#[derive(Debug)]
#[implement(washington_rs::IState)]
struct Atlantis {}

impl IState_Impl for Atlantis
{
    fn GetFlower(&self) ->  windows::core::Result<BSTR> {
        Ok("Red algae".into())
    }
}

fn main() -> windows::core::Result<()> {
    println!("{}", Atlantis{}.GetFlower()?);
    Ok(())
}
