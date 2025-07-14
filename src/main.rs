//! This example showcases a 2D top-down camera with smooth player tracking.
//!
//! ## Controls
//!
//! | Key Binding          | Action        |
//! |:---------------------|:--------------|
//! | `W`                  | Move up       |
//! | `S`                  | Move down     |
//! | `A`                  | Move left     |
//! | `D`                  | Move right    |

use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use crate::constants::*;
use bevy_rapier2d::prelude::*;
use crate::collider::*;

#[derive(Component)]
pub struct Player;

pub mod r#move;

mod enemy;
mod input;
mod input_move_map;
mod sword;
mod lerp_animation;
mod iterpolation;
mod movement;
mod constants;
mod collisions;
mod collider;
mod health_bar;
mod unit;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::input::InputPlugin)
        .add_plugins(crate::r#move::MovePlugin)
        .add_plugins(crate::health_bar::HealthBarPlugin)
        .add_plugins(crate::unit::UnitPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
        .add_systems(Update, (crate::input::handle_input, crate::input_move_map::input_map_to_move, crate::movement::move_player, crate::collisions::handle_collisions, update_camera).chain())
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // World where we move the player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(WORLD_COLOR)),
    ));

    // Player
    let player = commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(MESH_RADIUS))),
        MeshMaterial2d(materials.add(PLAYER_COLOR)), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(0., 0., 2.),
        DynamicPhysicsBundle::new_ball(MESH_RADIUS),
        crate::unit::Hp::new(50.0, 100.0),
    )).id();

    // Enemy - spawn a rectangle
    commands.spawn((
        crate::enemy::Enemy {},
        Mesh2d(meshes.add(Rectangle::new(MESH_RADIUS * 2., MESH_RADIUS * 2.))),
        MeshMaterial2d(materials.add(ENEMY_COLOR)), // Red color for enemy
        Transform::from_xyz(200., 150., 1.),
        DynamicPhysicsBundle::new_box(MESH_RADIUS, MESH_RADIUS),
        crate::unit::Hp::new(30.0, 30.0),
        crate::unit::Name::new("Guard"),
    ));

    crate::sword::equip_sword(&mut commands, &mut meshes, &mut materials, player, Vec3::new(50.0, 40.0, 0.1), 0.5);
}

fn setup_instructions(mut commands: Commands) {
    commands.spawn((
        Text::new("Move the light with WASD.\nThe camera will smoothly track the light."),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Bloom::NATURAL));
}

/// Update the camera position by tracking the player.
fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

