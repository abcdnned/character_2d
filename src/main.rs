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

use crate::ai::AIPlugin;
use crate::berserker::Berserker;
use crate::berserker::BerserkerPlugin;
use crate::collider::*;
use crate::constants::*;
use crate::float_text::FloatingTextPlugin;
use crate::force::Force;
use crate::global_entity_map::*;
use crate::move_components::MoveComponentsPlugin;
use crate::movement::SprintCD;
use crate::particle::ParticlePlugin;
use crate::rotation::RotationPlugin;
use crate::unit::Unit;
use crate::unit_death::UnitDeathPlugin;
use bevy::log::LogPlugin;
use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_tweening::TweeningPlugin;

#[derive(Component)]
pub struct Player;

pub mod custom_move;

mod ai;
mod animation_base;
mod berserker;
mod collider;
mod collisions;
mod constants;
mod damage;
mod enemy;
mod float_text;
mod force;
mod global_entity_map;
mod health_bar;
mod input;
mod input_move_map;
mod iterpolation;
mod lerp_animation;
mod move_components;
mod move_database;
mod movement;
mod particle;
mod physics;
mod rotation;
mod sword_trail;
mod unit;
mod unit_death;
mod weapon;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,character_2d::berserker=debug,character_2d::movement=warn".to_string(), // Specific filters
            ..Default::default()
        }))
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(crate::input::InputPlugin)
        .add_plugins(crate::custom_move::MovePlugin)
        .add_plugins(crate::health_bar::HealthBarPlugin)
        .add_plugins(crate::unit::UnitPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(crate::sword_trail::SwordTrailPlugin)
        .add_plugins(GlobalEntityMapPlugin)
        .add_plugins(MoveComponentsPlugin)
        .add_plugins(crate::animation_base::AnimationDatabasePlugin)
        .add_plugins(AIPlugin)
        .add_plugins(RotationPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(TweeningPlugin)
        .add_plugins(FloatingTextPlugin)
        .add_plugins(UnitDeathPlugin)
        .add_plugins(BerserkerPlugin)
        .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
        .add_systems(
            Update,
            (
                crate::input::handle_input,
                crate::input_move_map::input_map_to_move,
                crate::movement::move_player,
                crate::collisions::handle_collisions,
                crate::physics::update_knockback_timers,
                update_camera,
            )
                .chain(),
        )
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut global_map: ResMut<GlobalEntityMap>,
) {
    // World where we move the player
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(WORLD_COLOR)),
    ));

    // Player
    let player = commands
        .spawn((
            Player,
            Berserker{level: 0},
            Mesh2d(meshes.add(Circle::new(MESH_RADIUS))),
            MeshMaterial2d(materials.add(PLAYER_COLOR)), // RGB values exceed 1 to achieve a bright color for the bloom effect
            Transform::from_xyz(0., 0., 2.),
            DynamicPhysicsBundle::new_ball(MESH_RADIUS),
            Velocity::zero(),
            SprintCD(0.0),
            Unit::builder()
                .name("Hero")
                .hp(100.0)
                .unitType(unit::UnitType::Hero)
                .build(),
            Force {
                force: FORCE_PLAYER,
            },
            crate::ai::TargetDetector {
                target: Entity::PLACEHOLDER,      // No target initially
                alert_range: ALERT_RANGE,         // Alert range of 100 units
                dis_alert_range: DIS_ALERT_RANGE, // Disengage range slightly larger
                lock_type: ai::LockType::Free,
            },
        ))
        .with_children(|parent| {
            // Left eye (larger)
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(MESH_RADIUS * 0.15))),
                MeshMaterial2d(materials.add(Color::BLACK)),
                Transform::from_xyz(-MESH_RADIUS * 0.3, MESH_RADIUS * 0.2, 0.1),
            ));

            // Right eye (smaller)
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(MESH_RADIUS * 0.1))),
                MeshMaterial2d(materials.add(Color::BLACK)),
                Transform::from_xyz(MESH_RADIUS * 0.4, MESH_RADIUS * 0.25, 0.1),
            ));
        })
        .id();

    // Enemy - spawn a rectangle
    let enemy = commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(MESH_RADIUS * 2., MESH_RADIUS * 2.))),
            MeshMaterial2d(materials.add(ENEMY_COLOR)), // Red color for enemy
            Transform::from_xyz(200., 150., 1.),
            DynamicPhysicsBundle::new_box(MESH_RADIUS, MESH_RADIUS),
            Velocity::zero(),
            Unit::builder()
                .name("Guard")
                .max_hp(50.0)
                .unitType(unit::UnitType::SwordMan)
                .build(),
            crate::ai::TargetDetector {
                target: Entity::PLACEHOLDER,      // No target initially
                alert_range: ALERT_RANGE,         // Alert range of 100 units
                dis_alert_range: DIS_ALERT_RANGE, // Disengage range slightly larger
                lock_type: ai::LockType::Lock,
            },
            Force { force: FORCE_ENEMY },
            crate::ai::AI::new(
                global_map
                    .unittype_aioptions
                    .get(&unit::UnitType::SwordMan) // <-- take reference
                    .cloned() // <-- clone Vec<AIOption>
                    .unwrap_or_default(), // <-- fallback empty vec
            ),
        ))
        .with_children(|parent| {
            // Left eye (smaller for enemy)
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(MESH_RADIUS * 0.12))),
                MeshMaterial2d(materials.add(Color::BLACK)),
                Transform::from_xyz(-MESH_RADIUS * 0.4, MESH_RADIUS * 0.3, 0.1),
            ));

            // Right eye (larger for enemy)
            parent.spawn((
                Mesh2d(meshes.add(Circle::new(MESH_RADIUS * 0.18))),
                MeshMaterial2d(materials.add(Color::BLACK)),
                Transform::from_xyz(MESH_RADIUS * 0.35, MESH_RADIUS * 0.4, 0.1),
            ));
        })
        .id();

    crate::weapon::equip_sword(
        &mut commands,
        &mut meshes,
        &mut materials,
        player,
        Vec3::new(50.0, 40.0, 0.1),
        0.5,
        &mut global_map,
    );

    crate::weapon::equip_sword(
        &mut commands,
        &mut meshes,
        &mut materials,
        enemy,
        Vec3::new(50.0, 40.0, 0.1),
        0.5,
        &mut global_map,
    );
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
