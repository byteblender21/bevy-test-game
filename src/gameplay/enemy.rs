use bevy::app::App;
use bevy::core::Name;
use bevy::prelude::*;
use bevy::utils::default;
use hexx::algorithms::a_star;
use hexx::Hex;

use crate::{HexLocation, Map};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_enemy.in_base_set(StartupSet::PostStartup))
        ;
    }
}

#[derive(Component)]
pub struct EnemyTag;

#[derive(Component)]
pub struct WalkingPath {
    path: Vec<Hex>,
}

fn spawn_enemy(
    mut commands: Commands,
    map: Res<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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

    commands.spawn((
        Name::from("Enemy"),
        EnemyTag,
        HexLocation { location: initial_hex_field, },

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
    ));
}