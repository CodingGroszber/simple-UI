use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::backend::backend;

// Components
#[derive(Component)]
pub struct Highlighted(pub bool);

#[derive(Component)]
pub struct ContentNode;

// Resources
#[derive(Resource)]
pub struct ScrollOffset(pub f32);

// Events
#[derive(Event)]
pub struct HighlightChangeEvent {
    entity: Entity,
    is_highlighted: bool,
}

// UI Setup System
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn the 2D camera
    commands.spawn(Camera2dBundle::default());

    // Read the Markdown file content
    let content = backend::read_file("test/eng.md");
    let content_to_use = if content == "Error reading file" || content.is_empty() {
        "This is default text for testing.".to_string()
    } else {
        content
    };

    // Tokenize the content
    let tokens = backend::tokenize(&content_to_use);
    let font = asset_server.load("fonts/FiraSans-Regular.ttf");

    // Spawn the root UI node
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
            // Spawn the content node with flex layout
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
                    // Iterate through tokens and create UI elements
                    for token in tokens {
                        match token.token_type {
                            backend::TokenType::Word => {
                                content.spawn((
                                    TextBundle {
                                        text: Text::from_section(
                                            token.text.clone(),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 20.0,
                                                color: Color::WHITE,
                                            },
                                        ),
                                        style: Style {
                                            margin: UiRect::all(Val::Px(0.0)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    Interaction::default(),
                                    Highlighted(false),
                                ));
                            }
                            backend::TokenType::Punctuation => {
                                content.spawn(TextBundle {
                                    text: Text::from_section(
                                        token.text.clone(),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    style: Style {
                                        margin: UiRect::all(Val::Px(0.0)),
                                        ..default()
                                    },
                                    ..default()
                                });
                            }
                            backend::TokenType::Space => {
                                content.spawn(TextBundle {
                                    text: Text::from_section(
                                        token.text.clone(), // e.g., "  " for multiple spaces
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    style: Style {
                                        margin: UiRect::all(Val::Px(0.0)),
                                        ..default()
                                    },
                                    ..default()
                                });
                            }
                            backend::TokenType::Tab => {
                                content.spawn(TextBundle {
                                    text: Text::from_section(
                                        "    ".to_string(), // Replace tab with 4 spaces
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 20.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    style: Style {
                                        margin: UiRect::all(Val::Px(0.0)),
                                        ..default()
                                    },
                                    ..default()
                                });
                            }
                            backend::TokenType::Newline => {
                                content.spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(0.0),
                                        ..default()
                                    },
                                    ..default()
                                });
                            }
                            // Skip Markdown-specific tokens (e.g., MarkdownHeader, MarkdownBold)
                            _ => {}
                        }
                    }
                });
        });

    // Initialize scroll offset resource
    commands.insert_resource(ScrollOffset(0.0));
}

// Input handling systems
pub fn handle_scroll_input(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut scroll_offset: ResMut<ScrollOffset>,
) {
    for event in mouse_wheel.read() {
        scroll_offset.0 -= event.y * 20.0; // Scroll speed
    }
}

pub fn handle_clicks(
    mut query: Query<(Entity, &Interaction, &mut Highlighted), Changed<Interaction>>,
    mut events: EventWriter<HighlightChangeEvent>,
) {
    for (entity, interaction, mut highlighted) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            // Toggle highlight state
            highlighted.0 = !highlighted.0;

            // Emit event when highlight state changes
            events.send(HighlightChangeEvent {
                entity,
                is_highlighted: highlighted.0,
            });
        }
    }
}

// UI update systems
pub fn update_text_styles(
    mut events: EventReader<HighlightChangeEvent>,
    mut text_query: Query<&mut Text>,
) {
    for event in events.read() {  // Changed from .iter() to .read()
        // Only process texts that had their highlight state changed
        if let Ok(mut text) = text_query.get_mut(event.entity) {
            if let Some(section) = text.sections.first_mut() {
                section.style.color = if event.is_highlighted {
                    Color::YELLOW
                } else {
                    Color::WHITE
                };
            }
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