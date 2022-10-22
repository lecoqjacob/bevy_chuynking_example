use crate::prelude::*;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;

lazy_static! {
    pub static ref REGIONS: Lazy<RwLock<Regions>> = Lazy::new(|| RwLock::new(Regions::new()));
}

#[derive(Default)]
pub struct Regions {
    pub regions: HashMap<usize, Region>,
}

impl Regions {
    pub fn new() -> Self { Self { regions: HashMap::new() } }

    #[inline]
    pub fn get_region(&self, key: usize) -> Option<&Region> { self.regions.get(&key) }

    #[inline]
    pub fn get_region_mut(&mut self, key: usize) -> Option<&mut Region> {
        self.regions.get_mut(&key)
    }
}

pub struct Region {
    pub location: PlanetLocation,
}

impl Region {
    pub fn new(location: PlanetLocation) -> Self { Self { location } }
}

impl BaseMap for Region {}

impl Algorithm2D for Region {
    fn dimensions(&self) -> Point { Point::new(REGION_WIDTH, REGION_HEIGHT) }
}
