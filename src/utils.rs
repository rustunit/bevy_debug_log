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

#[derive(Component)]
struct ChipMarker;

#[derive(Component)]
pub(crate) struct ChipLeadingTextMarker;

pub(crate) fn spawn_chip<B: Bundle + Clone>(
    children: &mut ChildBuilder,
    bundle: B,
    color: Color,
    leading_text: String,
    label_text: String,
    active: bool,
    name: &str,
) {
    children
        .spawn((
            ButtonBundle {
                border_color: if active {
                    color.into()
                } else {
                    Color::WHITE.into()
                },
                background_color: if active {
                    color.with_alpha(0.25).into()
                } else {
                    {
                        color.with_alpha(0.).into()
                    }
                },
                border_radius: BorderRadius::all(Val::Px(20.)),
                style: Style {
                    border: UiRect::all(Val::Px(1.)),
                    justify_content: JustifyContent::Center,
                    align_self: AlignSelf::Center,
                    margin: UiRect::all(Val::Px(1.)),
                    ..default()
                },
                ..default()
            },
            ChipMarker,
            bundle.clone(),
            Name::new(name.to_string()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    // Circle with number of messages
                    NodeBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            margin: UiRect::all(Val::Px(5.)),
                            display: Display::Flex,
                            min_height: Val::Px(10.),
                            min_width: Val::Px(14.),
                            ..default()
                        },
                        border_radius: BorderRadius::all(Val::Px(10.)),
                        background_color: color.into(),
                        ..default()
                    },
                    Name::new("chip_leading"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            leading_text,
                            TextStyle {
                                font_size: 10.,
                                color: Color::BLACK,
                                ..default()
                            },
                        )
                        .with_text_justify(JustifyText::Center)
                        .with_style(Style {
                            align_self: AlignSelf::Center,
                            flex_grow: 1.,
                            margin: UiRect::all(Val::Px(2.)),
                            ..default()
                        }),
                        Label,
                        bundle,
                        ChipLeadingTextMarker,
                    ));
                });
            parent.spawn((
                TextBundle::from_section(
                    label_text,
                    TextStyle {
                        font_size: 10.,
                        ..default()
                    },
                )
                .with_text_justify(JustifyText::Center)
                .with_style(Style {
                    align_self: AlignSelf::Center,
                    margin: UiRect::right(Val::Px(5.)),
                    flex_grow: 1.,
                    ..default()
                }),
                Label,
                Name::new("chip_label"),
            ));
        });
}
