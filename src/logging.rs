use std::sync::mpsc;

use bevy::a11y::accesskit::{NodeBuilder, Role};
use bevy::a11y::AccessibilityNode;
use bevy::color::palettes::css;
use bevy::log::{BoxedLayer, Level};
use bevy::render::view::visibility::RenderLayers;
use bevy::{
    log::tracing_subscriber::{self, Layer},
    prelude::*,
    utils::tracing,
    utils::tracing::Subscriber,
};

const RENDER_LAYER: usize = 55;

#[derive(Debug, Event, Clone)]
struct LogEvent {
    message: String,
    metadata: &'static tracing::Metadata<'static>,
}

#[derive(Deref, DerefMut)]
struct LogEventsReceiver(mpsc::Receiver<LogEvent>);

fn transfer_log_events(
    receiver: Option<NonSend<LogEventsReceiver>>,
    mut log_events: EventWriter<LogEvent>,
) {
    if let Some(receiver) = receiver {
        // Pop all events from the channel and send them to the event writer.
        // Use `try_iter()` and not `iter()` to prevent blocking.
        log_events.send_batch(receiver.try_iter());
    }
}

struct CaptureLayer {
    sender: mpsc::Sender<LogEvent>,
}
impl<S: Subscriber> Layer<S> for CaptureLayer {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // In order to obtain the log message, we have to create a struct that implements
        // Visit and holds a reference to our string. Then we use the `record` method and
        // the struct to modify the reference to hold the message string.
        let mut message = None;
        event.record(&mut CaptureLayerVisitor(&mut message));
        if let Some(message) = message {
            self.sender
                .send(LogEvent {
                    message,
                    metadata: event.metadata(),
                })
                .expect("Sending log event should not fail");
        }
    }
}

/// A [`Visit`](tracing::field::Visit)or that records log messages that are transferred to [`CaptureLayer`].
struct CaptureLayerVisitor<'a>(&'a mut Option<String>);
impl tracing::field::Visit for CaptureLayerVisitor<'_> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        // This if statement filters out events without a message field.
        if field.name() == "message" {
            *self.0 = Some(format!("{value:?}"));
        }
    }
}

pub fn plugin(app: &mut App) {
    app.add_event::<LogEvent>();
    app.add_systems(Update, transfer_log_events);

    app.observe(handle_log_viewer_visibilty);
    app.observe(handle_log_viewer_fullscreen);
    app.observe(handle_log_viewer_clear);

    app.add_systems(Startup, setup_log_viewer_ui);
    app.add_systems(
        Update,
        (
            update_log_ui,
            on_close_button,
            on_fullscreen_button,
            on_clear_button,
        ),
    );
}

pub fn log_capture_layer(app: &mut App) -> Option<BoxedLayer> {
    let (sender, receiver) = mpsc::channel();

    let layer = CaptureLayer { sender };
    let log_receiver = LogEventsReceiver(receiver);

    app.insert_non_send_resource(log_receiver);

    Some(layer.boxed())
}

#[derive(Resource, Default)]
struct LogViewer {
    visible: bool,
    fullscreen: bool,
}

#[derive(Event, Reflect, Debug, Clone, Copy)]
pub enum LogViewerVisibility {
    Show,
    Hide,
    Toggle,
}

#[derive(Event, Reflect, Debug, Clone, Copy)]
pub enum LogViewerSize {
    Big,
    Small,
    Toggle,
}

#[derive(Event, Reflect, Debug, Clone, Copy)]
pub struct ClearLogs;

#[derive(Component)]
struct LogViewerMarker;

#[derive(Component)]
struct ListMarker;

#[derive(Component)]
struct LogLineMarker;

#[derive(Component)]
struct CloseButton;

#[derive(Component)]
struct SizeButton;

#[derive(Component)]
struct ClearButton;

fn setup_log_viewer_ui(mut commands: Commands) {
    commands.insert_resource(LogViewer::default());

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
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    align_content: AlignContent::Stretch,
                                    justify_self: JustifySelf::Center,
                                    padding: UiRect::all(Val::Px(5.)),
                                    ..default()
                                },
                                ..default()
                            },
                            Name::new("title_text"),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle::from_section(
                                    "Logs",
                                    TextStyle {
                                        font_size: 14.,
                                        ..default()
                                    },
                                ),
                                Label,
                            ));
                        });
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
                                SizeButton,
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
                            Name::new("size_btn"),
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
                                ClearButton,
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
                                CloseButton,
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

