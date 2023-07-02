use std::f32::consts::TAU;

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(build_player);
    }
}

fn build_player(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            fov: TAU / 4.0,
            ..Default::default()
        }),
        transform: Transform::from_xyz(0.0, 2.0, -3.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
