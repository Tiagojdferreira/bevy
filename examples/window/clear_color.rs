use bevy::prelude::*;

fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(0.2, 0.2, 0.8)))
        .add_default_plugins()
        .run();
}