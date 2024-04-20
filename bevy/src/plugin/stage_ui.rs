use bevy::app::{App, Plugin, PreStartup, Startup};
use bevy::hierarchy::BuildChildren;
use bevy::input::ButtonInput;
use bevy::prelude::{BackgroundColor, Bundle, ButtonBundle, Color, Commands, Component, Entity, Event, EventWriter, IntoSystemConfigs, MouseButton, NodeBundle, PositionType, Query, Reflect, ReflectComponent, Res, resource_exists_and_changed, Text, TextBundle, Update, Val, With};
use bevy::text::{TextSection, TextStyle};
use bevy::ui::{Display, FlexDirection, Interaction, Style, UiRect};
use bevy::utils::default;
use bevy_mod_picking::picking_core::Pickable;

use crate::consts::CELL_COLOR;
use crate::plugin::stage::{Answer, CorrectAnswerNum};

#[derive(Copy, Clone, Component, Reflect, Debug, Eq, PartialEq)]
#[reflect(Component)]
pub struct RootUi;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Eq, PartialEq)]
#[reflect(Component)]
pub struct RightPanel;

#[derive(Debug, Default, Copy, Clone, Event)]
pub struct RequestResetStage;

#[derive(Debug, Default, Copy, Clone, Event)]
pub struct RequestRegenerateStage;

#[derive(Debug, Default, Copy, Clone, Event)]
pub struct RequestPlayAnswerMode;

#[derive(Debug, Default, Copy, Clone, Event)]
pub struct RequestCellUndo;

#[derive(Debug, Default, Copy, Clone, Event)]
pub struct RequestCellRedo;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Eq, PartialEq)]
#[reflect(Component)]
struct AnswerText;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Eq, PartialEq)]
#[reflect(Component)]
struct CorrectAnswerNumText;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Eq, PartialEq)]
#[reflect(Component)]
pub struct StageClearText;

#[derive(Component)]
struct ResetStageButton;

#[derive(Component)]
struct RegenerateStageButton;

#[derive(Component)]
struct PlayAnswerButton;

#[derive(Component)]
struct UndoButton;

#[derive(Component)]
struct RedoButton;

#[derive(Component)]
struct InputButton;


pub struct StageUiPlugin;

impl Plugin for StageUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<RootUi>()
            .register_type::<AnswerText>()
            .register_type::<CorrectAnswerNumText>()
            .register_type::<StageClearText>()
            .register_type::<RightPanel>()
            .add_event::<RequestResetStage>()
            .add_event::<RequestRegenerateStage>()
            .add_event::<RequestPlayAnswerMode>()
            .add_event::<RequestCellUndo>()
            .add_event::<RequestCellRedo>()
            .add_systems(PreStartup, spawn_root_ui)
            .add_systems(Startup, (
                spawn_input_label,
                spawn_right_panel
            ).chain())
            .add_systems(Update, (
                update_answer_text.run_if(resource_exists_and_changed::<Answer>),
                update_correct_answer_num_text.run_if(resource_exists_and_changed::<CorrectAnswerNum>),
                update_button_colors,
                input_reset_stage.run_if(just_pressed_mouse_left),
                input_regenerate_stage.run_if(just_pressed_mouse_left),
                input_play_answer_mode.run_if(just_pressed_mouse_left),
                input_undo.run_if(just_pressed_mouse_left),
                input_redo.run_if(just_pressed_mouse_left)
            ));
    }
}

fn spawn_root_ui(
    mut commands: Commands
) {
    commands.spawn((
        RootUi,
        Pickable::IGNORE,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                ..default()
            },
            ..default()
        }
    ));
}

fn spawn_input_label(
    mut commands: Commands,
    root: Query<Entity, With<RootUi>>,
) {
    let text_style = {
        TextStyle {
            font_size: 32.,
            color: Color::BLACK,
            ..default()
        }
    };
    let input_label = commands.spawn((
        Pickable::IGNORE,
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.),
                ..default()
            },
            ..default()
        }
    ))
        .with_children(|parent| {
            parent
                .spawn((
                    ResetStageButton,
                    input_button_bundle()
                ))
                .with_children(|ui| {
                    ui.spawn(TextBundle {
                        text: Text::from_section("[R]: Reset stage", text_style.clone()),
                        ..default()
                    });
                });

            parent
                .spawn((
                    RegenerateStageButton,
                    input_button_bundle()
                ))
                .with_children(|ui| {
                    ui.spawn(TextBundle {
                        text: Text::from_section("[G]: Generate a new stage", text_style.clone()),
                        ..default()
                    });
                });

            parent
                .spawn((
                    PlayAnswerButton,
                    input_button_bundle()
                ))
                .with_children(|ui| {
                    ui.spawn(TextBundle {
                        text: Text::from_section("[P]: Play answer", text_style.clone()),
                        ..default()
                    });
                });

            parent.spawn((
                UndoButton,
                input_button_bundle()
            ))
                .with_children(|ui| {
                    ui.spawn(TextBundle {
                        text: Text::from_section("[Z]: Undo", text_style.clone()),
                        ..default()
                    });
                });

            parent.spawn((
                RedoButton,
                input_button_bundle()
            ))
                .with_children(|ui| {
                    ui.spawn(TextBundle {
                        text: Text::from_section("[X]: Redo", text_style),
                        ..default()
                    });
                });
        })
        .id();

    commands
        .entity(root.single())
        .add_child(input_label);
}


