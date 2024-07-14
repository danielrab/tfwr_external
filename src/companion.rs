use rand::prelude::*;

use crate::{farm::SimplePlant, util::Position};

#[derive(Debug)]
pub struct Companion {
    position: Position,
    kind: SimplePlant,
}

impl Position {
    fn generate_companion_position(&self, farm_size: usize) -> Position {
        let mut rng = rand::thread_rng();
        let mut new_position;

        loop {
            // Generate random offsets within the Manhattan distance of 3
            let dx = rng.gen_range(-3..=3);
            let dy = rng.gen_range(-3..=3);

            new_position = Position {
                x: (self.x as isize + dx).rem_euclid(farm_size as isize) as usize,
                y: (self.y as isize + dy).rem_euclid(farm_size as isize) as usize,
            };
            
            // Ensure the new position is not the same as the current position and within distance
            if new_position != *self
                && self.wrapped_manhattan_distance(&new_position, farm_size, farm_size) <= 3
            {
                break new_position;
            }
        }
    }
}

impl Companion {
    pub fn new(plant_position: Position, farm_size: usize) -> Self {
        Self { position: plant_position.generate_companion_position(farm_size), kind: rand::random() }
    }
}