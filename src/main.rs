use bevy::prelude::*;
use player::PlayerPlugin;
use world::WorldPlugin;

mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
