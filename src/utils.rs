use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct CheckboxIconMarker;

pub(crate) fn spawn_checkbox<B: Bundle + Clone>(
    children: &mut ChildBuilder,
    bundle: B,
    name: &str,
    checked: bool,
    text: String,
) {
    children
        .spawn((
            NodeBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(5.)),
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            Name::new(name.to_string()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        border_color: Color::WHITE.into(),
                        border_radius: BorderRadius::all(Val::Px(5.)),
                        style: Style {
                            border: UiRect::all(Val::Px(1.)),
                            width: Val::Px(20.),
                            height: Val::Px(20.),
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("check_box_button"),
                    bundle,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                width: Val::Px(10.),
                                height: Val::Px(10.),
                                display: if checked {
                                    Display::Flex
                                } else {
                                    Display::None
                                },
                                ..default()
                            },
                            border_radius: BorderRadius::all(Val::Px(3.)),
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        CheckboxIconMarker,
                        Name::new("check_box_icon"),
                    ));
                });
        });

    children
        .spawn((
            NodeBundle {
                style: Style {
                    align_content: AlignContent::Stretch,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("check_box_label"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font_size: 10.,
                        ..default()
                    },
                ),
                Label,
            ));
        });
}
