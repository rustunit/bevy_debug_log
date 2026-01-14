use bevy_color::prelude::*;
use bevy_ecs::{prelude::*, relationship::RelatedSpawnerCommands};
use bevy_text::prelude::*;
use bevy_ui::prelude::*;
use bevy_utils::prelude::*;

#[derive(Component)]
pub(crate) struct CheckboxIconMarker;

pub(crate) fn spawn_checkbox<B: Bundle + Clone>(
    children: &mut RelatedSpawnerCommands<ChildOf>,
    bundle: B,
    name: &str,
    checked: bool,
    text: String,
) {
    children
        .spawn((
            Node {
                padding: UiRect::all(Val::Px(5.)),
                align_items: AlignItems::End,
                ..default()
            },
            Name::new(name.to_string()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        border: UiRect::all(Val::Px(1.)),
                        width: Val::Px(20.),
                        height: Val::Px(20.),
                        justify_content: JustifyContent::Center,
                        border_radius: BorderRadius::all(Val::Px(5.)),
                        ..default()
                    },
                    BorderColor::all(Color::WHITE),
                    Name::new("check_box_button"),
                    bundle,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Node {
                            align_self: AlignSelf::Center,
                            width: Val::Px(10.),
                            height: Val::Px(10.),
                            display: if checked {
                                Display::Flex
                            } else {
                                Display::None
                            },
                            border_radius: BorderRadius::all(Val::Px(3.)),
                            ..default()
                        },
                        BackgroundColor(Color::WHITE),
                        CheckboxIconMarker,
                        Name::new("check_box_icon"),
                    ));
                });
        });

    children
        .spawn((
            Node {
                align_content: AlignContent::Stretch,
                align_self: AlignSelf::Center,
                ..default()
            },
            Name::new("check_box_label"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: 10.,
                    ..default()
                },
                Label,
            ));
        });
}

#[derive(Component)]
struct ChipMarker;

#[derive(Component)]
pub(crate) struct ChipLeadingTextMarker;

pub(crate) fn spawn_chip<B: Bundle + Clone>(
    children: &mut RelatedSpawnerCommands<ChildOf>,
    bundle: B,
    color: Color,
    leading_text: String,
    label_text: String,
    active: bool,
    name: &str,
) {
    let bg = BackgroundColor(if active {
        color.with_alpha(0.25)
    } else {
        color.with_alpha(0.)
    });

    children
        .spawn((
            Button,
            Node {
                border: UiRect::all(Val::Px(1.)),
                justify_content: JustifyContent::Center,
                align_self: AlignSelf::Center,
                margin: UiRect::all(Val::Px(1.)),
                border_radius: BorderRadius::all(Val::Px(20.)),
                ..default()
            },
            BorderColor::all(if active { color } else { Color::WHITE }),
            bg,
            ChipMarker,
            bundle.clone(),
            Name::new(name.to_string()),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    // Circle with number of messages
                    Node {
                        align_self: AlignSelf::Center,
                        margin: UiRect::all(Val::Px(5.)),
                        display: Display::Flex,
                        min_height: Val::Px(15.),
                        min_width: Val::Px(15.),
                        border_radius: BorderRadius::all(Val::Px(10.)),
                        ..default()
                    },
                    BackgroundColor(color),
                    Name::new("chip_leading"),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new(leading_text),
                        TextLayout::new_with_justify(Justify::Center),
                        Node {
                            align_self: AlignSelf::Center,
                            flex_grow: 1.,
                            margin: UiRect::all(Val::Px(2.)),
                            ..default()
                        },
                        TextFont {
                            font_size: 10.,
                            ..default()
                        },
                        TextColor(Color::BLACK),
                        BackgroundColor(color),
                        Label,
                        bundle,
                        ChipLeadingTextMarker,
                    ));
                });

            parent.spawn((
                Text::new(label_text),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    align_self: AlignSelf::Center,
                    margin: UiRect::right(Val::Px(5.)),
                    flex_grow: 1.,
                    ..default()
                },
                TextFont {
                    font_size: 10.,
                    ..default()
                },
                Label,
                Name::new("chip_label"),
            ));
        });
}
