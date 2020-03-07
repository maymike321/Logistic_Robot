use core::fmt::Error;
use core::fmt::Formatter;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ProducerType {
    AssemblingMachine,
    Furnace,
    ChemicalPlant,
    RocketSilo,
}

#[derive(Copy, Clone, Debug)]
pub enum AssemblingMachineLevel {
    One,
    Two,
    Three,
}
impl Display for AssemblingMachineLevel {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(
            formatter,
            "{}",
            match self {
                AssemblingMachineLevel::One => "1",
                AssemblingMachineLevel::Two => "2",
                AssemblingMachineLevel::Three => "3",
            }
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FurnaceLevel {
    Stone,
    Steel,
}
impl Display for FurnaceLevel {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(
            formatter,
            "{}",
            match self {
                FurnaceLevel::Stone => "Stone",
                FurnaceLevel::Steel => "Steel",
            }
        )
    }
}
