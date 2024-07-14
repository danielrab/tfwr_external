use std::time::Duration;
use std::iter;

use standard_dist::StandardDist;

use crate::companion::Companion;
use crate::util::Position;

#[derive(Debug, Default)]
enum GroundType {
    #[default]
    Turf,
    Soil,
}

#[derive(Debug, StandardDist)]
enum DinoColor {
    Black,
    White,
    Brown,
    Grey,
}

#[derive(Debug, StandardDist)]
pub enum SimplePlant {
    Grass,
    Bush,
    Tree,
    Carrots,
}

#[derive(Debug)]
enum EntityData {
    SimplePlant { kind: SimplePlant, companion: Companion },
    Pumpkin,
    Cactus { size: u8 },
    Sunflower { petals_count: u8 },
    Dinosaur { color: DinoColor },
}
impl EntityData {
    fn simple(kind: SimplePlant, farm_size: usize, position: Position) -> Self {
        Self::SimplePlant { kind, companion: Companion::new(position, farm_size) }
    }
}

#[derive(Debug)]
struct Entity {
    data: EntityData,
    growth_time: Duration,
    elapsed_time: Duration,
}
impl Entity {
    fn new(data: EntityData) -> Entity{
        let growth_time = Duration::ZERO;
        Self {
            data,
            growth_time,
            elapsed_time: Duration::ZERO,
        }
    }
}

#[derive(Debug)]
struct Tile {
    ground_type: GroundType,
    entity: Option<Entity>,
    water_level: f64,
}
impl Tile {
    fn new(farm_size: usize, position: Position) -> Self {
        Self {
            ground_type: GroundType::default(),
            entity: Some(Entity::new(EntityData::simple(SimplePlant::Grass, farm_size, position))),
            water_level: 0.0,
        }
    }
}

#[derive(Debug)]
struct NormalFarm {
    tiles: Vec<Vec<Tile>>,
}
impl NormalFarm {
    fn new(farm_size: usize) -> Self {
        Self {tiles: (0..farm_size).map(|x| {(0..farm_size).map(|y| Tile::new(farm_size, Position {x, y})).collect()}).collect()}
    }
}

#[derive(Debug)]
struct Maze {
    chest: Position,
    next_chest: Position,
    walls_horizontal: Vec<bool>,
    walls_vertical: Vec<bool>,
}

#[derive(Debug)]
pub enum Farm {
    Normal(NormalFarm),
    Maze(Maze),
}
impl Farm {
    pub fn new(farm_size: usize) -> Self {
        Farm::Normal(NormalFarm::new(farm_size))
    }
}