fn handle_log_viewer_visibilty(
    trigger: Trigger<LogViewerVisibility>,
    mut log_viewer_query: Query<&mut Style, With<LogViewerMarker>>,
    mut log_viewer_res: ResMut<LogViewer>,
) {
    let visible = match trigger.event() {
        LogViewerVisibility::Show => true,
        LogViewerVisibility::Hide => false,
        LogViewerVisibility::Toggle => !log_viewer_res.visible,
    };

    if visible {
        for mut style in log_viewer_query.iter_mut() {
            style.display = Display::Flex;
        }
        log_viewer_res.visible = true;
    } else {
        for mut style in log_viewer_query.iter_mut() {
            style.display = Display::None;
        }
        log_viewer_res.visible = false;
    }
}

fn handle_log_viewer_clear(
    _trigger: Trigger<ClearLogs>,
    mut log_viewer_query: Query<(Entity, &Parent), With<LogLineMarker>>,
    mut commands: Commands,
) {
    for (entity, parent) in log_viewer_query.iter_mut() {
        commands.entity(parent.get()).remove_children(&[entity]);
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_log_viewer_fullscreen(
    trigger: Trigger<LogViewerSize>,
    mut log_viewer_query: Query<&mut Style, With<LogViewerMarker>>,
    mut log_viewer_res: ResMut<LogViewer>,
) {
    match trigger.event() {
        LogViewerSize::Big => {
            for mut style in log_viewer_query.iter_mut() {
                style.height = Val::Percent(100.0);
            }
            log_viewer_res.fullscreen = true;
        }
        LogViewerSize::Small => {
            for mut style in log_viewer_query.iter_mut() {
                style.height = Val::Percent(40.0);
            }
            log_viewer_res.fullscreen = false;
        }
        LogViewerSize::Toggle => {
            for mut style in log_viewer_query.iter_mut() {
                match style.height {
                    Val::Percent(100.0) => {
                        style.height = Val::Percent(40.0);
                        log_viewer_res.fullscreen = false;
                    }
                    _ => {
                        style.height = Val::Percent(100.0);
                        log_viewer_res.fullscreen = true;
                    }
                }
            }
        }
    }
}

fn update_log_ui(
    mut events: EventReader<LogEvent>,
    mut commands: Commands,
    mut query: Query<Entity, With<ListMarker>>,
) {
    for e in events.read() {
        if let Ok(parent) = query.get_single_mut() {
            let child_entity = commands
                .spawn((
                    logline_text(e),
                    Label,
                    LogLineMarker,
                    AccessibilityNode(NodeBuilder::new(Role::ListItem)),
                ))
                .id();

            commands.entity(parent).insert_children(0, &[child_entity]);
        }
    }
}

fn logline_text(event: &LogEvent) -> TextBundle {
    // A log line is made up of three sections: level, target, and message laid out as
    // LEVEL target: Message
    // Example:
    // INFO bevy_window::system: No windows are open, exiting
    // lvl^ ^^     target     ^^ ^^       message          ^^

    let (level, color) = match *event.metadata.level() {
        Level::ERROR => ("ERROR", css::RED),
        Level::WARN => ("WARN", css::YELLOW),
        Level::INFO => ("INFO", css::LIME),
        Level::DEBUG => ("DEBUG", css::WHITE),
        Level::TRACE => ("TRACE", css::WHITE.with_alpha(0.5)),
    };

    const LOG_LINE_FONT_SIZE: f32 = 8.;

    TextBundle::from_sections([
        TextSection::new(
            format!("{} ", level),
            TextStyle {
                font_size: LOG_LINE_FONT_SIZE,
                color: color.into(),
                ..default()
            },
        ),
        TextSection::new(
            format!("{}: ", event.metadata.target()),
            TextStyle {
                font_size: LOG_LINE_FONT_SIZE,
                color: css::WHITE.with_alpha(0.5).into(),
                ..default()
            },
        ),
        TextSection::new(
            event.message.clone(),
            TextStyle {
                font_size: LOG_LINE_FONT_SIZE,
                color: css::WHITE.into(),
                ..default()
            },
        ),
    ])
}

fn on_close_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<CloseButton>)>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            commands.trigger(LogViewerVisibility::Hide);
        }
    }
}

fn on_clear_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ClearButton>)>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            commands.trigger(ClearLogs);
        }
    }
}

fn on_fullscreen_button(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SizeButton>)>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            commands.trigger(LogViewerSize::Toggle);
        }
    }
}
