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
        parent
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
            });
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
    let blade_material = materials.add(Color::from(GRAY));
    let handle_material = materials.add(Color::from(MAROON));
    let metal_material = materials.add(Color::from(SILVER));
    let accent_material = materials.add(Color::from(ORANGE_RED));

    // Create meshes
    let blade_mesh = meshes.add(Rectangle::new(80.0, 60.0)); // Wide axe blade
    let handle_mesh = meshes.add(Rectangle::new(15.0, 180.0)); // Long handle
    let blade_edge_mesh = meshes.add(Rectangle::new(85.0, 8.0)); // Sharp edge
    let pommel_mesh = meshes.add(Circle::new(10.0)); // Handle end

    // Spawn axe as child of parent entity
    commands.entity(parent_entity).with_children(|parent| {
        parent
            .spawn((
                Transform::from_translation(offset).with_scale(Vec3::splat(scale)),
                Visibility::default(),
                crate::weapon::Weapon::new(offset, scale),
            ))
            .with_children(|axe_parent| {
                // Add collider as a separate child entity (sensor only, no physics control)
                let axe_collider = axe_parent
                    .spawn((
                        Transform::from_xyz(0.0, 50.0, 0.0), // Offset to center the collider on the axe
                        Collider::cuboid(
                            (90.0 * scale) / 2.0,  // half_width (wider than sword for axe head)
                            (240.0 * scale) / 2.0, // half_height (total axe length scaled)
                        ),
                        ActiveEvents::COLLISION_EVENTS,
                        Sensor, // Makes it a sensor (no physics forces, just collision detection)
                        ColliderDisabled,
                        crate::damage::Damage::physical(35.0, parent_entity), // Higher damage than sword
                        crate::physics::WeaponKnockback::new(1000.0, 2.5), // Stronger knockback
                    ))
                    .id();
                global_entities
                    .weapon_collider
                    .insert(axe_parent.target_entity(), axe_collider);
                global_entities
                    .player_to_collider
                    .insert(player_entity, axe_collider);

                // Main axe blade
                axe_parent.spawn((
                    Mesh2d(blade_mesh),
                    MeshMaterial2d(blade_material.clone()),
                    Transform::from_xyz(0.0, 120.0, 0.0),
                ));

                // Axe blade edge (sharp part)
                axe_parent.spawn((
                    Mesh2d(blade_edge_mesh),
                    MeshMaterial2d(metal_material.clone()),
                    Transform::from_xyz(0.0, 150.0, 0.0),
                ));

                // Axe tip - pointed end
                let tip = axe_parent
                    .spawn((
                        Mesh2d(meshes.add(Triangle2d::new(
                            Vec2::new(0.0, 12.0),
                            Vec2::new(-8.0, -6.0),
                            Vec2::new(8.0, -6.0),
                        ))),
                        MeshMaterial2d(metal_material.clone()),
                        Transform::from_xyz(0.0, 162.0, 0.0),
                    ))
                    .id();
                global_entities
                    .player_sword_trail // Reusing the same trail system
                    .insert(player_entity, tip);

                // Axe handle/haft
                axe_parent.spawn((
                    Mesh2d(handle_mesh),
                    MeshMaterial2d(handle_material.clone()),
                    Transform::from_xyz(0.0, 10.0, 0.0),
                ));

                // Handle binding/wrapping - leather strips
                for i in 0..4 {
                    axe_parent.spawn((
                        Mesh2d(meshes.add(Rectangle::new(18.0, 4.0))),
                        MeshMaterial2d(materials.add(Color::from(BLACK))),
                        Transform::from_xyz(0.0, 20.0 + (i as f32 * 25.0), 0.1),
                    ));
                }

                // Pommel - handle end
                axe_parent.spawn((
                    Mesh2d(pommel_mesh),
                    MeshMaterial2d(accent_material.clone()),
                    Transform::from_xyz(0.0, -80.0, 0.0),
                ));

                // Axe head decoration - small rectangles on blade
                for i in 0..2 {
                    axe_parent.spawn((
                        Mesh2d(meshes.add(Rectangle::new(8.0, 8.0))),
                        MeshMaterial2d(accent_material.clone()),
                        Transform::from_xyz(-20.0 + (i as f32 * 40.0), 125.0, 0.1),
                    ));
                }

                // Axe head connector - where blade meets handle
                axe_parent.spawn((
                    Mesh2d(meshes.add(Rectangle::new(20.0, 25.0))),
                    MeshMaterial2d(metal_material),
                    Transform::from_xyz(0.0, 90.0, 0.0),
                ));
            });
    });
}