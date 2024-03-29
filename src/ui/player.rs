use std::slice::Windows;
use std::time::Duration;

use bevy::app::{App, Plugin};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::utils::petgraph::visit::Walker;
use bevy::window::WindowResized;
use bevy_mod_picking::debug::PointerDebug;
use bevy_mod_picking::focus::HoverMap;
use bevy_mod_picking::PickableBundle;
use bevy_mod_picking::prelude::{Bubble, Click, ListenedEvent, OnPointer, PointerLocation, RaycastPickTarget};
use hexx::Hex;

use crate::{HexFieldClicked, HexLocation, Map};
use crate::gameplay::buildings::{BuildingTag, HasAttack};

pub struct PlayerUiPlugin;

struct ButtonClickEvent;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ButtonClickEvent>()
            .add_startup_system(setup_ui)
            .add_system(on_resize_system)
            .add_system(on_building_button_clicked)
            .add_system(
                show_building_to_place
                    .run_if(resource_exists::<BuildingPlacement>())
            )
            .add_system(
                on_hex_field_click
                    .run_if(resource_exists::<BuildingPlacement>())
            )
        ;
    }
}

#[derive(Component)]
struct ChangingUiPart;

#[derive(Resource)]
struct BuildingPlacement {
    building: Entity,
}

const BUILDING_SCALING: Vec3 = Vec3::splat(0.1);

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Window>,
) {
    let window = query.single();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position: UiRect {
                        top: Val::Px(window.height() - 150.0),
                        ..default()
                    },
                    position_type: PositionType::Relative,
                    size: Size::width(Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            ChangingUiPart
        ))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(150.0)),
                        ..default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::width(Val::Percent(100.0)),
                                ..default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                TextBundle::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 17.0,
                                        color: Color::WHITE,
                                    },
                                )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    }),
                                // Because this is a distinct label widget and
                                // not button/list item text, this is necessary
                                // for accessibility to treat the text accordingly.
                                Label,
                            ));

                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: UiImage {
                                        texture: asset_server.load("images/button-01.png"),
                                        ..default()
                                    },
                                    ..default()
                                });
                        });
                });
        });
}

fn on_hex_field_click(
    mut commands: Commands,
    map: Res<Map>,
    mut field_click_reader: EventReader<HexFieldClicked>,
    mut placement: ResMut<BuildingPlacement>,
) {
    if field_click_reader.is_empty() {
        return;
    }

    let event = field_click_reader.iter().next().unwrap();

    let world_pos = map.layout.hex_to_world_pos(event.0);
    let obj_entity = placement.building;

    commands.entity(obj_entity)
        .insert((
            BuildingTag,
            HasAttack {
                timer: Timer::new(Duration::from_millis(800), TimerMode::Repeating),
            },
            Transform::from_xyz(world_pos.x, 0.0, world_pos.y).with_scale(BUILDING_SCALING),
        ));

    // clear all fields again
    map.entities
        .iter()
        .for_each(|(hex, e)| {
            commands.entity(*e).insert(map.default_material.clone());
        });

    commands.remove_resource::<BuildingPlacement>();
}

fn show_building_to_place(
    mut commands: Commands,
    hover_map: Res<HoverMap>,
    map: Res<Map>,
    placement: Res<BuildingPlacement>,
) {
    if let Some((_, hit_data)) = hover_map.0.iter().next() {
        if let Some((entity, hit_value)) = hit_data.iter().next() {
            let entries = map.entities
                .iter()
                .map(|(hex, e)| {
                    commands.entity(*e).insert(map.default_material.clone());
                    return (hex, e);
                })
                .filter(|(hex, e)| *e == entity)
                .collect::<Vec<(&Hex, &Entity)>>();

            if let Some((hex_field, field_entity)) = entries.first() {
                let pos = hit_value.position.unwrap();
                commands.entity(placement.building).insert(
                    Transform::from_xyz(pos.x, 0.0, pos.z).with_scale(BUILDING_SCALING)
                );

                hex_field.ring(1)
                    .for_each(|h| {
                        if let Some(e) = map.entities.get(&h) {
                            commands.entity(*e).insert(map.selection_material.clone());
                        }
                    });
                commands.entity(**field_entity).insert(map.selection_material.clone());
            }
        }
    }
}

fn on_building_button_clicked(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    asset_server: Res<AssetServer>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                let entity = commands
                    .spawn((
                        SceneBundle {
                            scene: asset_server.load("models/tower-001.glb#Scene0"),
                            transform: Transform::from_scale(Vec3::splat(0.0)),
                            ..default()
                        },
                    )).id();

                commands.insert_resource(BuildingPlacement {
                    building: entity
                });
            }
            _ => {}
        }
    }
}

fn on_resize_system(
    mut q: Query<&mut Style, With<ChangingUiPart>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut text = q.single_mut();
    for e in resize_reader.iter() {
        // When resolution is being changed
        text.position = UiRect {
            top: Val::Px(e.height - 150.0),
            ..default()
        };
    }
}
