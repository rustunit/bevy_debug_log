use bevy::{prelude::*, render::view::RenderLayers, utils::tracing::level_filters::LevelFilter};

use crate::{debug_log_level::DebugLogLevel, utils};

const RENDER_LAYER: usize = 55;

#[derive(Component)]
pub(crate) struct LogViewerMarker;

#[derive(Resource)]
pub(crate) struct LogViewerState {
    pub(crate) visible: bool,
    pub(crate) fullscreen: bool,
    pub(crate) auto_open_threshold: LevelFilter,
    pub(crate) auto_open_enabled: bool,
    pub(crate) error_visible: bool,
    pub(crate) warn_visible: bool,
    pub(crate) info_visible: bool,
    pub(crate) debug_visible: bool,
    pub(crate) trace_visible: bool,
}

impl Default for LogViewerState {
    fn default() -> Self {
        Self {
            auto_open_threshold: LevelFilter::OFF,
            visible: false,
            fullscreen: false,
            auto_open_enabled: false,
            error_visible: true,
            warn_visible: true,
            info_visible: true,
            debug_visible: true,
            trace_visible: true,
        }
    }
}

#[derive(Component)]
pub(crate) struct ListMarker;

#[derive(Component)]
pub(crate) enum TrafficLightButton {
    Red,
    Yellow,
    Green,
}

#[derive(Component, Clone)]
pub(crate) struct AutoCheckBox;

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub(crate) enum LevelFilterChip {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Event)]
pub(crate) struct ChipToggle(pub(crate) LevelFilterChip);

pub fn setup_log_viewer_ui(mut commands: Commands, log_viewer_res: Res<LogViewerState>) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(RENDER_LAYER),
        LogViewerMarker,
    ));

    let safe_zone_top = if cfg!(target_os = "ios") { 50 } else { 0 };

    commands
        .spawn((
            Name::new("log-viewer-ui"),
            RenderLayers::layer(RENDER_LAYER),
            LogViewerMarker,
            NodeBundle {
                z_index: ZIndex::Global(i32::MAX),
                style: Style {
                    display: Display::None,
                    width: Val::Percent(100.0),
                    height: Val::Percent(40.0),
                    padding: UiRect::all(Val::Px(4.)).with_top(Val::Px(safe_zone_top as f32)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Stretch,
                    position_type: PositionType::Absolute,
                    overflow: Overflow::clip(),
                    ..default()
                },
                background_color: Color::srgba(0.15, 0.15, 0.15, 0.75).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            // Title Bar
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("title_bar"),
                ))
                .with_children(|parent| {
                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Error,
                        DebugLogLevel::ERROR.into(),
                        "0".into(),
                        "E".into(),
                        log_viewer_res.error_visible,
                        "error_swtich",
                    );

                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Warn,
                        DebugLogLevel::WARN.into(),
                        "0".into(),
                        "W".into(),
                        log_viewer_res.warn_visible,
                        "warn_swtich",
                    );

                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Info,
                        DebugLogLevel::INFO.into(),
                        "0".into(),
                        "I".into(),
                        log_viewer_res.info_visible,
                        "info_swtich",
                    );

                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Debug,
                        DebugLogLevel::DEBUG.into(),
                        "0".into(),
                        "D".into(),
                        log_viewer_res.debug_visible,
                        "debug_swtich",
                    );

                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Trace,
                        DebugLogLevel::TRACE.into(),
                        "0".into(),
                        "T".into(),
                        log_viewer_res.trace_visible,
                        "trace_swtich",
                    );

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                align_items: AlignItems::End,
                                flex_grow: 1.0,
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("title_bar_spacer"),
                    ));
                    // Show checkbox only when auto-open is enabled
                    if log_viewer_res.auto_open_threshold != LevelFilter::OFF {
                        // This cannot fail because LevelFilter cannot be OFF here
                        let level: DebugLogLevel = log_viewer_res
                            .auto_open_threshold
                            .try_into()
                            .expect("LevelFilter should be convertible to DebugLogLevel");
                        parent
                            .spawn((
                                NodeBundle {
                                    style: Style {
                                        align_items: AlignItems::End,
                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new("auto-open"),
                            ))
                            .with_children(|parent| {
                                utils::spawn_checkbox(
                                    parent,
                                    AutoCheckBox,
                                    "auto-open-checkbox",
                                    log_viewer_res.auto_open_enabled,
                                    format!("Auto-open on {}", level.title_case()),
                                );
                            });
                    }
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(5.)),
                                    align_items: AlignItems::End,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("size_btn"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    background_color: Color::srgb_u8(43, 198, 63).into(),
                                    border_radius: BorderRadius::all(Val::Px(20.)),
                                    style: Style {
                                        width: Val::Px(20.),
                                        height: Val::Px(20.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TrafficLightButton::Green,
                            ));
                        });
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(5.)),
                                    align_items: AlignItems::End,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("clear_btn"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    background_color: Color::srgb_u8(255, 188, 46).into(),
                                    border_radius: BorderRadius::all(Val::Px(20.)),
                                    style: Style {
                                        width: Val::Px(20.),
                                        height: Val::Px(20.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TrafficLightButton::Yellow,
                            ));
                        });
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    padding: UiRect::all(Val::Px(5.)),
                                    align_items: AlignItems::End,
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("close_logs_btn"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    background_color: Color::srgb_u8(255, 95, 87).into(),
                                    border_radius: BorderRadius::all(Val::Px(20.)),
                                    style: Style {
                                        width: Val::Px(20.),
                                        height: Val::Px(20.),
                                        ..default()
                                    },
                                    ..default()
                                },
                                TrafficLightButton::Red,
                            ));
                        });
                });
            // List Container
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.),
                            overflow: Overflow {
                                x: OverflowAxis::Visible,
                                y: OverflowAxis::Clip,
                            },
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("container"),
                ))
                .with_children(|children| {
                    children.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::ColumnReverse,
                                position_type: PositionType::Absolute,
                                bottom: Val::Px(0.),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("list"),
                        ListMarker,
                    ));
                });
        });
}
