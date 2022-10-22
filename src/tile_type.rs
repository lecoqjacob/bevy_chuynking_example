use enumflags2::bitflags;
use serde::{Deserialize, Serialize};

#[bitflags]
#[repr(u8)]
#[derive(Default, Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum TileType {
    #[default]
    Floor,
    Wall,
    Water,
    Grass,
}
