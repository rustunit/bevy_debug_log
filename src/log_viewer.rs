use bevy::{prelude::*, render::view::RenderLayers, utils::tracing::level_filters::LevelFilter};

use crate::{debug_log_level::DebugLogLevel, utils};

pub const RENDER_LAYER: usize = 55;

#[derive(Component)]
pub(crate) struct LogViewerMarker;

#[derive(Component, PartialEq)]
pub(crate) enum ScrollState {
    Auto,
    Manual,
}

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
    pub(crate) scroll_state: ScrollState,
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
            scroll_state: ScrollState::Auto,
        }
    }
}

#[derive(Component)]
pub(crate) struct ListMarker;

#[derive(Component)]
pub(crate) struct ListContainerMarker;

#[derive(Component)]
pub(crate) struct GoDownBtnMarker;

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
    let safe_zone_top = if cfg!(target_os = "ios") { 50 } else { 0 };

    commands
        .spawn((
            Name::new("log-viewer-ui"),
            RenderLayers::layer(RENDER_LAYER),
            LogViewerMarker,
            GlobalZIndex(i32::MAX),
            Node {
                display: Display::None,
                width: Val::Percent(100.0),
                height: Val::Percent(40.0),
                padding: UiRect::all(Val::Px(4.)).with_top(Val::Px(safe_zone_top as f32)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Stretch,
                position_type: PositionType::Absolute,
                overflow: Overflow::clip(),
                border: UiRect::bottom(Val::Px(1.)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.75)),
            BorderColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Title Bar
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::SpaceBetween,
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
                        "error_switch",
                    );

                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Warn,
                        DebugLogLevel::WARN.into(),
                        "0".into(),
                        "W".into(),
                        log_viewer_res.warn_visible,
                        "warn_switch",
                    );

                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Info,
                        DebugLogLevel::INFO.into(),
                        "0".into(),
                        "I".into(),
                        log_viewer_res.info_visible,
                        "info_switch",
                    );

                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Debug,
                        DebugLogLevel::DEBUG.into(),
                        "0".into(),
                        "D".into(),
                        log_viewer_res.debug_visible,
                        "debug_switch",
                    );

                    utils::spawn_chip(
                        parent,
                        LevelFilterChip::Trace,
                        DebugLogLevel::TRACE.into(),
                        "0".into(),
                        "T".into(),
                        log_viewer_res.trace_visible,
                        "trace_switch",
                    );

                    parent.spawn((
                        Node {
                            align_items: AlignItems::End,
                            flex_grow: 1.0,
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
                                Node {
                                    align_items: AlignItems::End,
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
                            Node {
                                padding: UiRect::all(Val::Px(5.)),
                                align_items: AlignItems::End,
                                ..default()
                            },
                            Name::new("size_btn"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Button,
                                Node {
                                    width: Val::Px(20.),
                                    height: Val::Px(20.),
                                    ..default()
                                },
                                BorderRadius::all(Val::Px(20.)),
                                BackgroundColor(Color::srgb_u8(43, 198, 63)),
                                TrafficLightButton::Green,
                            ));
                        });
                    parent
                        .spawn((
                            Node {
                                padding: UiRect::all(Val::Px(5.)),
                                align_items: AlignItems::End,
                                ..default()
                            },
                            Name::new("clear_btn"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Button,
                                Node {
                                    width: Val::Px(20.),
                                    height: Val::Px(20.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb_u8(255, 188, 46)),
                                BorderRadius::all(Val::Px(20.)),
                                TrafficLightButton::Yellow,
                            ));
                        });
                    parent
                        .spawn((
                            Node {
                                padding: UiRect::all(Val::Px(5.)),
                                align_items: AlignItems::End,
                                ..default()
                            },
                            Name::new("close_logs_btn"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Button,
                                Node {
                                    width: Val::Px(20.),
                                    height: Val::Px(20.),
                                    ..default()
                                },
                                BorderRadius::all(Val::Px(20.)),
                                BackgroundColor(Color::srgb_u8(255, 95, 87)),
                                TrafficLightButton::Red,
                            ));
                        });
                });

            // Button for scrolling to the bottom
            parent
                .spawn((
                    Node {
                        width: Val::Px(24.),
                        height: Val::Px(24.),
                        bottom: Val::Px(5.),
                        right: Val::Px(5.),
                        position_type: PositionType::Absolute,
                        display: Display::None,
                        margin: UiRect::all(Val::Px(1.)),
                        padding: UiRect {
                            left: Val::Px(5.),
                            right: Val::Px(5.),
                            top: Val::Px(10.),
                            bottom: Val::Px(5.),
                        },
                        border: UiRect::all(Val::Px(1.)),
                        ..default()
                    },
                    ZIndex(1),
                    Button,
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(20.)),
                    BackgroundColor(Color::BLACK.with_alpha(0.75)),
                    GoDownBtnMarker,
                    Name::new("go_down_btn"),
                ))
                .with_children(|parent| {
                    // Create a down-arrow icon by rotating a square 45 degrees
                    // and clipping the overflow at the top.
                    parent
                        .spawn((
                            Node {
                                overflow: Overflow::clip_y(),
                                align_items: AlignItems::End,
                                justify_content: JustifyContent::Center,
                                width: Val::Px(16.),
                                height: Val::Px(8.),
                                padding: UiRect::bottom(Val::Px(4.)),
                                ..default()
                            },
                            Name::new("icon_container"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Node {
                                    width: Val::Px(8.),
                                    height: Val::Px(8.),
                                    ..default()
                                },
                                Transform {
                                    rotation: Quat::from_rotation_z(std::f32::consts::FRAC_PI_4),
                                    ..default()
                                },
                                BackgroundColor(Color::WHITE),
                                Name::new("down_arrow"),
                            ));
                        });
                });

            // List Container
            parent
                .spawn((
                    Node {
                        height: Val::Percent(100.),
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    Name::new("container"),
                    ListContainerMarker,
                ))
                .observe(on_drag_scroll)
                .with_children(|children| {
                    children.spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        PickingBehavior {
                            should_block_lower: false,
                            ..default()
                        },
                        Name::new("list"),
                        ListMarker,
                    ));
                });
        });
}

fn on_drag_scroll(
    drag: Trigger<Pointer<Drag>>,
    mut scroll_positions: Query<&mut ScrollPosition, With<ListContainerMarker>>,
    mut log_viewer_state: ResMut<LogViewerState>,
) {
    if let Ok(mut scroll_position) = scroll_positions.get_mut(drag.entity()) {
        scroll_position.offset_y -= drag.delta.y;
        log_viewer_state.scroll_state = ScrollState::Manual;
    }
}
