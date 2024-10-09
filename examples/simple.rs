use bevy::{
    color::palettes::basic::PURPLE, log::LogPlugin, prelude::*, sprite::MaterialMesh2dBundle,
    window::WindowResolution,
};
use bevy_debug_log::LogViewerVisibility;

fn main() {
    App::new().add_plugins((
        DefaultPlugins.set(LogPlugin {
            filter: "info".into(),
            level: bevy::log::Level::INFO,
            custom_layer: bevy_debug_log::log_capture_layer,
        }),
        bevy_debug_log::plugin,
    ));

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
                filter: "info".into(),
                level: bevy::log::Level::INFO,
                custom_layer: bevy_debug_log::log_capture_layer,
            }),
    );
    app.add_plugins(bevy_debug_log::plugin);
    app.add_systems(Startup, setup);
    app.add_systems(Update, toggle_log);
    app.run();
}

fn toggle_log(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        commands.trigger(LogViewerVisibility::Toggle);

        info!("toggle log!");
        warn!("toggle log!");
        error!("toggle log!");
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: materials.add(Color::from(PURPLE)),
        ..default()
    });

    commands.spawn(
        TextBundle::from_section("Press space to toggle log", TextStyle::default()).with_style(
            Style {
                position_type: PositionType::Absolute,
                bottom: Val::Px(12.0),
                left: Val::Px(12.0),
                ..default()
            },
        ),
    );
}
