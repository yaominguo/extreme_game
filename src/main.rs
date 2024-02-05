use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true, // fill the browser window
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .run()
}
