use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Action {
    Run,
    Jump,
}

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // This plugin maps inputs to an input-type agnostic action-state
        // We need to provide it with an enum which stores the possible actions a player could take
        .add_plugin(InputManagerPlugin::<Action>::default())
        // The InputMap and ActionState components will be added to any entity with the Player component
        .add_startup_system(spawn_player)
        // Read the ActionState in your systems using queries!
        .add_system(jump)
        // setup env
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/house.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3 {
            x: 0.25,
            y: 0.25,
            z: 0.25,
        }),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_player(mut commands: Commands,
                mut meshes: ResMut<Assets<Mesh>>,
                mut materials: ResMut<Assets<StandardMaterial>>,) {
    commands
        .spawn(InputManagerBundle::<Action> {
            // Stores "which actions are currently pressed"
            action_state: ActionState::default(),
            // Describes how to convert from player inputs into those actions
            input_map: InputMap::new([(KeyCode::Space, Action::Jump)]),
        })
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.1,
                depth: 0.4,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.3, 0.5),
            ..default()
        })
        .insert(Player);
}

// Query for the `ActionState` component in your game logic systems!
fn jump(mut query: Query<(&ActionState<Action>, &mut Transform), With<Player>>) {
    let (action_state, mut transform) = query.single_mut();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(Action::Jump) {
        println!("I'm jumping!");
        transform.translation.y += 0.3;
    }
}