use std::sync::Mutex;

use bevy::{
    core::FrameCount,
    core_pipeline::bloom::Bloom,
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot, ScreenshotPlugin},
    window::PrimaryWindow,
};
use bevy_shape_transition::prelude::*;
use rand::{
    rngs::StdRng,
    Rng, SeedableRng,
};

#[derive(Debug, Resource)]
pub struct PartyRng(StdRng);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TransitionPlugin))
        .insert_resource(PartyRng(StdRng::from_entropy()))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

// create a camera
fn setup(
    mut commands: Commands
) {
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

// on update, check if the driver is 0.0 and send a transition event
fn update(
    mut commands: Commands,
    mut events: EventWriter<NewTransition>,
    mut shader_query: Query<&mut TransitionDefinition>,
    mut rng: ResMut<PartyRng>,
    mut time : ResMut<Time>,
    mut last_spawned: Local<f32>,
    mut counter: Local<u32>,
) {
    let duration = 2.5;
    let path = format!("party_screenshot_{:?}.png", counter);
    *counter += 1;
    commands.spawn(Screenshot::primary_window())
      .observe(save_to_disk(path));

    if duration + *last_spawned < time.elapsed_secs() {
        *last_spawned = time.elapsed_secs();

        let our_rng = &mut rng.0;
        events.send(NewTransition {
            angle: our_rng.gen_range(0.0..360.0),
            color: Color::srgba(
                our_rng.gen_range(0.0..1.0),
                our_rng.gen_range(0.0..1.0),
                our_rng.gen_range(0.0..1.0),
                1.0,
            ),
            duration,
            ease: match our_rng.gen_range(0..=22) {
                0 => EaseFunction::Linear,
                1 => EaseFunction::QuadraticIn,
                2 => EaseFunction::QuadraticOut,
                3 => EaseFunction::QuadraticInOut,
                4 => EaseFunction::QuarticIn,
                5 => EaseFunction::CubicIn,
                6 => EaseFunction::CubicOut,
                7 => EaseFunction::CubicInOut,
                8 => EaseFunction::QuarticIn,
                9 => EaseFunction::QuarticOut,
                10 => EaseFunction::QuarticInOut,
                11 => EaseFunction::QuinticIn,
                12 => EaseFunction::QuinticOut,
                13 => EaseFunction::QuinticInOut,
                14 => EaseFunction::CircularIn,
                15 => EaseFunction::CircularOut,
                16 => EaseFunction::CircularInOut,
                17 => EaseFunction::ExponentialIn,
                18 => EaseFunction::ExponentialOut,
                19 => EaseFunction::ExponentialInOut,
                20 => EaseFunction::BounceIn,
                21 => EaseFunction::BounceOut,
                22 => EaseFunction::BounceInOut,
                _ => unreachable!(),
            },
        });
    }
}
