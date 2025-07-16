use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::backend::backend;

#[derive(Component)]
pub struct Highlighted(bool);

#[derive(Component)]
pub struct ContentNode;

#[derive(Resource)]
pub struct ScrollOffset(pub f32);

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera for UI rendering - this is critical!
    commands.spawn(Camera2dBundle::default());

    let content = backend::read_file("test/eng.md");
    // Add fallback content in case file doesn't exist or is empty
    let content_to_use = if content == "Error reading file" || content.is_empty() {
        "This is default text for testing.".to_string()
    } else {
        content
    };

    let words = content_to_use.split_whitespace().collect::<Vec<_>>();

    let font = asset_server.load("fonts/FiraSans-Regular.ttf");

    // Rest of your UI setup remains the same...
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                overflow: Overflow::clip_y(),
                ..default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..default()
        })
        .with_children(|parent| {
            // Spawn content node
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            left: Val::Px(0.0),
                            top: Val::Px(0.0),
                            flex_direction: FlexDirection::Row,
                            flex_wrap: FlexWrap::Wrap,
                            ..default()
                        },
                        ..default()
                    },
                    ContentNode,
                ))
                .with_children(|content| {
                    for word in words {
                        content.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    format!("{} ", word), // Add space after each word
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                style: Style {
                                    margin: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            Interaction::default(),
                            Highlighted(false),
                        ));
                    }
                });
        });

    // Initialize scroll offset
    commands.insert_resource(ScrollOffset(0.0));
}

pub fn handle_input(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut scroll_offset: ResMut<ScrollOffset>,
) {
    for event in mouse_wheel.read() {
        scroll_offset.0 -= event.y * 20.0; // Scroll speed
    }
}

pub fn handle_clicks(mut query: Query<(&Interaction, &mut Highlighted), Changed<Interaction>>) {
    for (interaction, mut highlighted) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            highlighted.0 = !highlighted.0;
        }
    }
}

pub fn update_text_styles(mut query: Query<(&mut Text, &Highlighted)>) {
    for (mut text, highlighted) in query.iter_mut() {
        if let Some(section) = text.sections.first_mut() {
            section.style.color = if highlighted.0 {
                Color::YELLOW
            } else {
                Color::WHITE
            };
        }
    }
}

pub fn update_content_position(
    scroll_offset: Res<ScrollOffset>,
    mut query: Query<&mut Style, With<ContentNode>>,
) {
    for mut style in query.iter_mut() {
        style.top = Val::Px(-scroll_offset.0);
    }
}
