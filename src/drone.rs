use enum_map::{Enum, EnumMap};

use crate::util::Position;

#[derive(Debug, Enum)]
enum Item {
    Carrot,
    CarrotSeed,
    EmptyTank,
    Fertilizer,
    Gold,
    Hay,
    Power,
    Pumpkin,
    PumpkinSeed,
    SunflowerSeed,
    WaterTank,
    Wood,
    Cactus,
    CactusSeed,
    Egg,
    Bones,
}

#[derive(Debug, Enum)]
enum Upgrade {
    Loops,
    Speed,
    Grass,
    Plant,
    Senses,
    Debug,
    Benchmark,
    Debug2,
    Carrots,
    Trees,
    Sunflowers,
    Cactus,
    Dinosaurs,
    Watering,
    MultiTrade,
    Expand,
    Operators,
    Variables,
    Functions,
    Utilities,
    Lists,
    Polyculture,
    Dictionaries,
    Costs,
    AutoUnlock,
    Leaderboard,
    Pumpkins,
    Fertilizer,
    Mazes,
}

#[derive(Debug, Default)]
pub struct Drone { 
    position: Position,
    upgrades: EnumMap<Upgrade, u8>,
    items: EnumMap<Item, u32>,
}