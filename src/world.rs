use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(build_world).add_system(rotate_box);
    }
}

fn rotate_box(mut query: Query<&mut Transform, With<Handle<Mesh>>>) {
    for mut i in query.iter_mut() {
        i.rotate(Quat::from_euler(EulerRot::XYZ, 0.0, 0.01, 0.0));
    }
}

fn build_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(1.0, 1.0, 1.0).into()),
        material: materials.add(Color::WHITE.into()),
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 6000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(-2.0, 4.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
