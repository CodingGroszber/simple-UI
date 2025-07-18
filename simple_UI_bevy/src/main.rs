use bevy::prelude::*;

mod backend;
mod frontend;
use frontend::frontend::HighlightChangeEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Register the highlight change event
        .add_event::<HighlightChangeEvent>()
        .add_systems(Startup, frontend::frontend::setup_ui)
        .add_systems(
            Update,
            (
                frontend::frontend::handle_scroll_input,
                frontend::frontend::handle_clicks,
                frontend::frontend::update_text_styles,
                frontend::frontend::update_content_position,
            ),
        )
        .run();
}