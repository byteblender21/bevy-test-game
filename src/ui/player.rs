use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::window::WindowResized;

pub struct PlayerUiPlugin;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_ui)
            .add_system(on_resize_system)
        ;
    }
}

#[derive(Component)]
struct ChangingUiPart;

fn setup_ui(mut commands: Commands,
            asset_server: Res<AssetServer>,
            query: Query<&Window>) {
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
                        });
                });
        });
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