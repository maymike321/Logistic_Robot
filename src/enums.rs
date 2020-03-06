use core::fmt::Error;
use core::fmt::Formatter;
use std::fmt::Display;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ProducerType {
    AssemblyMachine,
    Furnace,
    ChemicalPlant,
    RocketSilo,
}

#[derive(Copy, Clone, Debug)]
pub enum AssemblyMachineLevel {
    One,
    Two,
    Three,
}
impl Display for AssemblyMachineLevel {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        write!(
            formatter,
            "{}",
            match self {
                AssemblyMachineLevel::One => "One",
                AssemblyMachineLevel::Two => "Two",
                AssemblyMachineLevel::Three => "Three",
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
