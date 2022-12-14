use super::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawBundle {
    pub biomes: Option<Vec<Biome>>,
    pub plants: Option<Vec<PlantDef>>,
    pub materials: Option<Vec<MaterialDef>>,
}

impl RawBundle {
    pub fn load(filename: &str) -> Self {
        println!("loading raw bundle: {}", filename);
        let f = File::open(filename).expect("Failed opening file");

        let bundle: RawBundle = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load bundle list: {}: {:?}", filename, e);
                std::process::exit(1);
            }
        };
        bundle
    }

    pub fn merge(&self, raws: &mut crate::raws::Raws) {
        if let Some(biomes) = &self.biomes {
            raws.biomes.areas.extend_from_slice(biomes);
        }
        if let Some(materials) = &self.materials {
            raws.materials.materials.extend_from_slice(materials);
        }
        if let Some(plants) = &self.plants {
            raws.plants.plants.extend_from_slice(plants);
        }
    }
}
