use crate::enums::*;
use crate::materials::material::Material;
use fraction::Fraction;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct TotalRawResult<'a> {
    pub producers: HashMap<&'a Material, (ProducerType, Fraction)>,
    pub total_raw: HashMap<&'a Material, Fraction>,
}
impl<'a> TotalRawResult<'a> {
    pub fn new(
        producers: HashMap<&'a Material, (ProducerType, Fraction)>,
        total_raw: HashMap<&'a Material, Fraction>,
    ) -> TotalRawResult<'a> {
        TotalRawResult {
            producers: producers,
            total_raw: total_raw,
        }
    }
    pub fn whole_ratio(
        &'a self,
        ignore_furnaces: bool,
        ignore_raw: bool,
    ) -> (Fraction, TotalRawResult<'a>) {
        let mut denominators: Vec<u64> = self
            .producers
            .iter()
            .filter(|(_material, (producer_type, _fraction))| {
                !ignore_furnaces || *producer_type != ProducerType::Furnace
            })
            .map(|(_material, (_producer_type, fraction))| fraction.denom().unwrap().clone())
            .collect();
        if !ignore_raw {
            denominators.extend(
                self.total_raw
                    .iter()
                    .map(|(_material, fraction)| fraction.denom().unwrap().clone()),
            );
        }
        let ratio = Fraction::from(lcm_multiple(denominators));
        let new_producers = self
            .producers
            .iter()
            .map(|(&material, (producer_type, fraction))| {
                (material, (*producer_type, fraction * &ratio))
            })
            .collect();
        let new_raw = self
            .total_raw
            .iter()
            .map(|(&material, &fraction)| (material, fraction * ratio))
            .collect();
        (ratio, TotalRawResult::new(new_producers, new_raw))
    }
}

fn lcm_multiple(numbers: Vec<u64>) -> u64 {
    numbers.iter().fold(1, |a, b| lcm(a, *b))
}

fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

fn gcd(a: u64, b: u64) -> u64 {
    match a.cmp(&b) {
        Ordering::Less => gcd(a, b - a),
        Ordering::Equal => a,
        Ordering::Greater => gcd(a - b, b),
    }
}
