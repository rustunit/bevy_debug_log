use crate::{
    debug_log_level::DebugLogLevel,
    log_viewer::{
        setup_log_viewer_ui, AutoCheckBox, ChipToggle, LevelFilterChip, ListMarker,
        LogViewerMarker, LogViewerState, TrafficLightButton,
    },
    utils::{CheckboxIconMarker, ChipLeadingTextMarker},
};
use bevy::{
    color::palettes::css,
    log::{
        tracing_subscriber::{self, Layer},
        BoxedLayer,
    },
    prelude::*,
    utils::tracing::{self, level_filters::LevelFilter, Subscriber},
};
use std::{num::NonZero, sync::mpsc};
use time::{format_description::well_known::iso8601, OffsetDateTime};

#[derive(Debug, Event, Clone)]
struct LogEvent {
    message: String,
    metadata: &'static tracing::Metadata<'static>,
    timestamp: OffsetDateTime,
}

#[derive(Deref, DerefMut)]
struct LogEventsReceiver(mpsc::Receiver<LogEvent>);

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
                    timestamp: OffsetDateTime::now_utc(),
                })
                .ok();
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

pub struct LogViewerPlugin {
    auto_open_threshold: LevelFilter,
}

impl Default for LogViewerPlugin {
    fn default() -> Self {
        Self {
            auto_open_threshold: LevelFilter::ERROR,
        }
    }
}

impl LogViewerPlugin {
    pub fn auto_open_threshold(mut self, level_filter: LevelFilter) -> Self {
        self.auto_open_threshold = level_filter;
        self
    }
}

impl Plugin for LogViewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LogEvent>();

        app.insert_resource(LogViewerState {
            auto_open_threshold: self.auto_open_threshold,
            auto_open_enabled: self.auto_open_threshold != LevelFilter::OFF,
            ..default()
        });
        app.add_observer(handle_log_viewer_visibilty);
        app.add_observer(handle_log_viewer_fullscreen);
        app.add_observer(handle_log_viewer_clear);
        app.add_observer(handle_auto_open_check);
        app.add_observer(handle_level_filter_chip_toggle);

        app.add_systems(Startup, setup_log_viewer_ui);

        // Running update_log_ui in PreUpdate to prevent data races between updating the UI and filtering log lines.
        // `handle_level_filter_chip_toggle`` can modify the `{level}_visible` fields in `LogViewerState`
        // while `update_log_ui` is adding new loglines to the viewer in parallel based on older values.
        app.add_systems(PreUpdate, (receive_logs, update_log_counts));

        app.add_systems(
            Update,
            (
                on_traffic_light_button,
                on_auto_open_check,
                on_level_filter_chip,
            ),
        );
    }
}

