use bevy_color::prelude::*;
use bevy_ecs::{
    prelude::*,
    spawn::{Spawn, SpawnableList},
};
use bevy_math::prelude::*;
use bevy_text::prelude::*;
use bevy_ui::prelude::*;
use bevy_utils::prelude::*;

/// Largest valid [`ScrollPosition`] for a scroll container, in logical px.
///
/// `ComputedNode` sizes are physical px, so the range has to be converted; going past it
/// leaves an invisible offset that has to be scrolled back before anything moves again.
pub(crate) fn max_scroll(container: &ComputedNode, content: &ComputedNode) -> Vec2 {
    (content.size() - container.size()).max(Vec2::ZERO) * container.inverse_scale_factor
}

fn small_text() -> TextFont {
    TextFont {
        font_size: FontSize::Px(10.),
        ..default()
    }
}

#[derive(Component)]
pub(crate) struct CheckboxIconMarker;

/// A checkbox and its label, as two sibling entities.
pub(crate) fn checkbox<B: Bundle + Clone>(
    bundle: B,
    name: &str,
    checked: bool,
    text: String,
) -> impl SpawnableList<ChildOf> {
    (
        Spawn((
            Node {
                padding: UiRect::all(Val::Px(5.)),
                align_items: AlignItems::End,
                ..default()
            },
            Name::new(name.to_string()),
            children![(
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
                children![(
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
                )],
            )],
        )),
        Spawn((
            Node {
                align_content: AlignContent::Stretch,
                align_self: AlignSelf::Center,
                ..default()
            },
            Name::new("check_box_label"),
            children![(Text::new(text), small_text(), Label)],
        )),
    )
}

#[derive(Component)]
struct ChipMarker;

#[derive(Component)]
pub(crate) struct ChipLeadingTextMarker;

/// A level filter chip: a message count in a coloured circle, followed by a label.
pub(crate) fn chip<B: Bundle + Clone>(
    bundle: B,
    color: Color,
    leading_text: String,
    label_text: String,
    active: bool,
    name: &str,
) -> impl Bundle {
    (
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
        BackgroundColor(color.with_alpha(if active { 0.25 } else { 0. })),
        ChipMarker,
        bundle.clone(),
        Name::new(name.to_string()),
        children![
            (
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
                children![(
                    Text::new(leading_text),
                    TextLayout::justify(Justify::Center),
                    Node {
                        align_self: AlignSelf::Center,
                        flex_grow: 1.,
                        margin: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    small_text(),
                    TextColor(Color::BLACK),
                    BackgroundColor(color),
                    Label,
                    bundle,
                    ChipLeadingTextMarker,
                )],
            ),
            (
                Text::new(label_text),
                TextLayout::justify(Justify::Center),
                Node {
                    align_self: AlignSelf::Center,
                    margin: UiRect::right(Val::Px(5.)),
                    flex_grow: 1.,
                    ..default()
                },
                small_text(),
                Label,
                Name::new("chip_label"),
            ),
        ],
    )
}
