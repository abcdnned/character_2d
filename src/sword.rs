use bevy::prelude::*;
use bevy::color::palettes::basic::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Sword {
    pub offset: Vec3,
    pub scale: f32,
}

impl Sword {
    pub fn new(offset: Vec3, scale: f32) -> Self {
        Self { offset, scale }
    }
}

pub fn equip_sword(
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
            crate::sword::Sword::new(offset, scale),
        )).with_children(|sword_parent| {
            // Add collider as a separate child entity (sensor only, no physics control)
            sword_parent.spawn((
                Transform::from_xyz(0.0, 70.0, 0.0), // Offset to center the collider on the sword
                Collider::cuboid(
                    (40.0 * scale) / 2.0,  // half_width (blade width scaled)
                    (450.0 * scale) / 2.0, // half_height (total sword length scaled)
                ),
                ActiveEvents::COLLISION_EVENTS,
                Sensor, // Makes it a sensor (no physics forces, just collision detection)
                crate::damage::Damage::physical(25.0, parent_entity),
                crate::physics::WeaponKnockback::new(800.0, 2.25),
            ));
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