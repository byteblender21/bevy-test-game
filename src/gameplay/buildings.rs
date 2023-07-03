use bevy::prelude::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(building_shooting)
        ;
    }
}

#[derive(Component)]
pub struct BuildingTag;

fn building_shooting(
    q: Query<&Transform, With<BuildingTag>>,
) {
    q.iter().for_each(|transform| {
        println!("Front: {}", transform.rotation);
    });
}