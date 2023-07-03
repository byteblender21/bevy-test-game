use std::time::Duration;
use bevy::prelude::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(building_shooting)
            .add_system(move_bullets)
        ;
    }
}

#[derive(Component)]
pub struct BuildingTag;

#[derive(Component)]
pub struct HasAttack {
    /// How often to spawn a new bullet? (repeating timer)
    pub(crate) timer: Timer,
}

#[derive(Component)]
pub struct Bullet {
    speed: f32,
    pub(crate) life_timer: Timer,
}

fn building_shooting(
    mut commands: Commands,
    mut q: Query<(&Transform, &mut HasAttack), With<BuildingTag>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    q.iter_mut().for_each(|(transform, mut attack)| {
        attack.timer.tick(time.delta());

        // if it finished, despawn the bomb
        if attack.timer.finished() {
            commands.spawn((
                Name::from("Bullet"),
                Bullet {
                    speed: 0.2,
                    life_timer: Timer::new(Duration::from_millis(300), TimerMode::Once),
                },
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::UVSphere {
                        radius: 0.05,
                        ..default()
                    })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(transform.translation.x, 0.3, transform.translation.z),
                    ..default()
                },
            ));
        }
    });
}

fn move_bullets(
    mut commands: Commands,
    mut q: Query<(&mut Bullet, &mut Transform, Entity)>,
    time: Res<Time>,
) {
    q.iter_mut().for_each(|(mut bullet, mut transform, e)| {
        transform.translation.x += bullet.speed;

        bullet.life_timer.tick(time.delta());

        if bullet.life_timer.finished() {
            commands.entity(e).despawn();
        }
    });
}