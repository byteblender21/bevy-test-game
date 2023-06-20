use bevy::app::{App, Plugin};
use bevy::prelude::*;
use leafwing_input_manager::InputManagerBundle;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::*;
use crate::UiAction;

#[derive(Resource)]
struct GameMenu;

pub struct GameMenuPlugin;

fn resource_not_exists<T>() -> impl FnMut(Option<Res<T>>) -> bool + Clone
    where
        T: Resource,
{
    move |res: Option<Res<T>>| res.is_none()
}

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputManagerPlugin::<UiAction>::default())
            .add_startup_system(setup_menu_keyboard)
            .add_system(
                handle_actions
                    .run_if(resource_not_exists::<GameMenu>())
            )
            .add_system(
                render_game_menu
                    .run_if(resource_added::<GameMenu>())
            )
        ;
    }
}

fn setup_menu_keyboard(mut commands: Commands,) {
    commands.spawn(InputManagerBundle::<UiAction> {
        // Stores "which actions are currently pressed"
        action_state: ActionState::default(),
        // Describes how to convert from player inputs into those actions
        input_map: InputMap::new(
            [
                (KeyCode::Space, UiAction::OpenMenu),
            ]
        ),
    });
}

fn handle_actions(mut commands: Commands, query: Query<&ActionState<UiAction>>) {
    if query.single().pressed(UiAction::OpenMenu) {
        commands.insert_resource(GameMenu);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn render_game_menu(mut commands: Commands, asset_server: Res<AssetServer>, current_state: Res<GameMenu>) {
    println!("Activate menu");
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
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
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}