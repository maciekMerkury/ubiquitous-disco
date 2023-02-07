use crate::materials::Resource;
use std::collections::HashMap;

pub struct ResourceMarketData {
    count: usize,
    price: f32,
}

pub struct LocalMarket {
    resources: HashMap<Resource, ResourceMarketData>,
}