pub fn log_capture_layer(app: &mut App) -> Option<BoxedLayer> {
    let (sender, receiver) = mpsc::channel();

    let layer = CaptureLayer { sender };
    let log_receiver = LogEventsReceiver(receiver);

    app.insert_non_send_resource(log_receiver);

    Some(layer.boxed())
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

#[derive(Event, Reflect, Debug, Clone, Copy)]
pub struct AutoOpenToggle;

#[derive(Component)]
struct LogLineMarker;

#[derive(Component)]
struct ErrLogLineMarker;

type WithOnlyErrLogLine = (
    With<ErrLogLineMarker>,
    Without<WarnLogLineMarker>,
    Without<InfoLogLineMarker>,
    Without<DebugLogLineMarker>,
    Without<TraceLogLineMarker>,
);

#[derive(Component)]
struct WarnLogLineMarker;

type WithOnlyWarnLogLine = (
    Without<ErrLogLineMarker>,
    With<WarnLogLineMarker>,
    Without<InfoLogLineMarker>,
    Without<DebugLogLineMarker>,
    Without<TraceLogLineMarker>,
);

#[derive(Component)]
struct InfoLogLineMarker;

type WithOnlyInfoLogLine = (
    Without<ErrLogLineMarker>,
    Without<WarnLogLineMarker>,
    With<InfoLogLineMarker>,
    Without<DebugLogLineMarker>,
    Without<TraceLogLineMarker>,
);

#[derive(Component)]
struct DebugLogLineMarker;

type WithOnlyDebugLogLine = (
    Without<ErrLogLineMarker>,
    Without<WarnLogLineMarker>,
    Without<InfoLogLineMarker>,
    With<DebugLogLineMarker>,
    Without<TraceLogLineMarker>,
);

#[derive(Component)]
struct TraceLogLineMarker;

type WithOnlyTraceLogLine = (
    Without<ErrLogLineMarker>,
    Without<WarnLogLineMarker>,
    Without<InfoLogLineMarker>,
    Without<DebugLogLineMarker>,
    With<TraceLogLineMarker>,
);

fn handle_log_viewer_visibilty(
    trigger: Trigger<LogViewerVisibility>,
    mut log_viewer_query: Query<&mut Node, With<LogViewerMarker>>,
    mut log_viewer_res: ResMut<LogViewerState>,
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
    logs: Query<Entity, With<LogLineMarker>>,
    mut commands: Commands,
) {
    for e in logs.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn handle_log_viewer_fullscreen(
    trigger: Trigger<LogViewerSize>,
    mut log_viewer_query: Query<&mut Node, With<LogViewerMarker>>,
    mut log_viewer_res: ResMut<LogViewerState>,
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

fn handle_auto_open_check(
    _trigger: Trigger<AutoOpenToggle>,
    mut log_viewer_res: ResMut<LogViewerState>,
    mut checkbox_query: Query<&mut Node, With<CheckboxIconMarker>>,
) {
    log_viewer_res.auto_open_enabled = !log_viewer_res.auto_open_enabled;

    for mut style in checkbox_query.iter_mut() {
        style.display = if log_viewer_res.auto_open_enabled {
            Display::Flex
        } else {
            Display::None
        };
    }
}

#[allow(clippy::too_many_arguments)]
fn handle_level_filter_chip_toggle(
    trigger: Trigger<ChipToggle>,
    mut chip_query: Query<
        (&mut BackgroundColor, &mut BorderColor, &LevelFilterChip),
        With<LevelFilterChip>,
    >,
    mut log_viewer_res: ResMut<LogViewerState>,
    mut err_logline_query: Query<&mut Node, WithOnlyErrLogLine>,
    mut warn_logline_query: Query<&mut Node, WithOnlyWarnLogLine>,
    mut info_logline_query: Query<&mut Node, WithOnlyInfoLogLine>,
    mut debug_logline_query: Query<&mut Node, WithOnlyDebugLogLine>,
    mut trace_logline_query: Query<&mut Node, WithOnlyTraceLogLine>,
) {
    for (mut bg_color, mut border_color, chip) in chip_query.iter_mut() {
        if trigger.event().0 == *chip {
            let BackgroundColor(color) = *bg_color;

            // If the color has an alpha value > 0, it means it's already selected.
            if color.alpha() != 0. {
                // Deselect the chip, make the background transparent and the border white.
                *bg_color = color.with_alpha(0.).into();
                *border_color = css::WHITE.into();
                // Hide the log lines of the deselected chip.
                match *chip {
                    LevelFilterChip::Error => {
                        for mut style in err_logline_query.iter_mut() {
                            log_viewer_res.error_visible = false;
                            style.display = Display::None;
                        }
                    }
                    LevelFilterChip::Warn => {
                        for mut style in warn_logline_query.iter_mut() {
                            log_viewer_res.warn_visible = false;
                            style.display = Display::None;
                        }
                    }
                    LevelFilterChip::Info => {
                        for mut style in info_logline_query.iter_mut() {
                            log_viewer_res.info_visible = false;
                            style.display = Display::None;
                        }
                    }
                    LevelFilterChip::Debug => {
                        for mut style in debug_logline_query.iter_mut() {
                            log_viewer_res.debug_visible = false;
                            style.display = Display::None;
                        }
                    }
                    LevelFilterChip::Trace => {
                        for mut style in trace_logline_query.iter_mut() {
                            log_viewer_res.trace_visible = false;
                            style.display = Display::None;
                        }
                    }
                }
            } else {
                // Select the chip, make the background translucent and the border solid.
                *bg_color = color.with_alpha(0.25).into();
                *border_color = color.with_alpha(1.).into();
                // Show the log lines that match the chip's level.
                match *chip {
                    LevelFilterChip::Error => {
                        for mut style in err_logline_query.iter_mut() {
                            log_viewer_res.error_visible = true;
                            style.display = Display::Flex;
                        }
                    }
                    LevelFilterChip::Warn => {
                        for mut style in warn_logline_query.iter_mut() {
                            log_viewer_res.warn_visible = true;
                            style.display = Display::Flex;
                        }
                    }
                    LevelFilterChip::Info => {
                        for mut style in info_logline_query.iter_mut() {
                            log_viewer_res.info_visible = true;
                            style.display = Display::Flex;
                        }
                    }
                    LevelFilterChip::Debug => {
                        for mut style in debug_logline_query.iter_mut() {
                            log_viewer_res.debug_visible = true;
                            style.display = Display::Flex;
                        }
                    }
                    LevelFilterChip::Trace => {
                        for mut style in trace_logline_query.iter_mut() {
                            log_viewer_res.trace_visible = true;
                            style.display = Display::Flex;
                        }
                    }
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn update_log_counts(
    chip_query: Query<(Entity, &LevelFilterChip), (With<ChipLeadingTextMarker>, With<Text>)>,
    err_logline_query: Query<&mut Node, WithOnlyErrLogLine>,
    warn_logline_query: Query<&mut Node, WithOnlyWarnLogLine>,
    info_logline_query: Query<&mut Node, WithOnlyInfoLogLine>,
    debug_logline_query: Query<&mut Node, WithOnlyDebugLogLine>,
    trace_logline_query: Query<&mut Node, WithOnlyTraceLogLine>,
    mut text_writer: TextUiWriter,
) {
    // Update the count of log lines for each chip.
    for (e, chip) in chip_query.iter() {
        let count = match *chip {
            LevelFilterChip::Error => err_logline_query.iter().count(),
            LevelFilterChip::Warn => warn_logline_query.iter().count(),
            LevelFilterChip::Info => info_logline_query.iter().count(),
            LevelFilterChip::Debug => debug_logline_query.iter().count(),
            LevelFilterChip::Trace => trace_logline_query.iter().count(),
        };
        *text_writer.text(e, 0) = count.to_string();
    }
}

fn receive_logs(
    mut commands: Commands,
    mut query: Query<Entity, With<ListMarker>>,
    log_viewer_res: Res<LogViewerState>,
    logs_rx: Option<NonSend<LogEventsReceiver>>,
) {
    if let Some(receiver) = logs_rx {
        for e in receiver.try_iter() {
            if let Ok(parent) = query.get_single_mut() {
                let child = spawn_logline(&mut commands, parent, &e);

                // Insert the relevant log line marker and set visibility based on the log level.
                match *e.metadata.level() {
                    tracing::Level::ERROR => add_level_info(
                        &mut commands,
                        child,
                        ErrLogLineMarker,
                        log_viewer_res.error_visible,
                    ),
                    tracing::Level::WARN => add_level_info(
                        &mut commands,
                        child,
                        WarnLogLineMarker,
                        log_viewer_res.warn_visible,
                    ),
                    tracing::Level::INFO => add_level_info(
                        &mut commands,
                        child,
                        InfoLogLineMarker,
                        log_viewer_res.info_visible,
                    ),
                    tracing::Level::DEBUG => add_level_info(
                        &mut commands,
                        child,
                        DebugLogLineMarker,
                        log_viewer_res.debug_visible,
                    ),
                    tracing::Level::TRACE => add_level_info(
                        &mut commands,
                        child,
                        TraceLogLineMarker,
                        log_viewer_res.trace_visible,
                    ),
                };
            }
            // If the log viewer is not visible, check if the log event should trigger it to open.
            if log_viewer_res.auto_open_enabled
                && !log_viewer_res.visible
                && *e.metadata.level() <= log_viewer_res.auto_open_threshold
            {
                commands.trigger(LogViewerVisibility::Show);
            }
        }
    }
}

fn add_level_info(
    commands: &mut Commands,
    child_entity: Entity,
    level_marker: impl Component,
    visible: bool,
) {
    commands.entity(child_entity).insert((
        level_marker,
        Node {
            display: if visible {
                Display::Flex
            } else {
                Display::None
            },
            ..default()
        },
    ));
}

fn spawn_logline(commands: &mut Commands, parent: Entity, event: &LogEvent) -> Entity {
    let dbg_level = DebugLogLevel::from(*event.metadata.level());
    const LOG_LINE_FONT_SIZE: f32 = 8.;

    commands
        .spawn((
            TextLayout::default().with_linebreak(LineBreak::AnyCharacter),
            Text::default(),
            // Label,
            LogLineMarker,
        ))
        .with_child((
            TextSpan::new(
                event
                    .timestamp
                    .format(&iso8601::Iso8601::<{ iso8601_with_two_digit_secs() }> {})
                    .unwrap_or("timestamp error".to_string()),
            ),
            TextFont::from_font_size(LOG_LINE_FONT_SIZE),
            TextColor(css::WHITE.with_alpha(0.5).into()),
        ))
        .with_child((
            TextSpan::new(format!(" {} ", dbg_level)),
            TextFont::from_font_size(LOG_LINE_FONT_SIZE),
            TextColor(dbg_level.into()),
        ))
        .with_child((
            TextSpan::new(format!("{}: ", event.metadata.target())),
            TextFont::from_font_size(LOG_LINE_FONT_SIZE),
            TextColor(css::WHITE.with_alpha(0.5).into()),
        ))
        .with_child((
            TextSpan::new(event.message.clone()),
            TextFont::from_font_size(LOG_LINE_FONT_SIZE),
            TextColor(css::WHITE.into()),
        ))
        .set_parent(parent)
        .id()
}

fn on_traffic_light_button(
    mut interaction_query: Query<(&TrafficLightButton, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (button, interaction) in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            match button {
                TrafficLightButton::Red => commands.trigger(LogViewerVisibility::Hide),
                TrafficLightButton::Yellow => commands.trigger(ClearLogs),
                TrafficLightButton::Green => commands.trigger(LogViewerSize::Toggle),
            }
        }
    }
}

fn on_auto_open_check(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<AutoCheckBox>)>,
    mut commands: Commands,
) {
    for interaction in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            commands.trigger(AutoOpenToggle);
        }
    }
}

fn on_level_filter_chip(
    mut interaction_query: Query<(&LevelFilterChip, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (level, interaction) in &mut interaction_query {
        if matches!(*interaction, Interaction::Pressed) {
            commands.trigger(ChipToggle(*level));
        }
    }
}

/// ISO 8601 Config with two decimal digits for seconds.
const fn iso8601_with_two_digit_secs() -> u128 {
    // This is a hack to get around .unwrap() not being stable as a const fn
    const TWO: NonZero<u8> = match NonZero::new(2) {
        Some(two) => two,
        None => unreachable!(),
    };

    iso8601::Config::DEFAULT
        .set_time_precision(iso8601::TimePrecision::Second {
            decimal_digits: Some(TWO),
        })
        .encode()
}
