use bevy::prelude::*;
use bevy::core::window::CreateWindow;

fn main() {
    App::build().add_defaults().setup(setup).run();
}

fn setup(_world: &mut World, resources: &mut Resources) {
    let mut create_window_events = resources.get_mut::<Event<CreateWindow>>().unwrap();
    create_window_events.send(CreateWindow {
        descriptor: WindowDescriptor {
            width: 800,
            height: 600,
            vsync: false,
            title: "another window".to_string(),
        }
    });
}