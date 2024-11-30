use bevy::{color::palettes::basic::PURPLE, log::LogPlugin, prelude::*, window::WindowResolution};
use bevy_debug_log::LogViewerVisibility;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bevy_debug_log example".into(),
                    resolution: WindowResolution::new(393.0, 852.0),
                    ..default()
                }),
                ..default()
            })
            .set(LogPlugin {
                level: bevy::log::Level::TRACE,
                filter: "INFO,simple=TRACE".into(),
                custom_layer: bevy_debug_log::log_capture_layer,
                ..default()
            }),
    );
    app.add_plugins(bevy_debug_log::LogViewerPlugin::default());
    app.add_systems(Startup, setup);
    app.add_systems(Update, toggle_log);
    app.run();
}

fn toggle_log(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(LogViewerVisibility::Toggle);

        info!("toggle log!");
    } else if keyboard.just_pressed(KeyCode::Digit1) {
        trace!("trace log");
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        debug!("debug log");
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        info!("info log");
    } else if keyboard.just_pressed(KeyCode::Digit4) {
        warn!("warn log");
    } else if keyboard.just_pressed(KeyCode::Digit5) {
        error!("error log");
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    commands.spawn((
        Transform::default().with_scale(Vec3::splat(128.)),
        Mesh2d(meshes.add(Rectangle::default()).into()),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
    ));

    commands.spawn((
        Text::new("Press space to toggle log window.\nPress 1-5 for logs."),
        TextFont::from_font_size(18.),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}
