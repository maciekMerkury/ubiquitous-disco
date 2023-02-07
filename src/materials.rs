#[non_exhaustive]
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum Resource {
    RawMaterials,
    FinishedGoods,
    Food,
    Energy,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {}
}
