use std::f32::consts::PI;

use crate::global_entity_map::*;
use bevy::color::palettes::basic::*;
use bevy::color::palettes::css::ORANGE_RED;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Weapon {
    pub offset: Vec3,
    pub scale: f32,
}

impl Weapon {
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
    global_entities: &mut ResMut<GlobalEntityMap>,
    player_entity: Entity,
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
        let sword = parent
            .spawn((
                Transform::from_translation(offset).with_scale(Vec3::splat(scale)),
                Visibility::default(),
                crate::weapon::Weapon::new(offset, scale),
            ))
            .with_children(|sword_parent| {
                // Add collider as a separate child entity (sensor only, no physics control)
                let sword_collider = sword_parent
                    .spawn((
                        Transform::from_xyz(0.0, 70.0, 0.0), // Offset to center the collider on the sword
                        Collider::cuboid(
                            (40.0 * scale) / 2.0,  // half_width (blade width scaled)
                            (450.0 * scale) / 2.0, // half_height (total sword length scaled)
                        ),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor, // Makes it a sensor (no physics forces, just collision detection)
                        ColliderDisabled,
                        crate::damage::Damage::physical(25.0, parent_entity),
                        crate::physics::WeaponKnockback::new(800.0, 2.25),
                    ))
                    .id();
                global_entities
                    .weapon_collider
                    .insert(sword_parent.target_entity(), sword_collider);
                global_entities
                    .player_to_collider
                    .insert(player_entity, sword_collider);
                // Blade - main sword blade
                sword_parent.spawn((
                    Mesh2d(blade_mesh),
                    MeshMaterial2d(blade_material.clone()),
                    Transform::from_xyz(0.0, 60.0, 0.0),
                ));

                // Blade tip - triangular point
                let tip = sword_parent
                    .spawn((
                        Mesh2d(meshes.add(Triangle2d::new(
                            Vec2::new(0.0, 15.0),
                            Vec2::new(-10.0, -7.5),
                            Vec2::new(10.0, -7.5),
                        ))),
                        MeshMaterial2d(blade_material),
                        Transform::from_xyz(0.0, 167.5, 0.0),
                    ))
                    .id();
                global_entities
                    .player_sword_trail
                    .insert(player_entity, tip);

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
            }).id();
            global_entities.player_weapon.insert(player_entity, sword);
    });
}

