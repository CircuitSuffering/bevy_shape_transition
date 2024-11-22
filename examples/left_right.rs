use bevy::{core_pipeline::bloom::Bloom, prelude::*};
use bevy_shape_transition::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TransitionPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

// create a camera
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..Default::default()
        },
        Bloom::NATURAL,
    ));
}

// on update, check for arrow key presses and send transition events
fn update(mut events: EventWriter<NewTransition>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        events.send(NewTransition {
            angle: 0.0,
            // pastel brown
            color: Color::srgba(0.8, 0.6, 0.6, 1.0),
            duration: 1.8,
            ease: EaseFunction::QuarticIn,
        });
    }
    if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        events.send(NewTransition {
            angle: 180.0,
            // pastel blue
            color: Color::srgba(0.6, 0.6, 0.8, 1.0),
            duration: 1.8,
            ease: EaseFunction::QuarticIn,
        });
    }
}
