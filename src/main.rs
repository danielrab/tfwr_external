use farm::Farm;
use game_state::GameState;

mod game_state;
mod drone;
mod util;
mod farm;
mod companion;
fn main() {

    println!("{:?}", Farm::new(10));
}