fn input_button_bundle() -> impl Bundle {
    (
        InputButton,
        ButtonBundle {
            background_color: BackgroundColor(CELL_COLOR),
            style: Style {
                padding: UiRect::all(Val::Px(8.)),
                ..default()
            },
            ..default()
        }
    )
}


fn spawn_right_panel(
    mut commands: Commands,
    root: Query<Entity, With<RootUi>>,
) {
    const LABEL_FONT_SIZE: f32 = 32.;
    let input_label = commands.spawn((
        Pickable::IGNORE,
        RightPanel,
        NodeBundle {
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                position_type: PositionType::Absolute,
                row_gap: Val::Px(16.),
                right: Val::Px(16.),
                top: Val::Px(16.),
                ..default()
            },
            ..default()
        }
    ))
        .with_children(|parent| {
            parent.spawn((
                AnswerText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("goal: ", TextStyle {
                            font_size: LABEL_FONT_SIZE,
                            ..default()
                        }),
                        TextSection::new("", TextStyle {
                            font_size: LABEL_FONT_SIZE,
                            color: Color::GOLD,
                            ..default()
                        }),
                    ]),
                    ..default()
                }
            ));

            parent.spawn((
                CorrectAnswerNumText,
                TextBundle {
                    text: Text::from_sections([
                        TextSection::new("consecutive answers: ", TextStyle {
                            font_size: LABEL_FONT_SIZE,
                            ..default()
                        }),
                        TextSection::new("0", TextStyle {
                            font_size: LABEL_FONT_SIZE,
                            color: Color::GOLD,
                            ..default()
                        })
                    ]),
                    ..default()
                }
            ));
        })
        .id();

    commands
        .entity(root.single())
        .add_child(input_label);
}

fn update_answer_text(
    answer: Res<Answer>,
    mut text: Query<&mut Text, With<AnswerText>>,
) {
    for mut text in text.iter_mut() {
        text.sections[1].value = format!("{}", answer.0);
    }
}

fn update_correct_answer_num_text(
    mut text: Query<&mut Text, With<CorrectAnswerNumText>>,
    num: Res<CorrectAnswerNum>,
) {
    for mut text in text.iter_mut() {
        text.sections[1].value = format!("{}", num.0);
    }
}

fn update_button_colors(
    mut input_buttons: Query<(&mut BackgroundColor, &Interaction), With<InputButton>>
) {
    for (mut bg, interaction) in input_buttons.iter_mut() {
        bg.0 = match interaction {
            Interaction::Hovered => {
                CELL_COLOR.with_a(0.8)
            }
            Interaction::None => {
                CELL_COLOR
            }
            Interaction::Pressed => {
                Color::YELLOW
            }
        };
    }
}

fn just_pressed_mouse_left(
    mouse: Res<ButtonInput<MouseButton>>
) -> bool {
    mouse.just_pressed(MouseButton::Left)
}

fn input_reset_stage(
    mut redo: EventWriter<RequestResetStage>,
    redo_button: Query<&Interaction, With<ResetStageButton>>,
) {
    if redo_button.get_single().is_ok_and(|b| matches!(b, Interaction::Pressed)) {
        redo.send(RequestResetStage);
    }
}

fn input_regenerate_stage(
    mut redo: EventWriter<RequestRegenerateStage>,
    button: Query<&Interaction, With<RegenerateStageButton>>,
) {
    if button.get_single().is_ok_and(|b| matches!(b, Interaction::Pressed)) {
        redo.send(RequestRegenerateStage);
    }
}

fn input_play_answer_mode(
    mut redo: EventWriter<RequestPlayAnswerMode>,
    button: Query<&Interaction, With<PlayAnswerButton>>,
) {
    if button.get_single().is_ok_and(|b| matches!(b, Interaction::Pressed)) {
        redo.send(RequestPlayAnswerMode);
    }
}

fn input_undo(
    mut undo: EventWriter<RequestCellUndo>,
    undo_button: Query<&Interaction, With<UndoButton>>,
) {
    if undo_button.get_single().is_ok_and(|b| matches!(b, Interaction::Pressed)) {
        undo.send(RequestCellUndo);
    }
}

fn input_redo(
    mut redo: EventWriter<RequestCellRedo>,
    redo_button: Query<&Interaction, With<RedoButton>>,
) {
    if redo_button.get_single().is_ok_and(|b| matches!(b, Interaction::Pressed)) {
        redo.send(RequestCellRedo);
    }
}