pub fn equip_axe(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parent_entity: Entity,
    offset: Vec3,
    scale: f32,
    global_entities: &mut ResMut<GlobalEntityMap>,
    player_entity: Entity,
) {
    // Create materials
    let blade_material = materials.add(Color::from(SILVER));
    let handle_material = materials.add(Color::from(MAROON));
    let accent_material = materials.add(Color::from(YELLOW));
    let tip_material = materials.add(Color::from(GRAY));

    // Create custom meshes for detailed axe blades
    // Create custom meshes for detailed axe blades
    let left_blade_vertices = vec![
        [-10.0, 20.0, 0.0],    // Top attach point
        [-35.0, 20.0, 0.0],    // Upper blade curve
        [-55.0, 50.0, 0.0],     // Far cutting edge point
        [-65.0, 20.0, 0.0],    // Sharp cutting tip
        [-65.0, -20.0, 0.0],    // Sharp cutting tip
        [-55.0, -50.0, 0.0],     // Far cutting edge point
        [-35.0, -20.0, 0.0],    // Upper blade curve
        [-10.0, -20.0, 0.0],    // Top attach point
    ];
    
    let mut left_blade_mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList, bevy::render::render_asset::RenderAssetUsages::default());
    
    // Create triangles by connecting vertices to center point (fan triangulation)
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // Add blade vertices
    for vertex in &left_blade_vertices {
        vertices.push(*vertex);
    }
    //     2
    // 0 1   3
    // 7 6   4
    //     5
    
    // Create triangle indices (fan from center)
    indices.extend_from_slice(&[0, 1, 6]);
    indices.extend_from_slice(&[0, 7, 6]);
    indices.extend_from_slice(&[1, 2, 3]);
    indices.extend_from_slice(&[6, 4, 5]);
    indices.extend_from_slice(&[1, 4, 3]);
    indices.extend_from_slice(&[1, 4, 6]);
    
    left_blade_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone());
    left_blade_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices.clone()));
    
    // Create right blade (mirrored)
    let right_blade_vertices = vec![
        [10.0, 20.0, 0.0],    // Top attach point
        [35.0, 20.0, 0.0],    // Upper blade curve
        [55.0, 50.0, 0.0],     // Far cutting edge point
        [65.0, 20.0, 0.0],    // Sharp cutting tip
        [65.0, -20.0, 0.0],    // Sharp cutting tip
        [55.0, -50.0, 0.0],     // Far cutting edge point
        [35.0, -20.0, 0.0],    // Upper blade curve
        [10.0, -20.0, 0.0],    // Top attach point
    ];
    
    let mut right_blade_mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList, bevy::render::render_asset::RenderAssetUsages::default());
    
    let mut vertices = Vec::new();
    // Add blade vertices
    for vertex in &right_blade_vertices {
        vertices.push(*vertex);
    }
    right_blade_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    right_blade_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    
    let left_blade_mesh = meshes.add(left_blade_mesh);
    let right_blade_mesh = meshes.add(right_blade_mesh);
    let handle_mesh = meshes.add(Rectangle::new(12.0, 160.0));
    let pommel_mesh = meshes.add(Circle::new(8.0));

    // Spawn axe as child of parent entity
    commands.entity(parent_entity).with_children(|parent| {
        parent
            .spawn((
                Transform::from_translation(offset).with_scale(Vec3::splat(scale)),
                Visibility::default(),
                crate::weapon::Weapon::new(offset, scale),
            ))
            .with_children(|axe_parent| {
                // Add collider for the double-bladed axe head
                let axe_collider = axe_parent
                    .spawn((
                        Transform::from_xyz(0.0, 120.0, 0.0),
                        Collider::cuboid(
                            (300.0 * scale) / 2.0, // Wider for double blades
                            (200.0 * scale) / 2.0, // Total axe length scaled
                        ),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor,
                        ColliderDisabled,
                        crate::damage::Damage::physical(40.0, parent_entity), // High axe damage
                        crate::physics::WeaponKnockback::new(1200.0, 3.0), // Strong knockback
                    ))
                    .id();
                global_entities
                    .weapon_collider
                    .insert(axe_parent.target_entity(), axe_collider);
                global_entities
                    .player_to_collider
                    .insert(player_entity, axe_collider);

                // Left axe blade
                axe_parent.spawn((
                    Mesh2d(left_blade_mesh),
                    MeshMaterial2d(blade_material.clone()),
                    Transform::from_xyz(0.0, 120.0, 0.0),
                ));

                // Right axe blade
                axe_parent.spawn((
                    Mesh2d(right_blade_mesh),
                    MeshMaterial2d(blade_material.clone()),
                    Transform::from_xyz(0.0, 120.0, 0.0),
                ));

                // Axe head center connector
                let connector = axe_parent
                    .spawn((
                        Mesh2d(meshes.add(Rectangle::new(20.0, 40.0))),
                        MeshMaterial2d(tip_material.clone()),
                        Transform::from_xyz(0.0, 120.0, 0.1),
                    ))
                    .id();
                global_entities
                    .player_sword_trail // Reusing trail system for visual effects
                    .insert(player_entity, connector);

                // Handle/haft
                axe_parent.spawn((
                    Mesh2d(handle_mesh),
                    MeshMaterial2d(handle_material.clone()),
                    Transform::from_xyz(0.0, 20.0, 0.0),
                ));

                // Handle grip wrapping (single band)
                axe_parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(15.0, 40.0))),
                    MeshMaterial2d(materials.add(Color::from(BLACK))),
                    Transform::from_xyz(0.0, 30.0, 0.1),
                ));

                // Pommel - handle end
                axe_parent.spawn((
                    Mesh2d(pommel_mesh),
                    MeshMaterial2d(accent_material),
                    Transform::from_xyz(0.0, -60.0, 0.0),
                ));
            });
    });
}

pub enum GearSet {
    LongSword,
    DoubleEdgeAxe,
}