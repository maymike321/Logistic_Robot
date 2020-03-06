use crate::enums::ProducerType;
use crate::materials::material::Material;
use fraction::Fraction;
    
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Recipe {
    pub producer_type: Option<ProducerType>,
    pub items: Vec<(Box<Material>, Fraction)>,
    pub time: Fraction,
    pub amount: Fraction
}
impl Recipe {
    pub fn new(producer_type: Option<ProducerType>, items: Vec<(Box<Material>, Fraction)>, time: Fraction, amount: Fraction) -> Recipe {
        Recipe {
            producer_type: producer_type,
            items: items,
            time: time,
            amount: amount
        }
    }
}