use bevy::prelude::*;
use bevy_ratatui_camera::{RatatuiCamera, RatatuiCameraPlugin, RatatuiCameraStrategy};

pub fn plugin(app: &mut App) {
    app.add_plugins(RatatuiCameraPlugin)
        .add_systems(Startup, (camera_setup, scene_setup))
        .add_systems(Update, spin_system);
}

#[derive(Component, Debug, Default)]
pub struct Spinner;

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera {
            order: 1,
            ..default()
        },
        Camera3d::default(),
        Transform::from_xyz(2., 2., 2.).looking_at(Vec3::ZERO, Vec3::Y),
        RatatuiCamera::default(),
        RatatuiCameraStrategy::luminance_misc(),
    ));
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Spinner,
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.4, 0.54, 0.7),
            ..Default::default()
        })),
    ));
    commands.spawn((
        PointLight {
            intensity: 2_000_000.,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(3., 4., 6.),
    ));
}

fn spin_system(mut spinner: Single<&mut Transform, With<Spinner>>, time: Res<Time>) {
    spinner.rotate_y(time.delta_secs());
}
