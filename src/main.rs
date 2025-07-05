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
use bevy::color::palettes::basic::*;


/// Player movement speed factor.
const PLAYER_SPEED: f32 = 300.;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 5.;

#[derive(Component)]
pub struct Player;

pub mod r#move;

mod input;
mod input_move_map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(crate::input::InputPlugin)
        .add_systems(Startup, (setup_scene, setup_instructions, setup_camera))
        .add_systems(Update, (crate::input::handle_input, crate::input_move_map::input_map_to_move, move_player, update_camera).chain())
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
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));

    // Player
    let player = commands.spawn((
        Player,
        Mesh2d(meshes.add(Circle::new(25.))),
        MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))), // RGB values exceed 1 to achieve a bright color for the bloom effect
        Transform::from_xyz(0., 0., 2.),
    )).id();

    equip_sword(&mut commands, &mut meshes, &mut materials, player, Vec3::new(50.0, 40.0, 0.1), 0.5);
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

/// Update the player position with keyboard inputs.
/// Note that the approach used here is for demonstration purposes only,
/// as the point of this example is to showcase the camera tracking feature.
///
/// A more robust solution for player movement can be found in `examples/movement/physics_in_fixed_timestep.rs`.
fn move_player(
    mut player: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
    mut move_events: EventReader<crate::input::MoveEvent>
) {
    let mut direction = Vec2::ZERO;
    
    // Accumulate all movement events this frame
    for event in move_events.read() {
        direction += event.direction;
    }
    // Progressively update the player's position over time. Normalize the
    // direction vector to prevent it from exceeding a magnitude of 1 when
    // moving diagonally.
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);
}

fn equip_sword(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parent_entity: Entity,
    offset: Vec3,
    scale: f32,
) {
    // Create materials
    let blade_material = materials.add(Color::from(SILVER));
    let guard_material = materials.add(Color::from(GRAY));
    let handle_material = materials.add(Color::from(MAROON));
    let pommel_material = materials.add(Color::from(YELLOW));

    // Create meshes
    let blade_mesh = meshes.add(Rectangle::new(20.0, 200.0));
    let guard_mesh = meshes.add(Rectangle::new(80.0, 15.0));
    let handle_mesh = meshes.add(Rectangle::new(12.0, 60.0));
    let pommel_mesh = meshes.add(Circle::new(12.0));

    // Spawn sword as child of parent entity
    commands.entity(parent_entity).with_children(|parent| {
        parent.spawn((
            Transform::from_translation(offset).with_scale(Vec3::splat(scale)),
            Visibility::default(),
        )).with_children(|sword_parent| {
            // Blade - main sword blade
            sword_parent.spawn((
                Mesh2d(blade_mesh),
                MeshMaterial2d(blade_material.clone()),
                Transform::from_xyz(0.0, 60.0, 0.0),
            ));

            // Blade tip - triangular point
            sword_parent.spawn((
                Mesh2d(meshes.add(Triangle2d::new(
                    Vec2::new(0.0, 15.0),
                    Vec2::new(-10.0, -7.5),
                    Vec2::new(10.0, -7.5),
                ))),
                MeshMaterial2d(blade_material),
                Transform::from_xyz(0.0, 167.5, 0.0),
            ));

            // Cross guard
            sword_parent.spawn((
                Mesh2d(guard_mesh),
                MeshMaterial2d(guard_material),
                Transform::from_xyz(0.0, -40.0, 0.0),
            ));

            // Handle/grip
            sword_parent.spawn((
                Mesh2d(handle_mesh),
                MeshMaterial2d(handle_material),
                Transform::from_xyz(0.0, -77.5, 0.0),
            ));

            // Pommel - round end piece
            sword_parent.spawn((
                Mesh2d(pommel_mesh),
                MeshMaterial2d(pommel_material),
                Transform::from_xyz(0.0, -115.0, 0.0),
            ));

            // Handle wrapping details (optional decorative rectangles)
            for i in 0..3 {
                sword_parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(14.0, 3.0))),
                    MeshMaterial2d(materials.add(Color::from(BLACK))),
                    Transform::from_xyz(0.0, -69.0 + (i as f32 * 10.0), 0.1),
                ));
            }
        });
    });
}