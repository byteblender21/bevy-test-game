use std::ops::{Add, Mul};

use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::*;
use bevy::utils::default;
use bevy_rapier3d::prelude::{ActiveEvents, Collider, CollisionEvent, Friction, GravityScale, RigidBody, Sensor};
use hexx::algorithms::a_star;
use hexx::Hex;

use crate::{HexLocation, Map};

pub struct EnemyPlugin;

pub struct EnemyArrivedAtEnd(Entity);

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EnemyArrivedAtEnd>()
            .add_startup_system(spawn_initial_enemy.in_base_set(StartupSet::PostStartup))
            .add_system(enemy_walking)
            .add_system(handle_enemy_events)
            .add_system(collision_event_handler)
        ;
    }
}

#[derive(Component)]
pub struct EnemyTag;

#[derive(Component)]
pub struct WalkingPath {
    path: Vec<Hex>,
    next_location: Hex,
}

fn spawn_initial_enemy(
    mut commands: Commands,
    map: Res<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    spawn_enemy(
        &mut commands,
        &map,
        &mut meshes,
        &mut materials
    );
}

fn enemy_walking(
    mut event_writer: EventWriter<EnemyArrivedAtEnd>,
    mut enemies: Query<(&mut Transform, &mut WalkingPath, &mut HexLocation, Entity), (With<EnemyTag>)>,
    time: Res<Time>,
    map: Res<Map>,
) {
    for (mut transform, mut walking_path, mut location, e) in &mut enemies {
        let mut current_pos = transform.translation;

        let next_location = walking_path.next_location;
        let future_pos = map.layout.hex_to_world_pos(next_location);

        let movement_vec = Vec3::new(
            future_pos.x - current_pos.x,
            0.0,
            future_pos.y - current_pos.z,
        );

        if approximate_pos(movement_vec) == Vec3::ZERO {

            if location.location == next_location {
                event_writer.send(EnemyArrivedAtEnd(e));
            } else {
                location.location = next_location;
                let mut updated_next_location: Option<Hex> = None;

                walking_path.path.windows(2).for_each(|two| {
                    let h1 = two.first().unwrap();
                    let h2 = two.last().unwrap();

                    if next_location == *h1 {
                        updated_next_location = Some(*h2);
                    }
                });

                if let Some(next_location) = updated_next_location {
                    walking_path.next_location = next_location;
                }
            }

        } else {
            transform.translation = current_pos.add(movement_vec.mul(time.delta_seconds() * 1.1));
        }
    }
}

fn approximate_pos(input: Vec3) -> Vec3 {
    return Vec3::new(
        (input.x * 7.0).trunc(),
        (input.y * 7.0).trunc(),
        (input.z * 7.0).trunc(),
    );
}

fn handle_enemy_events(
    mut walking_er: EventReader<EnemyArrivedAtEnd>,
    mut commands: Commands,
    map: Res<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in &mut walking_er {
        let enemy_entity = event.0;
        commands.entity(enemy_entity).despawn();

        spawn_enemy(
            &mut commands,
            &map,
            &mut meshes,
            &mut materials
        );
    }
}

fn spawn_enemy(
    mut commands: &mut Commands,
    map: &Res<Map>,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let initial_hex_field = Hex { x: 0, y: -13 };
    let world_pos = map.layout.hex_to_world_pos(initial_hex_field);
    let mut full_path: Vec<Hex> = vec![];

    let pos_1 = Hex { x: 5, y: -7 };
    let pos_2 = Hex { x: 0, y: 0 };
    let pos_3 = Hex { x: -9, y: 13 };

    let path = a_star(initial_hex_field, pos_1, |h| Some(1));
    if let Some(hex_fields) = path {
        hex_fields.iter().for_each(|pos| {
            commands.entity(*map.entities.get(pos).unwrap()).insert(map.highlighted_material.clone());
            full_path.push(*pos);
        })
    }

    let path = a_star(pos_1, pos_2, |h| Some(1));
    if let Some(hex_fields) = path {
        hex_fields.iter().for_each(|pos| {
            commands.entity(*map.entities.get(pos).unwrap()).insert(map.highlighted_material.clone());
            full_path.push(*pos);
        })
    }

    let path = a_star(pos_2, pos_3, |h| Some(1));
    if let Some(hex_fields) = path {
        hex_fields.iter().for_each(|pos| {
            commands.entity(*map.entities.get(pos).unwrap()).insert(map.highlighted_material.clone());
            full_path.push(*pos);
        })
    }

    let first_field = *full_path.get(1).unwrap();

    commands.spawn((
        Name::from("Enemy"),
        EnemyTag,
        HexLocation { location: initial_hex_field },
        WalkingPath {
            path: full_path,
            next_location: first_field,
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.1,
                depth: 0.4,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(world_pos.x, 0.1, world_pos.y),
            ..default()
        },
        Collider::ball(0.5),
        RigidBody::Dynamic,
        GravityScale(0.0),
        ActiveEvents::COLLISION_EVENTS
    ));
}

fn collision_event_handler(mut event_reader: EventReader<CollisionEvent>) {
    event_reader.iter().for_each(|e| {
        if CollisionEvent::Started(e1, e2, _) = *e {
            //
        }
    })
}