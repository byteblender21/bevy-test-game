mod ui;
mod state;

use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::time::common_conditions::on_timer;
use hexx::shapes;
use hexx::*;
use std::collections::HashMap;
use std::time::Duration;
use bevy::a11y::AccessKitEntityExt;
use bevy::ecs::system::EntityCommands;
use bevy_mod_picking::event_listening::{Bubble, ListenedEvent, OnPointer};
use bevy_mod_picking::events::Click;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use bevy_mod_picking::debug::DebugPickingPlugin;
use bevy_mod_picking::highlight::DefaultHighlightingPlugin;
use bevy_mod_picking::prelude::{RaycastPickCamera, RaycastPickTarget};
use leafwing_input_manager::buttonlike::MouseMotionDirection;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use rand::Rng;
use crate::ui::menu::GameMenuPlugin;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(1.0);
/// World space height of hex columns
const COLUMN_HEIGHT: f32 = 1.0;
/// Map radius
const MAP_RADIUS: u32 = 20;
/// Animation time step
const TIME_STEP: Duration = Duration::from_millis(100);

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Jump,
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBack,
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum UiAction {
    OpenMenu,
}

#[derive(Component)]
struct PlayerCamera;

fn main() {
    App::new()
        .add_plugin(GameMenuPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>()
                .disable::<DebugPickingPlugin>(),
        )
        // This plugin maps inputs to an input-type agnostic action-state
        // We need to provide it with an enum which stores the possible actions a player could take
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_system(move_camera)
        // setup env
        .add_startup_system(setup)
        .add_startup_system(setup_grid)
        .run();
}

fn hexagonal_column(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, COLUMN_HEIGHT)
        .without_bottom_face()
        .build();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs);
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
}

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

#[derive(Debug, Default, Resource)]
struct HighlightedHexes {
    ring: u32,
    hexes: Vec<Hex>,
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        hex_size: Vec2::new(0.3, 0.3),
        orientation: HexOrientation::flat(),
        ..default()
    };

    // materials
    let default_material = materials.add(Color::WHITE.into());
    let highlighted_material = materials.add(Color::YELLOW.into());
    // mesh
    let mesh = hexagonal_column(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::hexagon(Hex::ZERO, 100)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn((
                    PbrBundle {
                        transform: Transform::from_xyz(pos.x, -0.2, pos.y)
                            .with_scale(Vec3::new(1.0, 0.1, 1.0)),
                        mesh: mesh_handle.clone(),
                        material: default_material.clone(),
                        ..default()
                    },
                    PickableBundle::default(),
                    RaycastPickTarget::default(),
                    OnPointer::<Click>::run_callback(on_hex_clicked),
                ))
                .id();
            (hex, id)
        })
        .collect();

    let map_resource = Map {
        layout,
        entities,
        highlighted_material,
        default_material,
    };

    spawn_stuff(&map_resource, &mut meshes, &mut materials, &mut commands);

    commands.insert_resource(map_resource);
}

fn spawn_stuff(map: &Map,
               meshes: &mut ResMut<Assets<Mesh>>,
               materials: &mut ResMut<Assets<StandardMaterial>>,
               commands: &mut Commands,
) {
    let mut rng = rand::thread_rng();

    let keys = map.entities.keys().cloned().collect::<Vec<Hex>>();

    for _ in 1..10 {
        let key = keys.get(rng.gen_range(0..keys.len() + 1)).unwrap();
        let entity = map.entities.get(key).unwrap();
        let pos = map.layout.hex_to_world_pos(*key);

        commands.entity(*entity).insert(map.highlighted_material.clone());
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Capsule {
                        radius: 0.1,
                        depth: 0.4,
                        ..default()
                    })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(pos.x, 0.1, pos.y),
                    ..default()
                },
            ));
    }
}

fn on_hex_clicked(
    In(event): In<ListenedEvent<Click>>,
    mut commands: Commands,
    map: Res<Map>,
) -> Bubble {
    commands.entity(event.target).insert(map.highlighted_material.clone());
    return Bubble::Burst;
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load("models/house.gltf#Scene0"),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 {
    //         x: 0.25,
    //         y: 0.25,
    //         z: 0.25,
    //     }),
    //     ..default()
    // });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 4500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera

    let mut input_map = InputMap::new(
        [
            (KeyCode::Space, Action::Jump),
        ]
    );

    input_map
        .insert_many_to_one(
            [
                InputKind::MouseMotion(MouseMotionDirection::Left),
                InputKind::Keyboard(KeyCode::Left),
            ],
            Action::MoveLeft,
        );

    input_map
        .insert_many_to_one(
            [
                InputKind::MouseMotion(MouseMotionDirection::Right),
                InputKind::Keyboard(KeyCode::Right),
            ],
            Action::MoveRight,
        );

    input_map
        .insert_many_to_one(
            [
                InputKind::MouseMotion(MouseMotionDirection::Down),
                InputKind::Keyboard(KeyCode::Up),
            ],
            Action::MoveForward,
        );

    input_map
        .insert_many_to_one(
            [
                InputKind::MouseMotion(MouseMotionDirection::Up),
                InputKind::Keyboard(KeyCode::Down),
            ],
            Action::MoveBack,
        );

    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-2.0, 4.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            },
            RaycastPickCamera::default(),
            PlayerCamera,
        ))
        .insert(InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map,
        });
}

const ACCELERATION: f32 = 0.2;

// Query for the `ActionState` component in your game logic systems!
fn move_camera(mut query: Query<(&ActionState<Action>, &mut Transform), With<PlayerCamera>>) {
    let (action_state, mut transform) = query.single_mut();
    if action_state.pressed(Action::MoveLeft) {
        transform.translation.x -= ACCELERATION;
    }
    else if action_state.pressed(Action::MoveRight) {
        transform.translation.x += ACCELERATION;
    }

    if action_state.pressed(Action::MoveForward) {
        transform.translation.z -= ACCELERATION;
    }
    else if action_state.pressed(Action::MoveBack) {
        transform.translation.z += ACCELERATION;
    }
}