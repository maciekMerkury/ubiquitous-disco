use std::collections::{HashMap, HashSet};

use crate::pops;

#[derive(Default)]
pub struct DemandInputs {
    finance: f32,
    unit_price: f32,
    held_amount: usize,
    desired_amount: usize,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum PopDemandType {
    Simple,
    MoreComplex,
}

#[derive(Default)]
pub struct DemandCurves {
    curves: HashMap<PopDemandType, DemandCurve>,
}

impl DemandCurves {
    pub fn insert(
        &mut self,
        demand_type: PopDemandType,
        function: fn(DemandInputs) -> f32,
    ) -> Option<DemandCurve> {
        self.curves.insert(demand_type, DemandCurve::new(function))
    }
}

type Function = dyn Fn(DemandInputs) -> f32;

pub struct DemandCurve {
    func: Box<dyn Fn(DemandInputs) -> f32>,
}

impl DemandCurve {
    pub fn new(f: fn(DemandInputs) -> f32) -> Self {
        Self { func: Box::new(f) }
    }

    pub fn get_value(&self, inputs: DemandInputs) -> f32 {
        return (self.func)(inputs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_curve() {}
}
