//! Shows how to render a sword mesh by combining multiple basic shapes in a 2D scene.
use bevy::{color::palettes::basic::*, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

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

    // Spawn sword as parent entity
    commands
        .spawn((
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            // Blade - main sword blade
            parent.spawn((
                Mesh2d(blade_mesh),
                MeshMaterial2d(blade_material.clone()),
                Transform::from_xyz(0.0, 60.0, 0.0),
            ));

            // Blade tip - triangular point
            parent.spawn((
                Mesh2d(meshes.add(Triangle2d::new(
                    Vec2::new(0.0, 15.0),
                    Vec2::new(-10.0, -7.5),
                    Vec2::new(10.0, -7.5),
                ))),
                MeshMaterial2d(blade_material),
                Transform::from_xyz(0.0, 167.5, 0.0),
            ));

            // Cross guard
            parent.spawn((
                Mesh2d(guard_mesh),
                MeshMaterial2d(guard_material),
                Transform::from_xyz(0.0, -40.0, 0.0),
            ));

            // Handle/grip
            parent.spawn((
                Mesh2d(handle_mesh),
                MeshMaterial2d(handle_material),
                Transform::from_xyz(0.0, -77.5, 0.0),
            ));

            // Pommel - round end piece
            parent.spawn((
                Mesh2d(pommel_mesh),
                MeshMaterial2d(pommel_material),
                Transform::from_xyz(0.0, -115.0, 0.0),
            ));

            // Handle wrapping details (optional decorative rectangles)
            for i in 0..3 {
                parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(14.0, 3.0))),
                    MeshMaterial2d(materials.add(Color::from(BLACK))),
                    Transform::from_xyz(0.0, -70.0 + (i as f32 * 10.0), 0.1),
                ));
            }
        });
}