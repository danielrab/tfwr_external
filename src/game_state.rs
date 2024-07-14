use std::time::Duration;

use crate::drone::Drone;
use crate::farm::Farm;



pub struct GameState {
    farm: Farm,
    farm_size: usize,
    drone: Drone,
    time_elapsed: Duration,
    op_count: u64,
}