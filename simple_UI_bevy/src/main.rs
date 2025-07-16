use bevy::prelude::*;

mod backend;
mod frontend;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, frontend::frontend::setup_ui)
        .add_systems(
            Update,
            (
                frontend::frontend::handle_input,
                frontend::frontend::handle_clicks, // Added to resolve "never used" warning
                frontend::frontend::update_text_styles,
                frontend::frontend::update_content_position,
            ),
        )
        .run();
}
