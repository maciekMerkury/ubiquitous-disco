use crate::demand::{DemandCurve, PopDemandType};
use crate::materials::Resource;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct ResourceData {
    pub count: usize,
    pub desired_count: usize,
    pub demand_type: PopDemandType,
}

pub struct Person {
    pub health: u8,
    pub funds: usize,
    pub resources: HashMap<Resource, ResourceData>,
}

impl Default for Person {
    fn default() -> Self {
        use Resource::*;

        let mut res = HashMap::new();

        let data = ResourceData {
            count: 0,
            desired_count: 0,
            demand_type: PopDemandType::Simple,
        };

        for r in [RawMaterials, FinishedGoods, Food, Energy] {
            res.insert(r, data);
        }

        Self {
            health: u8::MAX,
            funds: 0,
            resources: res,
        }
    }
}

impl Person {
}
